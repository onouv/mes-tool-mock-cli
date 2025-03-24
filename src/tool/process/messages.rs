
use actix::{
    dev::{MessageResponse, OneshotSender}, 
    prelude::Actor, 
    Message
};

use super::{super::Part, ProcessError};

#[derive(Message)]
#[rtype(result = "ProcessResult")]
pub struct StartOperationCmd;

#[derive(Message)]
#[rtype(result = "ProcessResult")]
pub struct AbortOperationCmd;

pub struct ProcessResult(pub Result<(), ProcessError>);

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