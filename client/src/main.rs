mod command;

use anyhow::{Error, Result};
use command::{punchin, punchout, Command};
use reqwest::StatusCode;
use std::process;
use structopt::StructOpt;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let cmd = match Command::from_args_safe() {
        Ok(cmd) => cmd,
        Err(e) => {
            println!("failed to parse args: {}", e);
            sleep(Duration::from_secs(10)).await;
            process::exit(1)
        }
    };

    if let Err(e) = check_app_status().await {
        println!("health check failed: {}", e);
        sleep(Duration::from_secs(10)).await;
        process::exit(1)
    }

    match cmd {
        Command::Punchin(e) => {
            if let Err(e) = punchin(e).await {
                println!("failed to punch in: {}", e);
                sleep(Duration::from_secs(10)).await;
                process::exit(1)
            }
        }
        Command::Punchout(e) => punchout(e),
    }
}

pub async fn check_app_status() -> Result<()> {
    let online = reqwest::get("https://syn-yp9ox.ondigitalocean.app/v1/syn").await?;

    match online.status() {
        StatusCode::OK => Ok(()),
        _ => Err(Error::msg("status code not 200")),
    }
}
