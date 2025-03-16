mod tool;
use tool::Tool;

use actix::prelude::*;

#[actix::main]
async fn main() {
    let tool_1 = Tool::new("1", ).start();
    
}