use std::cell::RefCell;

use super::tool_entity::ToolEntity;


pub trait State {
    fn entity(self) -> ToolEntity;
    fn enter(&mut self);
    fn execute(&mut self);
    fn exit(&mut self);
}