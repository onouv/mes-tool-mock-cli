use actix::prelude::Message;

use super::ToolSnapshot;


#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct Start;

#[derive(Message)]
#[rtype(result = "ToolSnapshot")]
pub struct Stop;

