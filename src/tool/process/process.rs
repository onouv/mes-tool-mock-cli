use core::error;
use std::fmt::Error;
use std::iter::Cycle;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;
use actix::prelude::*;
use actix::clock::sleep;
//use tokio::task::JoinHandle;
use tokio::time;

use super::ProcessResult;
use super::ProcessError;
use super::super::commands::{StartOperation, StockOperation };
use super::super::{Part, Tool, ProcessFinished};

#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    pub time_stamp: Instant,
    pub is_running: bool,
    pub part_id: Option<String>,
}

pub struct Process {
    id: String,
    snapshot: ProcessSnapshot,
    actor: Option<Addr<ProcessActor>>
}

impl Process {
    pub fn new(id: &str, cycle_time: Duration) -> Self {
        Self { 
            id: id.to_string(),
            snapshot: ProcessSnapshot { 
                time_stamp: Instant::now(), 
                is_running: false, 
                part_id: None 
            },
            actor: None
        }
    }

    pub fn activate(&mut self, parent: Recipient<ProcessFinished>, cycle_time: Duration) {
        self.actor = Some(ProcessActor::new(self.id.clone(), cycle_time, parent).start());
    }

    pub fn load(&mut self, part: Part) -> ProcessResult {
        match self.actor {
            Some(actor) => {
                match actor.try_send(StockOperation { part }) {
                    Ok(()) => {
                        self.snapshot.time_stamp = Instant::now();
                        self.snapshot.part_id = Some(part.serial_no);
                        ProcessResult::Ok
                    },
                    Ok(ProcessResult::Error(error)) => {

                    }
                    Err(error) => {
                        println!("Tool {} ERROR: Software error - failed to stock operation of process ({:?})", self.id, error);
                        ProcessResult::Error(())
                    }
                }
            },
            None => {

            }
            
            
        }
    }

    pub fn start(& self) {
    
        match self.actor.unwrap() {
            Some(actor) => {
                match actor.try_send(StartOperation) {
                    Ok(()) => {
                        self.snapshot
                        self.state = ToolState::RUNNING;
                    },
                    Err(error) => {
                        println!("Tool {}: ERROR - failed to start process ({:?})", self.id, error);
                        self.state = ToolState::FAULTED
                    }
                }
            }
        }
    }

    pub fn abort() {
        
    }

    pub fn snapshot(&mut self) -> ProcessSnapshot {
        self.snapshot.time_stamp = std::time::Instant::now();
        self.snapshot.clone()
    }
}

#[derive(Debug)]
struct ProcessActor {
    id: String,
    part: Option<Part>,
    cycle_time: Duration,
    operation: Option<SpawnHandle>,
    parent: Recipient<ProcessFinished>
}



impl Actor for ProcessActor {
    type Context = Context<Self>;
}   

impl Handler<StockOperation> for ProcessActor {
    type Result = ProcessResult;

    fn handle(&mut self, msg: StockOperation, ctx: &mut Self::Context) -> Self::Result {
        if self.part.is_some() {
            return ProcessResult::Error(ProcessError::Occupied);
        }

        self.part = Some(msg.part);

        ProcessResult::Ok
    }
}

impl Handler<StartOperation> for ProcessActor {
    type Result = ProcessResult;

    fn handle(&mut self, _msg: StartOperation, ctx: &mut Self::Context) -> Self::Result {
        if self.part.is_none() {
            return ProcessResult::Error(ProcessError::Empty);
        }

        if let Some(part) = self.part.as_mut() {
            part.add_step(&self.id);
            part.failed = false;

            println!("process starting for part {}", part.serial_no);

            let cycle_time = self.cycle_time;
            let parent = self.parent.clone();
            let future = Box::pin(async move {
                sleep(cycle_time).await;

                match parent.try_send(ProcessFinished) {
                    Err(error) => {
                        // queue signal anyway in case parent recovers
                        parent.do_send(ProcessFinished);
                        println!("Process: ERROR - cannot indicate process finished. Likely software error.");
                    },
                    _ => ()
                }
            });

            let handle = ctx.spawn(future.into_actor(self));
            self.operation = Some(handle);

            return ProcessResult::Ok;
        } 

        ProcessResult::Error(ProcessError::Empty)
    }
}

impl ProcessActor {
    pub fn new(id: String, cycle_time: Duration, parent: Recipient<ProcessFinished>) -> Self {
        Self {
            id,
            part: None,
            operation: None,
            cycle_time,
            parent
        }
    }

}