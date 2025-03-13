use super::state::State;
use super::ToolEntity;


pub struct Stopping (ToolEntity);

impl Stopping {
    pub fn new(entity: ToolEntity) -> Self {

        Stopping (entity)
    }
}

impl State for Stopping {

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