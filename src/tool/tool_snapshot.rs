use actix::prelude::*;
use actix::dev::{MessageResponse, OneshotSender};

use super::state::ToolState;
use super::process::ProcessSnapshot;

#[derive(Debug)]
pub struct ToolSnapshot {
    pub id: String,
    pub parts_inport: usize,
    pub process: ProcessSnapshot,
    pub parts_outport: usize,
    pub state: ToolState
}

impl<A, M> MessageResponse<A, M> for ToolSnapshot
where
    A: Actor,
    M: Message<Result = ToolSnapshot>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}