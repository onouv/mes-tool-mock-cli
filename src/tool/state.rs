use super::{tool_entity::ToolEntity, Tool};

pub trait State {
    fn entity(&mut self) -> &mut ToolEntity;
    fn enter(&mut self);
    fn execute(&mut self);
    fn exit(&mut self);
}