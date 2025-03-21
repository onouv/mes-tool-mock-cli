#![allow(warnings)]

mod tool;
use std::{io::{stdout, Write}, process::{Command, Stdio}};
use text_io::read;

use tool::*;

use actix::prelude::*;

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
                let res = tool_1.send(Start).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::stop => {
                let res = tool_1.send(Stop).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::load => {
                let part_no = part_no + 1;
                let part = Part::new(&part_no.to_string(), "123");
                let res = tool_1.send(LoadPart{ part }).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::unload => {
                let res = tool_1.send(UnloadPart).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::abort => {
                let res = tool_1.send(Abort).await.unwrap();
                println!("{}Tool 1: {:?}", prompt, res);
            },
            UserCommand::exit => {
                break;
            },
            _ => {
                println!("Invalid command.");
            }
        }
        std::io::flush()
        
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

fn clean_input(input: String) -> String {
    let first_of_prompt: usize = 0;
    let last_of_prompt: usize = prompt.chars().count();
    substring(input, first_of_prompt, last_of_prompt)
}

fn substring(s: String, start: usize, len: usize) -> String {
    let mut char_pos = 0;
    let mut byte_start = 0;
    let mut it = s.chars();

    // find start position
    loop {
        if char_pos == start { break; }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_start += c.len_utf8();
        }
        else { break; }
    }

    // find end position
    char_pos = 0;
    let mut byte_end = byte_start;
    loop {
        if char_pos == len { break; }
        if let Some(c) = it.next() {
            char_pos += 1;
            byte_end += c.len_utf8();
        }
        else { break; }
    }
    
    // return owned String
    s[byte_start..byte_end].to_string()
}
