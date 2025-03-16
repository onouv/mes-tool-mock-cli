use super::faulted::Faulted;
use super::signal::Signal;
use super::state::State;
use super::Stopped;
use super::Running;
use super::Idle;
use super::ToolEntity;


/// The actual state machine.
/// Responisble for handling signals and transitioning between states.
/// 
pub enum Tool {
    Stopped(Stopped),
    Idle(Idle),
    Running(Running),
    Stopping,
    Fault(Faulted)
}
impl Tool {

    /// 
    /// constructor for the initial state
    /// 
    pub fn new(id: &str) -> Self {
        Tool::Stopped(Stopped::new(ToolEntity::new(id)))
    }

    


    /// 
    /// Handles signal for each state.
    /// 
    /// Makes decision about transitions based on state of the tool entity
    /// Upon transitioning, it calls exit() on old state and enter() and 
    /// then execute() on new state. 
    /// 
    /// May return the new state of the tool while consuming the old state, 
    /// or may return the old state instead (no transition). 
    /// 
    pub fn handle(self, signal: Signal) -> Self {
        match (self, signal) {
            (Tool::Stopped(stopped), Signal::Start) => {
                stopped.exit();
                let entity = stopped.entity();
                
                if entity.inport_empty() {
                    
                    let idle = Tool::Idle(Idle::new(entity));
                    idle.enter();
                    idle.execute();
                    
                    idle
                } else {
                    stopped.exit();
                    let running = Tool::Running(Running::new(entity));
                    running.enter();
                    running.execute();

                    running
                }

                self
            },
            (Tool::Idle(mut idle), Signal::LoadPart(part)) => {
                let entity = idle.entity();

                if entity.inport_free() {
                    entity.load_part(part);
                
                    if entity.process_empty() && entity.outport_free() {
                        idle.exit();
                        let running = Tool::Running(Running::new(entity));
                        running.enter();
                        running.execute();

                        running
                    } 
                }

                self
            },
            (Tool::Idle(mut tool), Signal::UnloadPart) => {
                // tool.0.outport.push_back(tool.0.process.pop().unwrap());
                tool
            },
            (Tool::Running(mut tool), Signal::LoadPart(part)) => {
                // load_inport(&mut tool.0, part);
                // Tool::Running(Running::new(tool.0))
                tool
            },
            (Tool::Fault(tool), Signal::ClearFault) => {
                tool
            },
            _ => self
        }
    }
}


// impl Display for Tool {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Tool::Stopped(tool) => {
//                 write!(f, "{} : Stopped, {} parts total", tool.0.id, calc_parts_total(&tool.0))
//             },
//             Tool::Idle(tool) => {
//                 write!(f, "{} : Idle, {} parts total", tool.0.id, calc_parts_total(&tool.0))
//             },
//             Tool::Running(tool) => {
//                 write!(f, "{} : Running, {} parts total", tool.0.id, calc_parts_total(&tool.0))
//             },
//             Tool::Fault(tool) => {
//                 write!(f, "{} : Fault, {} parts total", tool.0.id, calc_parts_total(&tool.0))
//             },
//             Tool::Stopping(tool) => {
//                 write!(f, "{} : Stopping, {} parts total", tool.0.id, calc_parts_total(&tool.0))
//             }
//         }
//     }
// }

