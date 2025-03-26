use actix::{
    dev::{MessageResponse, OneshotSender}, 
    prelude::Actor, 
    Message, Recipient
};

use super::{process::ProcessResult, Part, ProcessFinishedSgn};

pub struct BufferStatus {
    pub is_full: bool,
    pub is_empty: bool,
}

pub trait BufferInfo {
    fn  buffer_status(&self) -> BufferStatus;
}

pub enum MaterialFlowAction {
    Loaded,
    PushedDownstream,
    Unloaded(Part)
}

pub struct MaterialFlowStatus {
    pub buffer: BufferStatus,
    pub action: MaterialFlowAction
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
pub struct MaterialFlowResult(pub Result<MaterialFlowStatus, MaterialFlowError>);

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

pub trait MaterialFlowSourceActor {
    fn setHandlers(&mut self, upstream: Recipient<MaterialFlowResult>, downstream: Recipient<PushDownstreamCmd>);
}

pub trait MaterialFlowChainedActor {
    fn setHandlers(&mut self, handlers: MaterialFlowChainHandlers);
}

#[derive(Debug)]
pub struct MaterialFlowChainHandlers {
    pub downstream: Recipient<PushDownstreamCmd>,
    pub upstream:  Recipient<MaterialFlowResult>,
    pub parent: Recipient<ProcessFinishedSgn>,
    pub parent_result_handler: Recipient<ProcessResult>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct InitializeChainedActorCmd {
   pub handlers: MaterialFlowChainHandlers
}