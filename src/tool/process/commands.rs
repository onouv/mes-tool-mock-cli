
use actix::Message;

use super::super::Part;
use super::ProcessResult;

#[derive(Message)]
#[rtype(result = "ProcessResult")]
pub struct StartOperation;

#[derive(Message)]
#[rtype(result = "ProcessResult")]
pub struct StockOperation { pub part: Part }

