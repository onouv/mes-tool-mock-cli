use actix::prelude::*;
use std::collections::VecDeque;

use super::Part;

pub struct Tool {
    id: String,
    inport: VecDeque<Part>,
    process: Vec<Part>,
    outport: VecDeque<Part>,
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

impl Tool {
    pub fn new(id: &str) -> Self {
        Tool {
            id: id.to_string(),
            inport: VecDeque::new(),
            process: Vec::new(),
            outport: VecDeque::new(),
        }
    }
}


