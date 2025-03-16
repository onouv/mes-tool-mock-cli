mod tool;
use tool::Tool;

use actix::prelude::*;

#[actix::main]
async fn main() {
    let tool_1 = Tool::new("1").start();
    let tool_2 = Tool::new("2").start();
    let tool_3 = Tool::new("3").start();

    let res = tool_1.send(tool::signal::Start).await.unwrap();
    println!("Tool 1 state: {:?}", res);
    tool_2.send(tool::signal::Start).await.unwrap();
    tool_3.send(tool::signal::Start).await.unwrap();
}