mod tool;
use tool::Tool;

use actix::prelude::*;

#[actix::main]
async fn main() {
    let tool_1 = Tool::new("1", ).start();
    let tool_2 = Tool::new("2", ).start();
    let tool_3 = Tool::new("3", ).start();
}