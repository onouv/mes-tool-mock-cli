use std::collections::VecDeque;
use actix::{fut::future::result, prelude::*};

use crate::tool::{
    Part,
    material_flow::*, 
    load_port::*,
    //process::{StartOperationCmd}
};

#[derive(Copy, Clone, Debug)]
enum InportState {
    UnInitialized,
    Ready,
    BusyLoadPort,
    BusyMaterialFlow
}

enum InportEvent {
    Initialize(SourceActorCfg),
    StartNextCycle(StartNextCycleCmd),    
    LoadCommand(LoadCmd),    
    UnloadCommand(UnloadCmd),    
}

enum TransitionRes {
    Initialize,
    Unload(UnloadRes),
    Load(LoadRes),
    MaterialFlow(MaterialFlowResult),
    NoTransition
}


#[derive(Debug)]
pub struct Inport {
    id: String,
    state: InportState,
    buffer: Buffer,
    user_load_client: Option<Recipient<LoadRes>>,
    user_unload_client: Option<Recipient<UnloadRes>>,
    downstream: Option<Recipient<PushDownstreamCmd>>,
    upstream: Option<Recipient<MaterialFlowResult>>,
}

impl Actor for Inport {
    type Context = Context<Self>;
}

impl Inport {
    
    pub fn new(id: String, capacity: usize) -> Self {

        Self {
            id,
            state: InportState::UnInitialized,
            buffer: Buffer::new(capacity),
            user_load_client: None,
            user_unload_client: None,
            downstream: None,
            upstream: None,
        }
    }

    fn next_state(&mut self, event: InportEvent) -> TransitionRes {
        let mut txRes: TransitionRes;
        
        match (self.state, event) {
            (InportState::UnInitialized, InportEvent::Initialize(cfg)) => {
                self.user_load_client = Some(cfg.user_load);
                self.user_unload_client = Some(cfg.user_unload);
                self.downstream = Some(cfg.downstream);
                self.upstream = Some(cfg.upstream);
                self.state = InportState::Ready;

                TransitionRes::Initialize
            },
            (InportState::Ready, InportEvent::StartNextCycle(cmd)) => {
                match self.buffer_status() {
                    BufferStatus::Empty => {
                        // no state change...
                        
                        return Some(MaterialFlowResult(
                                Ok(self.buffer_status())));
                    },
                    _ => {
                        self.state = InportState::BusyMaterialFlow;

                        return Some(self.push_part_downstream());
                    }
                }
            },
            (InportState::Ready, InportEvent::LoadCommand(cmd)) => {
                TransitionRes::Load(self.buffer.push(cmd.part))
            },
            (InportState::Ready, InportEvent::UnloadCommand(cmd)) => {
                TransitionRes::Unload(self.buffer.pop())
            }
            _ => { 
                TransitionRes::NoTransition
            }
        }

    }

    fn push_part_downstream(&self) -> MaterialFlowResult {
        match self.pop() {
            Ok(part) => {
                match self.downstream.try_send(PushDownstreamCmd { part: part.clone() }) {
                    Ok(()) => {                        
                        self.is_handling = false;

                        MaterialFlowResult(Ok(self.buffer_status()))
                    },
                    Err(error) => {
                        let s_n = part.serial_no.clone();
                        println!("Inport {}: Failure to push part {} to downstream actor: {}",
                            self.id, s_n, error);

                        // we must still maintain hold of the part
                        self.buffer.push_back(part);
                        
                        MaterialFlowResult(Err(MaterialFlowError::EquipmentFailure))
                    }
                }
            },
            Err(error) => {

                MaterialFlowResult(Err(error))
            }
        }
    }
}

impl Handler<SourceActorCfg> for Inport {
    type Result = ();

    fn handle(&mut self, msg: SourceActorCfg, ctx: &mut Self::Context) -> Self::Result {
        self.next_state(InportEvent::Initialize(msg));
    }
}

impl Handler<LoadCmd> for Inport {
    type Result = MessageResult<LoadCmd>;

    fn handle(&mut self, cmd: LoadCmd, _ctx: &mut Self::Context) -> Self::Result {
        
        let part = cmd.part.clone();

        match self.next_state(InportEvent::LoadCommand(cmd)) {
            TransitionRes::Load(result) => {
                MessageResult(result)
            },
            _ => {
                MessageResult(LoadRes(Err(LoadError{
                    part,
                    reason: LoadErrorReason::InvalidState 
                })))
            }
        }
    }
}

impl Handler<UnloadCmd> for Inport {
    type Result = UnloadRes;

    fn handle(&mut self, cmd: UnloadCmd, _ctx: &mut Self::Context) -> Self::Result {
        self.next_state(InportEvent::UnloadCommand(cmd))
    }
}

impl Handler<StartNextCycleCmd> for Inport {
    type Result = MaterialFlowResult;

    fn handle(&mut self, msg: StartNextCycleCmd, ctx: &mut Self::Context) -> Self::Result {

        match self.next_state(InportEvent::StartNextCycle(())) {
            MaterialFlowResult(result) => result,
            None => {
                let mut error: MaterialFlowError; 
                match self.state {
                    InportState::BusyMaterialFlow => {
                        error = MaterialFlowError::Busy;
                    },
                    InportState::BusyOperation => {
                        error = MaterialFlowError::Busy;
                    },
                    InportState::BlockedByDownstream =>  {
                        error - MaterialFlowError::BlockedByDownstream;
                    }
                }
                return MaterialFlowResult(Err(error));
            }
        }
    }
}

impl Handler<MaterialFlowResult> for Inport {
    type Result = ();

    fn handle(&mut self, msg: MaterialFlowResult, ctx: &mut Self::Context) -> Self::Result {
        
    }
}

