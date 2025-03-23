#[derive(Debug)]
pub enum ProcessError {
    Occupied,
    Empty,
    Unavailable,
    Busy,
    OperationFailed
}