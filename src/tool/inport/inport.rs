use std::collections::VecDeque;
use actix::prelude::*;

use crate::tool::{material_flow::*, Part};

static INPORT_BUFFER_SIZE: usize = 10;

/// 
/// Outport is a material flow source.
/// 
/// It can handle commands to:
/// - load part from the upstream user into its buffer 
/// - unload a part from its buffer back upstream to the user
/// - push a part to a downstream downstream recipient (FIFO)
///
#[derive(Debug)]
pub struct Inport {
    id: String,
    buffer: VecDeque<Part>,
    downstream: Recipient<LoadCmd>,
    is_handling: bool
}

impl Actor for Inport {
    type Context = Context<Self>;

}

impl Inport {
    pub fn new(id: String, downstream: Recipient<LoadCmd>) -> Self {
        Self {
            id,
            buffer: VecDeque::default(),
            downstream,
            is_handling: false
        }
    }

    fn push(&mut self, part: Part) -> Result<(), MaterialFlowError>{
        if self.buffer.len() < INPORT_BUFFER_SIZE {
            self.buffer.push_front(part);
            return Ok(());
        }

        Err(MaterialFlowError::Full)
    }

    fn pop(&mut self) -> Result<Part, MaterialFlowError> {
        if let part = self.buffer.pop_back() {
            return Ok(part.unwrap());
        }

        Err(MaterialFlowError::Empty)
    }
}

impl Handler<LoadCmd> for Inport {
    type Result = MaterialFlowResult;

    fn handle(&mut self, msg: LoadCmd, _ctx: &mut Self::Context) -> Self::Result {
    
        // reject new parts while still handling downstream push 
        // to protect against overflow of part buffer 
        if self.is_handling {
            return MaterialFlowResult(Err(MaterialFlowError::Busy));
        }

        match self.push(msg.part) {
            Ok(()) => {
                return MaterialFlowResult(Ok(MaterialFlowStatus {
                    buffer: self.buffer_status(),
                    action: MaterialFlowAction::PushedDownstream
                }));
            },
            Err(error) => {
                return MaterialFlowResult(Err(error));
            }
        }
    }
}

impl Handler<UnloadCmd> for Inport {
    type Result = MaterialFlowResult;

    fn handle(&mut self, msg: UnloadCmd, _ctx: &mut Self::Context) -> Self::Result {
        match self.pop() {
            Ok(part) => {
                MaterialFlowResult(Ok(
                    MaterialFlowStatus { 
                        buffer: self.buffer_status(), 
                        action: MaterialFlowAction::Unloaded(part) 
                    }
                ))
            },
            Err(error) => {
                MaterialFlowResult(Err(error))
            }
        }
    }
}

impl Handler<PushDownNextCmd> for Inport {
    type Result = MaterialFlowResult;

    fn handle(&mut self, msg: PushDownNextCmd, ctx: &mut Self::Context) -> Self::Result {
        if self.is_handling {
            return MaterialFlowResult(Err(MaterialFlowError::Busy));
        }
        self.is_handling = true;

        match self.pop() {
            Ok(part) => {
                match self.downstream.try_send(LoadCmd { part: part.clone() }) {
                    Ok(()) => {                        
                        self.is_handling = false;

                        MaterialFlowResult(Ok(MaterialFlowStatus { 
                            buffer: self.buffer_status(),
                            action: MaterialFlowAction::PushedDownstream 
                        }))
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
                self.is_handling = false;

                MaterialFlowResult(Err(error))
            }
        }
    }
}

impl BufferInfo for Inport {
    fn buffer_status(&self) -> BufferStatus {
        BufferStatus { 
            is_full: self.buffer.len() >= INPORT_BUFFER_SIZE,
            is_empty: self.buffer.is_empty()
        }
    }
}