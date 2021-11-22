mod command;

use std::process;
use anyhow::{Error, Result};
use reqwest::StatusCode;
use structopt::StructOpt;
use tokio::time::{sleep, Duration};
use command::{punchin, punchout, Command};

#[tokio::main]
async fn main() {
    if let Err(e) = check_app_status().await {
        println!("health check failed: {}", e);
        sleep(Duration::from_secs(10)).await;
        process::exit(1)
    }

    let cmd = match Command::from_args_safe() {
        Ok(cmd) => cmd,
        Err(e) => {
            println!("failed to parse args: {}", e);
            sleep(Duration::from_secs(10)).await;
            process::exit(1)
        }
    };

    match cmd {
        Command::Punchin(e) => punchin(e),
        Command::Punchout(e) => punchout(e),
    }
}


pub async fn check_app_status() -> Result<()> {
    let online = reqwest::get("https://syn-yp9ox.ondigitalocean.app/v1/covid").await?;

    match online.status() {
        StatusCode::OK => Ok(()),
        _ => Err(Error::msg("status code not 200")),
    }
}
