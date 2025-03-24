use core::error;
use std::fmt::Error;
use std::iter::Cycle;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;
use actix::prelude::*;
use actix::clock::sleep;
use tokio::time;

use crate::tool::material_flow::BufferInfo;
use crate::tool::material_flow::BufferStatus;
use crate::tool::material_flow::MaterialFlowAction;
use crate::tool::material_flow::MaterialFlowError;
use crate::tool::material_flow::MaterialFlowResult;
use crate::tool::material_flow::MaterialFlowStatus;
use crate::tool::material_flow::PushDownNextCmd;

use super::ProcessResult;
use super::ProcessError;
use super::Reason;
use super::{StartOperationCmd, AbortOperationCmd };
use super::super::{Part, ProcessFinishedSgn};

#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    pub time_stamp: Instant,
    pub is_running: bool,
    pub part_id: Option<String>,
}

#[derive(Debug)]
pub struct Process {
    id: String,
    part: Option<Part>,
    cycle_time: Duration,
    operation: Option<SpawnHandle>,
    parent: Recipient<ProcessFinishedSgn>,
    downstream_blocked: bool,
    is_busy: bool
}



impl Actor for Process {
    type Context = Context<Self>;
}   

// impl Handler<StockOperation> for Process {
//     type Result = ProcessResult;

//     fn handle(&mut self, msg: StockOperation, ctx: &mut Self::Context) -> Self::Result {
//         if self.part.is_some() {
//             return ProcessResult::Error(ProcessError::Occupied);
//         }

//         self.part = Some(msg.part);

//         ProcessResult::Ok
//     }
// }

impl Handler<StartOperationCmd> for Process {
    type Result = ProcessResult;

    fn handle(&mut self, _msg: StartOperationCmd, ctx: &mut Self::Context) -> Self::Result {
        
        if self.downstream_blocked {
            return ProcessResult(Err(ProcessError::DownstreamBlocked));
        }

        if self.is_busy {
            return ProcessResult(Err(ProcessError::Busy));
        }
        
        match &self.part {
            Some(part) => {
                let (part, result) = self.decide_outcome(part.clone());
                
                println!("process starting for part {}", part.serial_no);

                let cycle_time = self.cycle_time;
                let parent = self.parent.clone();
                let future = Box::pin(async move {
                    sleep(cycle_time).await;

                    match parent.try_send(ProcessFinishedSgn) {
                        Err(error) => {
                            // queue signal anyway in case parent recovers
                            parent.do_send(ProcessFinishedSgn);    
                            println!("Process: ERROR - cannot indicate process finished. Likely software error.");
                        },
                        _ => ()
                    }
                });

                let handle = ctx.spawn(future.into_actor(self));
                self.operation = Some(handle);

                return ProcessResult(Ok(()));
            },
            None => {
                return ProcessResult(
                    Err(ProcessError::MaterialMissing)
                )
            }        
        }
    }
}


impl Handler<PushDownNextCmd> for Process {
    type Result = MaterialFlowResult;

    ///
    /// Take the part out of the operation and load it to downstream recipient
    /// 
    /// Return error MaterialFlowError::Busy when either is ongoing:
    /// - operation  
    /// - message handling with downstream recipient
    /// 
    fn handle(&mut self, msg: PushDownNextCmd, _ctx: &mut Self::Context) -> Self::Result {

        if self.is_busy {
            return MaterialFlowResult(Err(MaterialFlowError::Busy));
        }



        match self.push(msg.part) {
            Ok(()) => {
                self.is_busy = true;
                return MaterialFlowResult(Ok(MaterialFlowStatus {
                    buffer: self.buffer_status(),
                    action: MaterialFlowAction::PushedDownstream
                }));
            },
            Err(error) => {
                return MaterialFlowResult(Err(error))
            }
        }
    }
}



impl Handler<MaterialFlowResult> for Process {
    type Result = ();

    fn handle(&mut self, msg: MaterialFlowResult, _ctx: Self::Context) -> Self::Result {
        
    }
}

struct Outcome<'a> { 
    pub part: &'a Part,
    pub result: ProcessResult
}

impl Process {
    pub fn new(id: String, cycle_time: Duration, parent: Recipient<ProcessFinishedSgn>) -> Self {
        Self {
            id,
            part: None,
            operation: None,
            cycle_time,
            is_busy: false,
            downstream_blocked: false,
            parent
        }
    }

    fn decide_outcome(&self, mut part: Part) -> (Part, ProcessResult) {
        // TODO: future statistical magic with recipies and part history
        part.add_step(&self.id);
        part.failed = false;
        
        (part, ProcessResult(Ok(())))
    }
}

impl BufferInfo for Process {
    fn buffer_status(&self) -> BufferStatus {
        BufferStatus { 
            is_full: self.part.is_some(),
            is_empty: self.part.is_none()
        }
    }
}