use std::time::Duration;
use actix::prelude::*;
use actix::clock::sleep;
use tokio::task::JoinHandle;

use super::signal::ProcessFinished;
use super::{Part, Tool};

#[derive(Debug)]
pub struct Process {
    part: Option<Part>,
    task: Option<JoinHandle<()>>,
    cycle_time: Duration
}

impl Process {
    pub fn new(cycle_time: Duration) -> Self {
        Self {
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

    pub async fn start(&mut self, ctx: &mut Context<Tool>) {
        let cycle_time = self.cycle_time;
        let ctx_address = ctx.address();

        let part = self.part.as_mut().unwrap();
        part.add_step("process");
        part.failed = false;

        println!("process starting for part {}", part.serial_no);

        self.task = Some(tokio::spawn(async move {
            sleep(cycle_time).await;
            ctx_address.do_send(ProcessFinished);
        }));
    }

    pub async fn abort(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
            let part = self.part.as_mut().unwrap();
            part.failed = true;
        }
    }
}