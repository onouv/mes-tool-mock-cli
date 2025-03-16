use actix::prelude::*;
use std::collections::VecDeque;
use std::fmt::{self, Display};

use crate::tool::state::ToolState;

use super::Part;
use super::signal::{Start, Stop};
use super::ToolSnapshot;

static INPORT_CAPACITY: usize = 3;

#[derive(Debug)]
pub struct Tool {
    id: String,
    inport: VecDeque<Part>,
    process: Vec<Part>,
    outport: VecDeque<Part>,
    state: ToolState
}

impl Actor for Tool {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Tool {} actor started.", self.id);
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Tool {} actor stopped.", self.id);
    }
}

impl Handler<Start> for Tool {
    type Result = ToolSnapshot;
    fn handle(&mut self, _msg: Start, _ctx: &mut Context<Self>) -> Self::Result {
        
        match self.state {
            ToolState::STOPPED => {
                if self.inport_empty() {        
                    self.state = ToolState::IDLE;
                } else if self.outport_free() {
                    self.shift_part_to_process();
                    self.state = ToolState::RUNNING;
                }
            },
            _ => ()
        }
        
        self.snapshot()        
    }
}

impl Handler<Stop> for Tool {
    type Result = ToolSnapshot;
    fn handle(&mut self, _msg: Stop, _ctx: &mut Context<Self>) -> Self::Result {
        
        match self.state {
            ToolState::IDLE => {
                self.state = ToolState::STOPPED
            },
            ToolState::RUNNING => {
                
                self.shift_part_to_outport();
                self.state = ToolState::IDLE;
            },
            _ => ()
        }
        
        self.snapshot()        
    }
}

impl Tool {
    pub fn new(id: &str) -> Self {
        Tool {
            id: id.to_string(),
            inport: VecDeque::new(),
            process: Vec::new(),
            outport: VecDeque::new(),
            state: ToolState::STOPPED
        }
    }

    fn inport_free(&self) -> bool {
        self.inport.len() < INPORT_CAPACITY
    }

    fn inport_empty(&self) -> bool {
        self.inport.is_empty()
    }

    fn process_empty(&self) -> bool {
        self.process.is_empty()
    }

    fn outport_free(&self) -> bool {
        self.outport.len() < INPORT_CAPACITY
    }

    fn outport_empty(&self) -> bool {
        self.outport.is_empty()
    }

    fn load_part(&mut self, part: Part) {
        self.inport.push_back(part);
    }

    fn shift_part_to_process(&mut self) {
        self.process.push(self.inport.pop_back().unwrap());
    }
    
    fn shift_part_to_outport(&mut self) {
        self.outport.push_front(self.process.pop().unwrap());
    }
                        

    fn snapshot(&self) -> ToolSnapshot {
        ToolSnapshot {
            id: self.id.clone(),
            parts_inport: self.inport.len(),
            process_occupied: !self.process_empty(),
            process_completion: None,
            parts_outport: self.outport.len(),
            state: self.state.clone()
        }
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tool {} in state {}", self.id, self.state)
    }
}