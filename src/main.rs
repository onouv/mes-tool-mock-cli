#![allow(warnings)]
use std::{io::{stdout, Write}, process::{Command, Stdio}};
use text_io::read;
use actix::prelude::*;

mod tool;
use tool::*;

static prompt: &str = ". ";
enum UserCommand {
    start,
    stop,
    abort,
    load,
    unload,
    exit,
    nothing
}

#[actix::main]
async fn main() {
    let tool_1 = Tool::new("1").start();
    let mut part_no = 0;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        match retrieve_tool_command() {
            UserCommand::start => {
                let res = tool_1.send(StartCmd).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::stop => {
                let res = tool_1.send(StopCmd).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::load => {
                let part_no = part_no + 1;
                let part = Part::new(&part_no.to_string(), "123");
                let res = tool_1.send(LoadPartCmd{ part }).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::unload => {
                let res = tool_1.send(UnloadPartCmd).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::abort => {
                let res = tool_1.send(AbortCmd).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::exit => {
                break;
            },
            _ => {
                println!("Invalid command.");
            }
        }        
    }
   
}

fn retrieve_tool_command() -> UserCommand {
    
    let input: String = read!("{}\n");
    return match input.as_str() {
        "start" => {
            UserCommand::start
        },
        "stop" => {
            UserCommand::stop
        },
        "load" => {
            UserCommand::load
        },
        "unload" => {
            UserCommand::unload
        },
        "abort" => {
            UserCommand::abort
        },
        "exit" => {
            UserCommand::exit
        },
        "q" => {
            UserCommand::exit
        },
        _ => {
            UserCommand::nothing
        }
    }
}
