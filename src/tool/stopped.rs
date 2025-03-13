use super::{State, Tool, ToolEntity};



/// A tool in the state when it has been actively commanded to stop processing.
/// This state guarantees en empty process, but parts may remain in the input ports and/or output ports. 
pub struct Stopped(ToolEntity);
impl Stopped {
    pub fn new(entity: ToolEntity) -> Self {

        Stopped (entity)
    }
}


impl State for Stopped {
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