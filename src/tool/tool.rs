use actix::prelude::*;
use actix::clock::sleep;
use std::collections::VecDeque;
use std::fmt::{self, format, Display};
use std::time::{Duration, Instant};

use super::inport::Inport;
use super::material_flow::{
    InlineActorCfg, 
    SourceActorCfg,
    MaterialFlowResult, 
    PushDownstreamCmd, 
    SinkActorCfg, 
    StartNextCycleCmd
};
use super::outport::Outport;
use super::process::ProcessResult;
use super::StartOperationCmd;
use super::Process;
use super::ToolState;
use super::Part;
use super::messages::*;
use super::ToolSnapshot;

static INPORT_CAPACITY: usize = 3;
static OUTPORT_CAPACITY: usize = 3;
static PROCESS_OPERATION_TIME: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct Tool {
    id: String,
    inport: Option<Inport>,
    // process: Option<Process>,
    // outport: Outport,
    state: ToolState,
    load_pending: bool,
    unload_pending: bool
}

impl Actor for Tool {
    type Context = Context<Self>;
}
/* 
impl Handler<StartCmd> for Tool {
    type Result = ToolSnapshot;
    fn handle(&mut self, _msg: StartCmd, ctx: &mut Context<Self>) -> Self::Result {
        
        println!("Tool {}: handling Start signal...", self.id);



        match self.state {
            ToolState::STOPPED => {
                if self.inport_empty() {
                    // TX (1)
                    self.state = ToolState::IDLE;
                } else if self.outport_free() {
                    
                    let part = self.inport.pop_back();
                    
                    match part {
                        Some(part) => {
                            self.process.load(part);
                        },
                        None => {
                            println!("Tool {} ERROR: Software error - inport did not actually contain any part when it was indicated it does.", self.id);
                            self.state = ToolState::FAULTED;
                        }
                    }                    
                }
            },
            _ => ()
        }
        
        self.snapshot()        
    }
}

impl Handler<StopCmd> for Tool {
    type Result = ToolSnapshot;

    fn handle(&mut self, _msg: StopCmd, _ctx: &mut Context<Self>) -> Self::Result {

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

impl Handler<ProcessFinishedSgn> for Tool {
    type Result = ();

    fn handle(&mut self, _msg: ProcessFinishedSgn, _ctx: &mut Context<Self>) -> Self::Result {

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
 */
impl Handler<LoadPartCmd> for Tool {
    type Result = LoadResult;

    fn handle(&mut self, msg: LoadPartCmd, ctx: &mut Context<Self>) -> Self::Result {

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
                        self.process_task = self.process.start(ctx);
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

impl Handler<LoadRes> for Tool {
    type Result = ();

    fn handle(&mut self, msg: LoadRes, ctx: &mut Self::Context) -> Self::Result {
        
    }

}
/* 
impl Handler<UnloadPartCmd> for Tool {
    type Result =  MessageResult<UnloadPartCmd>;

    fn handle(&mut self, _msg: UnloadPartCmd, ctx: &mut Context<Self>) -> Self::Result {
    
        println!("Tool {} received UnloadPart signal.", self.id);

        let part = self.outport.pop_back();
        
        match self.state {
            ToolState::IDLE => {
                // (5)
                if self.outport_free() {
                    self.shift_part_to_process();
                    self.process_task = self.process.start(ctx);
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

impl Handler<AbortCmd> for Tool {
    type Result = ToolSnapshot;

    fn handle(&mut self, _msg: AbortCmd, ctx: &mut Context<Self>) -> Self::Result {
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

impl Handler<MaterialFlowResult> for Tool {
    type Result = ();
    fn handle(&mut self, msg: MaterialFlowResult, ctx: &mut Self::Context) -> Self::Result {
        
    }
}

impl Handler<ProcessResult> for Tool {
    type Result = ();
    fn handle(&mut self, msg: ProcessResult, ctx: &mut Self::Context) -> Self::Result {
        
    }
}
 */
impl Tool {
    pub fn new(id: &str) -> Self {
        Tool {
            id: id.to_string(),
            inport: VecDeque::new(),
            // process: Process::new(id.push_str("-").push_str(id)),
            // outport: VecDeque::new(),
            state: ToolState::STOPPED
        }
    }    

    /* fn snapshot(&self) -> ToolSnapshot {
        ToolSnapshot {
            id: self.id.clone(),
            parts_inport: self.inport.len(),
            process_occupied: !self.process_empty(),
            process_completion: None,
            parts_outport: self.outport.len(),
            state: self.state.clone()
        }
    } */
}

impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tool {} in state {}", self.id, self.state)
    }
}

pub async fn create_tool(id: &str) -> Addr<Tool> {

    //
    // Create and start all actors in raw state
    //
    
    let tool = Tool::new(id).start();   

    let inport_id = format!("{}.01", id);
    let mut inport = Inport::new(inport_id, 4).start();
    
    let process_id = format!("{}.02", id);
    let mut process = Process::new(process_id, PROCESS_OPERATION_TIME).start();
    
    let outport_id = format!("{}.03", id);
    let outport = Outport::new(outport_id).start(); 

    let inport_handlers = SourceActorCfg {
        user_load: tool.clone().recipient()
        // upstream: tool.clone().recipient::<MaterialFlowResult>(),
        // downstream: process.clone().recipient::<PushDownstreamCmd>()
    };
    inport.send(inport_handlers).await;

    //
    // interconnect all message handlers
    //
    process.send(InlineActorCfg {         
        downstream: outport.recipient::<PushDownstreamCmd>(),
        upstream: inport.recipient::<MaterialFlowResult>(),
        parent: tool.clone().recipient::<ProcessFinishedSgn>(),
        parent_result_handler: tool.recipient::<ProcessResult>().clone() }).await;
    
    let outport_handlers = SinkActorCfg {
        upstream: process.recipient::<MaterialFlowResult>().clone(),
        loadport_result: tool.clone().recipient::<MaterialFlowResult>()
    };
    outport.send(outport_handlers).await;

    let tool_recipients = ToolActorCfg {
        downstream: inport.recipient(),
        load: inport.recipient(),
        unload: outport.recipient(),
        start_op: process.recipient(),
        abort_op: process.recipient()
    }
    
    tool.clone()
}