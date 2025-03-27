use actix::{prelude::Message, Recipient};

use super::{
    material_flow::{
        LoadCmd, 
        StartNextCycleCmd, 
        UnloadCmd}, 
    process::{
        AbortOperationCmd,
        StartOperationCmd
    }, 
    Part, 
    ToolSnapshot
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct ToolActorCfg {
    pub downstream: Recipient<StartNextCycleCmd>,
    pub load: Recipient<LoadCmd>,
    pub unload: Recipient<UnloadCmd>,
    pub start_op: Recipient<StartOperationCmd>,
    pub abort_op: Recipient<AbortOperationCmd>
}

#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct StartCmd;

#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct StopCmd;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ProcessFinishedSgn;

#[derive(Message, Debug)]
#[rtype(result = "ToolSnapshot")]
pub struct LoadPartCmd {
    pub part: Part
}

#[derive(Debug)]
pub struct UnloadPartResponse { 
    pub part: Option<Part>,
    pub snapshot: ToolSnapshot
}

#[derive(Message)]
#[rtype(result = "UnloadPartResponse")]
pub struct UnloadPartCmd;

#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct AbortCmd;