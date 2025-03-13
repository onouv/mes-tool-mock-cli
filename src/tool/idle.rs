use super::{State, ToolEntity};

/// A tool in a state which prevents it from running:
/// - empty input port
/// - full output port
pub struct Idle(ToolEntity);

impl Idle {
    pub fn new(entity: ToolEntity) -> Self {
        Idle (entity)
    }
}

impl State for Idle {

    fn entity(&mut self) -> &mut ToolEntity {
        &mut self.0
    }
    
    fn enter(&mut self) {
        println!("Idle: enter");
    }

    fn execute(&mut self) {
        println!("Idle: execute");
    }

    fn exit(&mut self) {
        println!("Idle: exit");
    }
}