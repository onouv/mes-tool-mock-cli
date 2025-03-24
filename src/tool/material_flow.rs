use actix::{
    dev::{MessageResponse, OneshotSender}, 
    prelude::Actor, 
    Message
};

use super::Part;

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
pub struct UnloadCmd;

/// 
/// Push next part in line to downstream recipient.
/// 
#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct PushDownNextCmd;