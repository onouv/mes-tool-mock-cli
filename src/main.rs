#![allow(warnings)]

mod tool;
use std::io::{stdout, Write};
use text_io::read;

use tool::*;

use actix::prelude::*;

#[actix::main]
async fn main() {
    
    // use std::io::{stdin,stdout,Write};
    // let mut s=String::new();
    // print!("Please enter some text: ");
    // let _=stdout().flush();
    // stdin().read_line(&mut s).expect("Did not enter a correct string");
    // if let Some('\n')=s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r')=s.chars().next_back() {
    //     s.pop();
    // }
    // println!("You typed: {}",s);
    
    

    let tool_1 = Tool::new("1").start();
    // let tool_2 = Tool::new("2").start();
    
    
    // let res = tool_1.send(tool::signal::Start).await.unwrap();
    // println!("Tool 1: {:?}", res);

    // let res = tool_1.send(LoadPart{ part: Part::new("1", "123")}).await.unwrap();
    // println!("Tool 1: {:?}", res);

    // let res = tool_1.send(Start).await.unwrap();
    // println!("Tool 1: {:?}", res);

    let mut part_no = 0;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        match retrieve_tool_command().as_str() {
            "start" => {
                let res = tool_1.send(Start).await.unwrap();
                println!("Tool 1: {:?}", res);
            },
            "stop" => {
                let res = tool_1.send(Stop).await.unwrap();
                println!("Tool 1: {:?}", res);
            },
            "load" => {
                let part_no = part_no + 1;
                let part = Part::new(&part_no.to_string(), "123");
                let res = tool_1.send(LoadPart{ part }).await.unwrap();
                println!("Tool 1: {:?}", res);
            },
            "unload" => {
                let res = tool_1.send(UnloadPart).await.unwrap();
                println!("Tool 1: {:?}", res);
            },
            "abort" => {
                let res = tool_1.send(Abort).await.unwrap();
                println!("Tool 1: {:?}", res);
            },
            "exit" => {
                break;
            },
            _ => {
                println!("Invalid command.");
            }
        }
    }
   
}

fn retrieve_tool_command() -> String {
    let mut input = String::new();
    
    loop {
        let input: String = read!("{}\n");
        return match input.trim() {
                    "start" => {
                        "start".to_string()
                    },
                    "stop" => {
                        "stop".to_string()
                    },
                    "load" => {
                        "load".to_string()
                    },
                    "unload" => {
                        "unload".to_string()
                    },
                    "abort" => {
                        "abort".to_string()
                    },
                    "exit" => {
                        "exit".to_string()
                    },
                    "q" => {
                        "exit".to_string()
                    },
                    _ => {
                        println!("Invalid command.");
                        continue;
                    }
                }
        }
    }
