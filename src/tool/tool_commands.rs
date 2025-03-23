use actix::prelude::Message;

use super::{ToolSnapshot, Part};


#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct Start;

#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct Stop;


#[derive(Message)]
#[rtype(result = "()")]
pub struct ProcessFinished;

#[derive(Message, Debug)]
#[rtype(result = "ToolSnapshot")]
pub struct LoadPart {
    pub part: Part
}

#[derive(Debug)]
pub struct UnloadPartResponse { 
    pub part: Option<Part>,
    pub snapshot: ToolSnapshot
}

#[derive(Message)]
#[rtype(result = "UnloadPartResponse")]
pub struct UnloadPart;

#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct Abort;