use actix::{
    dev::{MessageResponse, OneshotSender}, 
    prelude::Actor, 
    Message, Recipient
};

use super::{process::ProcessResult, Part, ProcessFinishedSgn};



pub trait Buffer {
    fn buffer_status(&self) -> BufferStatus;
    fn push(&mut self, part: Part) -> Result<(), MaterialFlowError>;
    fn pop(&mut self) -> Result<Part, MaterialFlowError>;
}

pub enum MaterialFlowAction {
    Loaded,
    PushedDownstream,
    Unloaded(Part)
}

pub enum BufferStatus {
    Empty,
    Full,
    Balanced
}

pub enum MaterialFlowError {
    Full,
    BlockedByDownstream,
    Empty,
    Busy,
    EquipmentFailure
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MaterialFlowResult(pub Result<BufferStatus, MaterialFlowError>);

impl<A, M> MessageResponse<A, M> for MaterialFlowResult
where
    A: Actor,
    M: Message<Result = MaterialFlowResult>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct LoadCmd {
    pub part: Part
}

#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct StartNextCycleCmd;

#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct UnloadCmd;

/// Accept incoming part and push next part in line to downstream recipient.
/// This operation is expected to continue in chain down the stream.
/// 
#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct PushDownstreamCmd { pub part: Part }

#[derive(Debug)]
pub struct MaterialFlowInlineHandlers {
    pub downstream: Recipient<PushDownstreamCmd>,
    pub upstream:  Recipient<MaterialFlowResult>,
    pub parent: Recipient<ProcessFinishedSgn>,
    pub parent_result_handler: Recipient<ProcessResult>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct InlineActorCfg {
   pub handlers: MaterialFlowInlineHandlers
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SourceActorCfg {
    pub upstream: Recipient<MaterialFlowResult>, 
    pub downstream: Recipient<PushDownstreamCmd>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SinkActorCfg {
    pub upstream: Recipient<MaterialFlowResult>,
    pub loadport_result: Recipient<MaterialFlowResult>
}

