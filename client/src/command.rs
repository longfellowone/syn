use anyhow::{Context, Result};
use std::process;
use structopt::StructOpt;
use tokio::time::{sleep, Duration};

#[derive(StructOpt)]
pub enum Command {
    Punchin(Employee),
    Punchout(Employee),
}

#[derive(StructOpt, Debug)]
pub struct Employee {
    #[structopt(short)]
    username: String,
    #[structopt(short)]
    password: String,
}

pub async fn punchin(e: Employee) -> Result<()> {
    let _client = syn::Client::new(e.username, e.password).await?;

    // Sleep for 0-3 minutes
    // sleep(Duration::from_secs(10)).await;s

    Ok(())
}

pub async fn punchout(e: Employee) -> Result<()> {
    let _client = syn::Client::new(e.username, e.password).await?;

    Ok(())
}
