#![allow(warnings)]
use std::{io::{stdout, Write}, process::{Command, Stdio}};
use text_io::read;
use actix::prelude::*;

mod tool;
use tool::{ create_tool, LoadPartCmd, Part, StartCmd, Tool};

#[actix::main]
async fn main() {
    let tool1 = create_tool("=001");
    tool1.await.send(LoadPartCmd{ part: Part::new("123-446-554", "12435") });
    let tool1_snapshot = tool1.await.send(StartCmd);

    
}

