use actix::prelude::Message;

use super::{ToolSnapshot, Part};


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