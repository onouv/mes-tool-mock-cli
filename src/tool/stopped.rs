use std::cell::RefCell;

use super::{ State, ToolEntity };



/// A tool in the state when it has been actively commanded to stop processing.
/// This state guarantees en empty process, but parts may remain in the input ports and/or output ports. 
pub struct Stopped {
    entity: Box<RefCell<ToolEntity>>
}

impl Stopped {
    pub fn new(entity: ToolEntity) -> Self {

        Stopped { entity: Box::new(RefCell::new(entity)) }
    }

    
}


impl State for Stopped {

    fn entity(self) -> ToolEntity {
        stopped.exit();
        self.entity.into_inner()
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