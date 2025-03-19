use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolState {
    STOPPED,
    IDLE,
    RUNNING,
    STOPPING,
    FAULTED
}

impl Display for ToolState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToolState::STOPPED => write!(f, "STOPPED"),
            ToolState::IDLE => write!(f, "IDLE"),
            ToolState::RUNNING => write!(f, "RUNNING"),
            ToolState::STOPPING => write!(f, "STOPPING"),
            ToolState::FAULTED => write!(f, "FAULTED")
        }
    }
}