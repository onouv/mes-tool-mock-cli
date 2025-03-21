use std::time::Duration;
use actix::prelude::*;
use actix::clock::sleep;
use tokio::task::JoinHandle;

use super::signal::ProcessFinished;
use super::{Part, Tool};

#[derive(Debug)]
pub struct Process {
    name: String,
    part: Option<Part>,
    task: Option<SpawnHandle>,
    cycle_time: Duration
}

impl Process {
    pub fn new(name: &str, cycle_time: Duration) -> Self {
        Self {
            name: name.to_string(),
            part: None,
            task: None,
            cycle_time
        }
    }

    pub fn load_part(&mut self, part: Part) {
        self.part = Some(part);
    }

    pub fn unload_part(&mut self) -> Option<Part> {
        self.part.take()
    }

    pub fn is_empty(&self) -> bool {
        self.part.is_none()
    }

    pub fn start(&mut self, actor: &Tool, ctx: &mut Context<Tool>) {
        let cycle_time = self.cycle_time;
        let ctx_address = ctx.address();

        if let Some(part) = self.part.as_mut() {
            part.add_step(&self.name);
            part.failed = false;

            println!("process starting for part {}", part.serial_no);

            let future = Box::pin(async move {
                sleep(cycle_time).await;
                ctx_address.do_send(ProcessFinished);
            });

            let handle = ctx.spawn(future.into_actor(actor));
            self.task = Some(handle);
        } 
    }

    pub async fn abort(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
            let part = self.part.as_mut().unwrap();
            part.failed = true;
        }
    }
}