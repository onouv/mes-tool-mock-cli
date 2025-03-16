use super::{Part, State, ToolEntity};

/// A tool in a state where it is actively processing parts.
/// - input port may contain parts
/// - output port may contain parts
/// - process guaranteed to contain a part
pub struct Running(ToolEntity); 
impl Running {
    pub fn new(entity: ToolEntity) -> Self {
        Running(entity)
    }

    fn entity(&mut self) -> &mut ToolEntity {
        &mut self.0
    }
}

impl State for Running {

    fn entity(&mut self) -> &mut ToolEntity {
        self.entity.borrow_mut()
    }

    fn enter(&mut self) {
        println!("Running: enter");
    }

    fn execute(&mut self) {
        println!("Running: execute");
    }

    fn exit(&mut self) {
        println!("Running: exit");
    }
}