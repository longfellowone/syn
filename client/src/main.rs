// #![allow(dead_code, unused_imports, unused_variables)]
mod command;

use crate::command::run;
use log::error;
use std::process;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{:?}", e);
        error!("{:?}", e);

        sleep(Duration::from_secs(600)).await;
        process::exit(1)
    }
}
