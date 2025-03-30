use actix::Message;
use std::collections::VecDeque;
use super::Part;

///
/// Types, Actix-messages and traits for the user-facing side
/// of tool loadports.
/// 


/// A loadport in general can be for loading parts to a tool, 
/// unloading them from a tool or for both features.
/// However, it is assumed that all three types of loadports always 
/// allow to load and unload, even if there primary function is only
/// one of the two, simply to allow for managing failure conditions
/// in the tool. Therefore, the trait defines both features.
pub trait LoadPort {
    fn load(part: Part) -> LoadRes;
    fn unload() -> UnloadRes;
}

#[derive(Message, Clone)]
#[rtype(result = "LoadRes")]
pub struct LoadCmd {
    pub part: Part
}

pub enum LoadPortResult {
    UnloadResult(UnloadRes),
    LoadResult(LoadRes)
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LoadRes(pub Result<(), LoadError>);

pub enum LoadErrorReason {
    BufferFull,
    InvalidState,
    EquipmentFailure,
    Unknown
}

pub struct LoadError{
    pub part: Part,
    pub reason: LoadErrorReason
}

#[derive(Message)]
#[rtype(result = "UnloadRes")]
pub struct UnloadCmd;

#[derive(Message)]
#[rtype(result = "()")]
pub struct UnloadRes(pub Result<Part, UnloadError>);

pub enum UnloadError {
    BufferEmpty,
    EquipmentFailure,
    Unknown
}

#[derive(Debug)]
pub struct Buffer {
    capacity: usize,
    parts: VecDeque<Part>
}
pub enum BufferStatus {
    Empty,
    Balanced,
    Full
}

/// A Buffer hold parts and allows to push parts into them 
/// and push parts out of them. This struct is intended to be 
/// composed into loadport implementations.
impl Buffer {
    pub fn new(capacity: usize) -> Self {
        Self { 
            capacity, 
            parts: VecDeque::default()
        }
    } 

    pub fn buffer_status(&self) -> BufferStatus {
        let num_parts = self.parts.len();
        if num_parts == self.capacity {
            return BufferStatus::Empty;
        }

        if num_parts < self.capacity {
            return BufferStatus::Balanced;
        }

        BufferStatus::Full
    }

    pub fn push(&mut self, part: Part) -> LoadRes {
        if self.parts.len() < self.capacity {
            self.parts.push_front(part);
            
            return LoadRes(Ok(()));
        }

        LoadRes(Err(LoadError{ part, reason: LoadErrorReason::BufferFull}))
    }


    pub fn pop(&mut self) -> UnloadRes {
        if let part = self.parts.pop_back() {
            return UnloadRes(Ok(part.unwrap()));
        }

        UnloadRes(Err(UnloadError::BufferEmpty))
    }
}

