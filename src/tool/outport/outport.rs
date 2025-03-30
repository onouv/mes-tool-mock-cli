use core::error;
use std::collections::VecDeque;

use actix::prelude::*;

use crate::tool::{
    material_flow::*,
    Part};

static OUTPORT_BUFFER_SIZE: usize = 10;

/// 
/// Outport is a material flow sink.
/// 
/// Therefore, it does not handle 
/// commands to push parts further downstream. 
/// 
/// It can handle only commands to:
/// - unload a part from its buffer
/// - being loaded a part into its buffer from an upstream actor
/// 
#[derive(Debug)]
pub struct Outport {
    id: String,
    buffer: VecDeque<Part>,
}

impl Actor for Outport {
    type Context = Context<Self>;
}

impl Outport {
    pub fn new(id: String) -> Self {
        Self {
            id,
            buffer: VecDeque::default(),
        }
    }

    fn push(&mut self, part: Part) -> Result<(), MaterialFlowError>{
        if self.buffer.len() < OUTPORT_BUFFER_SIZE {
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

impl Handler<PushDownstreamCmd> for Outport {
    type Result = MaterialFlowResult;

    /// 
    /// Accept incoming part form upstream
    /// 
    /// Return Ok(PushedDownstream) part accepted and able to accept further parts
    /// 
    /// Return Ok(Full) if part accepted but unable to accept further parts
    /// 
    /// Return Err(...) accordingly
    /// 
    fn handle(&mut self, msg: PushDownstreamCmd, _ctx: &mut Self::Context) -> Self::Result {
        match self.push(msg.part) {
            Ok(()) => {
                return MaterialFlowResult(Ok(MaterialFlowStatus {
                    buffer: self.buffer_status(),
                    action: MaterialFlowAction::PushedDownstream
                }));
            },
            Err(error) => {
                match error {
                    MaterialFlowError::BlockedByDownStream(p) => {
                        println!("Huhu !");
                    },
                    _ => {}
                }
                return MaterialFlowResult(Err(error));
            }
        }
    }
}

impl Handler<UnloadCmd> for Outport {
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

impl BufferInfo for Outport {
    fn buffer_status(&self) -> BufferStatus {
        BufferStatus { 
            is_full: self.buffer.len() >= OUTPORT_BUFFER_SIZE,
            is_empty: self.buffer.is_empty()
        }
    }
}