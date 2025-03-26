use core::error;
use std::fmt::Error;
use std::iter::Cycle;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;
use actix::prelude::*;
use actix::clock::sleep;
use tokio::time;

use super::{StartOperationCmd, AbortOperationCmd, ProcessResult, ProcessError, Reason };
use super::super::{Part, ProcessFinishedSgn, material_flow::*};

#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    pub time_stamp: Instant,
    pub is_running: bool,
    pub part_id: Option<String>,
}

#[derive(Clone, Copy, Debug)]
enum State {
    Created,
    Initialized,
    OperationRunning,
    OperationCompleted,
    HandlingMateialFlow
}

enum Command {
    Initialize(InitializeChainedActorCmd),
    
}

#[derive(Debug)]
pub struct Process {
    id: String,
    state: State,
    part: Option<Part>,
    cycle_time: Duration,
    operation: Option<SpawnHandle>,
    handlers: Option<MaterialFlowChainHandlers>,
    downstream_blocked: bool,
    is_operating: bool,
    material_flow_action_pending: bool
}

impl Actor for Process {
    type Context = Context<Self>;
}   

impl Handler<StartOperationCmd> for Process {
    type Result = ProcessResult;

    fn handle(&mut self, _msg: StartOperationCmd, ctx: &mut Self::Context) -> Self::Result {
        
        if self.downstream_blocked {
            return ProcessResult(Err(ProcessError::DownstreamBlocked));
        }

        if self.is_operating {
            return ProcessResult(Err(ProcessError::Busy));
        }
        
        match &self.part {
            Some(part) => {
                let (part, result) = self.simulate_outcome(part.clone());
                
                println!("process starting for part {}", part.serial_no);

                let cycle_time = self.cycle_time;
                let parent = self.handlers.parent.clone();

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

impl Handler<PushDownstreamCmd> for Process {
    type Result = MaterialFlowResult;

    /// Swap part1 in the operation with the incoming part2 and push part1 to downstream recipient.
    /// 
    /// Return error MaterialFlowError::Busy when either is ongoing:
    /// - operation  
    /// - message handling with downstream recipient
    /// 
    /// Return Ok(PushedDownstream) part accepted and able to accept further parts
    /// 
    /// Return Ok(Full) if part accepted but unable to accept further parts
    /// 
    /// Return Err(...) accordingly
    /// 
    fn handle(&mut self, msg: PushDownstreamCmd, _ctx: &mut Self::Context) -> Self::Result {

        if self.material_flow_action_pending || self.is_operating {
            return MaterialFlowResult(Err(MaterialFlowError::Busy));
        }

        if self.downstream_blocked {
            return MaterialFlowResult(Err(MaterialFlowError::BlockedByDownstream))
        }

        match self.part.take() {
            Some(outgoing_part) => {
                self.material_flow_action_pending = true;
                self.part = Some(msg.part);
                self.handlers.downstream.do_send(PushDownstreamCmd { part: outgoing_part });
                return MaterialFlowResult(Ok(MaterialFlowStatus {
                    buffer: self.buffer_status(),
                    action: MaterialFlowAction::PushedDownstream
                }));
            },
            None => {
                self.part = Some(msg.part);
                return MaterialFlowResult(Err(MaterialFlowError::Empty));
            }
        }
    }
}

impl Handler<MaterialFlowResult> for Process {
    type Result = ();

    fn handle(&mut self, msg: MaterialFlowResult, _ctx: Self::Context) -> Self::Result {
        if self.material_flow_action_pending {
            self.material_flow_action_pending = false;
            match msg.0 {
                Ok(status) => {
                    self.downstream_blocked = status.buffer.is_full;
                    self.upstream.do_send(MaterialFlowResult(Ok(MaterialFlowStatus { 
                        buffer: self.buffer_status(), 
                        action: status.action 
                    })));
                },
                Err(error) => {
                    self.upstream.do_send(MaterialFlowResult(Err(error)));
                }
            }
        }
    }
}

impl Process {
    pub fn new(id: String, cycle_time: Duration) -> Self {
        Self {
            id,
            state: State::Created,
            part: None,
            operation: None,
            cycle_time,
            is_operating: false,
            material_flow_action_pending: false,
            downstream_blocked: false,
            handlers: None
        }
    }

    pub fn next(&mut self, command: Command) {
        match (self.state, command) {
            (State::Created, Command::Initialize(cmd)) => {
                self.handlers = Some(cmd.handlers);
                self.state = State::Initialized;
            },
            _ => {}
        }
    }

    fn simulate_outcome(&self, mut part: Part) -> (Part, ProcessResult) {
        // TODO: future statistical magic with recipies and part history
        part.add_step(&self.id);
        part.failed = false;
        
        (part, ProcessResult(Ok(())))
    }
}

impl Handler<InitializeChainedActorCmd> for Process {
    type Result = ();

    fn handle(&mut self, msg: InitializeChainedActorCmd, ctx: &mut Self::Context) -> Self::Result {
        self.handle(msg, ctx)
    }
}

// impl MaterialFlowChainedActor for Process {
//     fn setHandlers(&mut self, handlers: MaterialFlowChainHandlers) {
//         fn handle(&mut self, msg: InitializeChainedActorCmd, ctx: &mut Self::Context) -> Self::Result {
//             self.handle(msg, ctx)
//         }
//     }
// }

impl BufferInfo for Process {
    fn buffer_status(&self) -> BufferStatus {
        BufferStatus { 
            is_full: self.part.is_some(),
            is_empty: self.part.is_none()
        }
    }
}