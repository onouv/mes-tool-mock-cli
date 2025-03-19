use actix::prelude::*;
use actix::clock::sleep;
use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::time::Duration;

use super::ToolState;

use super::Part;
use super::signal::{Abort, LoadPart, ProcessFinished, Start, Stop, UnloadPart, UnloadPartResponse};
use super::ToolSnapshot;

static INPORT_CAPACITY: usize = 3;
static OUTPORT_CAPACITY: usize = 3;
static PROCESS_TIME: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct Tool {
    id: String,
    inport: VecDeque<Part>,
    process: Option<Part>,
    process_task: Option<SpawnHandle>,
    outport: VecDeque<Part>,
    state: ToolState
}

impl Actor for Tool {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Tool {} created.", self.id);
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Tool {} destroyed.", self.id);
    }
}

impl Handler<Start> for Tool {
    type Result = ToolSnapshot;
    fn handle(&mut self, _msg: Start, ctx: &mut Context<Self>) -> Self::Result {
        
        println!("Tool {}: handling Start signal...", self.id);

        match self.state {
            ToolState::STOPPED => {
                if self.inport_empty() {
                    // (1)
                    self.state = ToolState::IDLE;
                } else if self.outport_free() {
                    // (2)
                    self.shift_part_to_process();
                    let cycle_time = PROCESS_TIME;
                    let ctx_address = ctx.address();

                    let future = Box::pin(async move {
                        sleep(cycle_time).await;
                        ctx_address.do_send(ProcessFinished);
                        ()  // Explicitly return unit type
                    });

                    println!("Tool {}: process starting...", self.id);
                    self.process_task = Some(ctx.spawn(future.into_actor(self)));   

                    self.state = ToolState::RUNNING;
                }
            },
            _ => ()
        }
        
        self.snapshot()        
    }
}

impl Handler<Stop> for Tool {
    type Result = ToolSnapshot;

    fn handle(&mut self, _msg: Stop, _ctx: &mut Context<Self>) -> Self::Result {

        println!("Tool {}: handling Stop signal...", self.id);
        
        match self.state {
            ToolState::IDLE => {
                // (3)
                self.state = ToolState::STOPPED
            },
            ToolState::RUNNING => {
                
                // (11)
                self.state = ToolState::STOPPING;
            },
            _ => ()
        }
        
        self.snapshot()        
    }
}

impl Handler<ProcessFinished> for Tool {
    type Result = ();

    fn handle(&mut self, _msg: ProcessFinished, _ctx: &mut Context<Self>) -> Self::Result {

        println!("Tool {}: handling ProcessFinished signal...", self.id);

        self.shift_part_to_outport(false);

        match self.state {
            
            ToolState::RUNNING => {
                // (4)
                if self.inport_empty() {
                    // (10)
                    self.state = ToolState::IDLE;
                } else {
                    // (9)
                    self.shift_part_to_process();
                    // self.process.start(_ctx);
                }
            },
            ToolState::STOPPING => {
                // (14)
                self.state = ToolState::STOPPED;
            },
            _ => ()
        }
    }

}

impl Handler<LoadPart> for Tool {
    type Result = ToolSnapshot;

    fn handle(&mut self, msg: LoadPart, ctx: &mut Context<Self>) -> Self::Result {

        println!("Tool {}: handling LoadPart {:?} signal...", self.id, msg);

        let inport_free = self.inport_free();
        let process_empty = self.process_empty();
        let outport_free = self.outport_free();

        match self.state {
            ToolState::IDLE => {
                if inport_free {
                    self.inport.push_front(msg.part);
                
                    if inport_free && process_empty && outport_free {
                        self.shift_part_to_process();
                        
                        let cycle_time = PROCESS_TIME;
                    let ctx_address = ctx.address();

                    let future = Box::pin(async move {
                        sleep(cycle_time).await;
                        ctx_address.do_send(ProcessFinished);
                        ()  // Explicitly return unit type
                    });

                        self.process_task = Some(ctx.spawn(future.into_actor(self)));
                        println!("Tool {}: process starting...", self.id);

                        self.state = ToolState::RUNNING;
                    }

                    // (6) implicit
                }
            },
            ToolState::RUNNING => {
                if inport_free {

                    // (8)
                    self.inport.push_front(msg.part);
                }
            },
            _ => ()
        }

        self.snapshot()
    }
}

impl Handler<UnloadPart> for Tool {
    type Result =  MessageResult<UnloadPart>;

    fn handle(&mut self, _msg: UnloadPart, ctx: &mut Context<Self>) -> Self::Result {
    
        println!("Tool {} received UnloadPart signal.", self.id);

        let part = self.outport.pop_back();
        
        match self.state {
            ToolState::IDLE => {
                // (5)
                if self.outport_free() {
                    let cycle_time = PROCESS_TIME;
                    let ctx_address = ctx.address();

                    let future = Box::pin(async move {
                        sleep(cycle_time).await;
                        ctx_address.do_send(ProcessFinished);
                        ()  // Explicitly return unit type
                    });

                    self.process_task = Some(ctx.spawn(future.into_actor(self)));
                    println!("Tool {} process starting...", self.id);
                    
                    self.state = ToolState::RUNNING;
                }
            },
            ToolState::RUNNING => {
                // (7)
                if self.outport.is_empty() {
                    self.state = ToolState::FAULTED;
                }
            },
            ToolState::STOPPING => {
                // (13)
                self.state = ToolState::STOPPED;
            },
            _ => ()
        }

        MessageResult(UnloadPartResponse {
            part: part,
            snapshot: self.snapshot()
        })

    }
}

impl Handler<Abort> for Tool {
    type Result = ToolSnapshot;

    fn handle(&mut self, _msg: Abort, ctx: &mut Context<Self>) -> Self::Result {
        match self.state {
            ToolState::RUNNING => {
                // (12)
                if let Some(task) = self.process_task.take() {
                    ctx.cancel_future(task);
                    self.shift_part_to_outport(true);
                    println!("Tool {}: Error, process aborted!", self.id);
                }
                self.state = ToolState::STOPPED;
            },
            _ => ()
        }

        self.snapshot()
    }
}

impl Tool {
    pub fn new(id: &str) -> Self {
        Tool {
            id: id.to_string(),
            inport: VecDeque::new(),
            process: None,
            process_task: None,
            outport: VecDeque::new(),
            state: ToolState::STOPPED
        }
    }

    fn inport_free(&self) -> bool {
        self.inport.len() < INPORT_CAPACITY
    }

    fn inport_empty(&self) -> bool {
        self.inport.is_empty()
    }

    fn outport_free(&self) -> bool {
        self.outport.len() < INPORT_CAPACITY
    }

    fn process_empty(&self) -> bool {
        self.process.is_none()
    }
    
    fn shift_part_to_process(&mut self) {
        if let Some(mut part) = self.inport.pop_back() {
            println!("Tool {}: moving part {} from inport to process.", self.id, part.serial_no);
            
            part.add_step("process");
            part.failed = false;
            self.process = Some(part);
        }
    }
    
    fn shift_part_to_outport(&mut self, part_has_error: bool) {
        if let Some(mut part) = self.process.take() {  
            println!("Tool {}: moving part {} from process to outport.", self.id, part.serial_no);

            part.failed = part_has_error;      
            self.outport.push_front(part);
        }
    }                       

    fn snapshot(&self) -> ToolSnapshot {
        ToolSnapshot {
            id: self.id.clone(),
            parts_inport: self.inport.len(),
            process_occupied: !self.process_empty(),
            process_completion: None,
            parts_outport: self.outport.len(),
            state: self.state.clone()
        }
    }

}

impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tool {} in state {}", self.id, self.state)
    }
}
