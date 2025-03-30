use actix::{
    dev::{MessageResponse, OneshotSender}, 
    prelude::Actor, 
    Message, Recipient
};

use super::{ProcessResult, LoadRes, Part, ProcessFinishedSgn, UnloadRes};

pub enum MaterialFlowError {
    BufferFull,
    BlockedByDownStream(Part),
    BufferEmpty,
    EquipmentFailure,
    Unknown
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MaterialFlowResult(pub Result<(), MaterialFlowError>);

// impl<A, M> MessageResponse<A, M> for MaterialFlowResult
// where
//     A: Actor,
//     M: Message<Result = MaterialFlowResult>,
// {
//     fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
//         if let Some(tx) = tx {
//             let _ = tx.send(self);
//         }
//     }
// }

#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct StartNextCycleCmd;

#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct PushDownstreamCmd { pub part: Part }

#[derive(Message)]
#[rtype(result = "()")]
pub struct InlineActorCfg {
    pub downstream: Recipient<PushDownstreamCmd>,
    pub upstream:  Recipient<MaterialFlowResult>,
    pub parent: Recipient<ProcessFinishedSgn>,
    pub parent_result_handler: Recipient<ProcessResult>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SourceActorCfg {
    pub user_load: Recipient<LoadRes>,
    // pub user_unload: Recipient<UnloadRes>,
    // pub upstream: Recipient<MaterialFlowResult>, 
    // pub downstream: Recipient<PushDownstreamCmd>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SinkActorCfg {
    pub upstream: Recipient<MaterialFlowResult>,
    pub loadport_result: Recipient<MaterialFlowResult>
}

