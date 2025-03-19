use std::collections::VecDeque;
use super::part::Part;

static INPORT_CAPACITY: usize = 3;

pub struct ToolEntity {
    id: String,
    inport: VecDeque<Part>,
    process: Vec<Part>,
    outport: VecDeque<Part>,
}

impl ToolEntity {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            inport: VecDeque::default(),
            process: vec![],
            outport: VecDeque::default(),
        }
    }

    pub fn inport_free(&self) -> bool {
        self.inport.len() < INPORT_CAPACITY
    }

    pub fn inport_empty(&self) -> bool {
        self.inport.is_empty()
    }

    pub fn process_empty(&self) -> bool {
        self.process.is_empty()
    }

    pub fn outport_free(&self) -> bool {
        self.outport.len() < INPORT_CAPACITY
    }

    pub fn outport_empty(&self) -> bool {
        self.outport.is_empty()
    }

    pub fn load_part(&mut self, part: Part) {
        self.inport.push_back(part);
    }

    pub fn shift_part_to_process(&mut self) {
        self.process.push(self.inport.pop_back().unwrap());
    }
    
    pub fn shift_part_to_outport(&mut self) {
        self.outport.push_front(self.process.pop().unwrap());
    }
    
}
