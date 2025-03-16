use std::cell::RefCell;

use super::{state::State, ToolEntity};

/// A tool in a state which prevents it from running:
/// - empty input port
/// - full output port
pub struct Idle {
    entity: Box<RefCell<ToolEntity>>
};

impl Idle {
    pub fn new(entity: Box<RefCell<ToolEntity>>) -> Self {
        Idle { entity }
   }
}


impl State for Idle {

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