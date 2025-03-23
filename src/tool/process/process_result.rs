use crate::tool::Part;
use actix::prelude::*;
use actix::dev::{MessageResponse, OneshotSender};

use super::ProcessError;

pub enum ProcessResult {
    Ok,
    Error(ProcessError)
}

impl<A, M> MessageResponse<A, M> for ProcessResult
where
    A: Actor,
    M: Message<Result = ProcessResult>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}