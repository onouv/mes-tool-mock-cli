use super::{State, ToolEntity};



/// A tool when a fault has occurred that prevents it from running.
/// - input port may contain parts, no further parts can be loaded
/// - output port may contain parts and parts may be unloaded
/// - process may contain a part and may be in an inconsistent process step or be incomplete
pub struct Faulted(ToolEntity); 

impl Faulted {
    pub fn new(entity: ToolEntity) -> Self {
        Faulted(entity)
    }

    fn entity(self) -> ToolEntity {
        self.0
    }

}

impl State for Faulted {
    
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