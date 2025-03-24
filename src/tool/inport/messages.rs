use actix::Message;


use super::super::Part;
use super::super::MaterialFlowResult;

#[derive(Message)]
#[rtype(result = "MaterialFlowResult")]
pub struct Load {
    part: Part
}