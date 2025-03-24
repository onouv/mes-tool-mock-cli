
#[derive(Debug)]
pub enum Reason {
    SupplyDropOut,          // electricty r media lost during process
    Operator,               // operator interaction
    EquipmentFailure,       // technical breakdown 
    Part,                   // incoming part failure (e.g. dimensions)
    ProcessWindowViolated,  // one or more process parameters out of limits
    Unknown
}

#[derive(Debug)]
pub enum ProcessError {
    Unavailable,
    Busy,
    DownstreamBlocked,
    MaterialMissing,
    OperationFailed(Reason)
}