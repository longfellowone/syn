use anyhow::Context;
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

pub async fn punchin(e: Employee) -> anyhow::Result<()> {
    let _client = syn::Client::new(e.username, e.password).await?;

    Ok(())

    // Sleep for 0-3 minutes
    // sleep(Duration::from_secs(10)).await;s
}

pub fn punchout(e: Employee) {
    // let _client = match syn::Client::new(e.username, e.password).await {
    //     Ok(client) => client,
    //     Err(e) => {
    //         println!("failed to retrieve token: {}", e);
    //         sleep(Duration::from_secs(10)).await;
    //         process::exit(1)
    //     }
    // };
}
