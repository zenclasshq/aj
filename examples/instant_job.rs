use actix_rt::time::sleep;
use actix_rt::System;
use std::time::Duration;

use aj::async_trait::async_trait;
use aj::mem::InMemory;
use aj::serde::{Deserialize, Serialize};
use aj::{get_now_as_ms, start_engine, AJ};
use aj::{Executable, JobBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintJob {
    pub number: i32,
}

#[async_trait]
impl Executable for PrintJob {
    type Output = ();

    async fn execute(&self) -> Self::Output {
        // Do your stuff here in async mode
        println!("Hello in background {} at {}", self.number, get_now_as_ms(),);
    }
}

fn run_job_instantly() {
    let job = JobBuilder::new(PrintJob { number: 1 }).build();
    AJ::add_job(job);
}

fn main() {
    start_engine();
    let backend = InMemory::default();
    AJ::register::<PrintJob>("print_job", backend);
    println!("Now is {}", get_now_as_ms());
    run_job_instantly();

    // Sleep 1 sec
    std::thread::spawn(|| {
        System::new().block_on(async {
            sleep(Duration::from_secs(1)).await;
        })
    })
    .join()
    .expect("Cannot spawn thread");
}
