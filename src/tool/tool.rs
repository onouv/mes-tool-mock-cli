use std::collections::VecDeque;
use std::fmt::{self, Display};

use super::part::Part;
use super::signal::Signal;

struct ToolEntity {
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
}

/// A tool in the state when it has been actively commanded to stop processing.
/// This state guarantees en empty process, but parts may remain in the input ports and/or output ports. 
struct Stopped(ToolEntity); // nice and short
impl Stopped {
    pub fn new(entity: ToolEntity) -> Self {

        Stopped (entity)
    }
}

/// A tool in a state which prevents it from running:
/// - empty input por
/// - full output port
struct Idle(ToolEntity);
impl Idle {
    pub fn new(entity: ToolEntity) -> Self {
        Idle (entity)
    }
}

/// A tool in a state where it is actively processing parts.
/// - input port may contain parts
/// - output port may contain parts
/// - process guaranteed to contain a part
struct Running(ToolEntity); 
impl Running {
    pub fn new(entity: ToolEntity) -> Self {
        Running(entity)
    }
}

/// A tool when a fault has occurred that prevents it from running.
/// - input port may contain parts, no further parts can be loaded
/// - output port may contain parts and parts may be unloaded
/// - process may contain a part and may be in an inconsistent process step or be incomplete
struct Faulted(ToolEntity); 

impl Faulted {
    pub fn new(entity: ToolEntity) -> Self {
        Faulted(entity)
    }
}

/// The actual state machine.
pub enum Tool {
    Stopped(Stopped),
    Idle(Idle),
    Running(Running),
    Fault(Faulted)
}
impl Tool {

    /// constructor for the initial state
    pub fn new(id: &str) -> Self {
        Tool::Stopped(Stopped::new(ToolEntity::new(id)))
    }

    /// All possible state transitions, triggered by signals
    pub fn handle(self, signal: Signal) -> Self {
        match (self, signal) {
            (Tool::Stopped(tool), Signal::Start) => {
                Tool::Idle(Idle::new(tool.0))
            },
            (Tool::Idle(mut tool), Signal::LoadPart(part)) => {
                load_inport(&mut tool.0, part);
                shift_part_to_process(&mut tool.0);

                Tool::Running(Running::new(tool.0))
            },
            (Tool::Idle(mut tool), Signal::UnloadPart) => {
                tool.0.outport.push_back(tool.0.process.pop().unwrap());
                Tool::Idle(tool)
            },
            (Tool::Running(mut tool), Signal::LoadPart(part)) => {
                load_inport(&mut tool.0, part);

                Tool::Running(Running::new(tool.0))
            }
            (Tool::Running(mut tool), Signal::UnloadPart) => {
                tool.0.outport.pop_back();
                Tool::Running(tool)
                // Tool::Running(Running::new(tool.0))
            }
            _ => todo!()
        }
    }
}


impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tool::Stopped(tool) => {
                write!(f, "{} : Stopped, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            },
            Tool::Idle(tool) => {
                write!(f, "{} : Idle, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            },
            Tool::Running(tool) => {
                write!(f, "{} : Running, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            },
            Tool::Fault(tool) => {
                write!(f, "{} : Fault, {} parts total", tool.0.id, calc_parts_total(&tool.0))
            }
        }
        
    }
}


// private functions to implement the actual actions required for the state transitions, 
// acting on the ToolEntity instance passed in from the state tuple structs.
//
// Note these methods trust on consistent internal state of the tool member fields,
// as they are only called from the state machine, which is the only one to have access to
// the ToolEntity instance.

fn shift_part_to_process(tool: &mut ToolEntity) {
    tool.process.push(tool.inport.pop_back().unwrap());
}

fn load_inport(tool: &mut ToolEntity, part: Part) {
    tool.inport.push_front(part);
}

fn shift_part_to_outport(tool: &mut ToolEntity) {
    tool.outport.push_back(tool.process.pop().unwrap());
}

fn calc_parts_total(tool: &ToolEntity) -> usize {
    tool.inport.len() + tool.process.len() + tool.outport.len()
}