use anyhow::{Context, Error, Result};
use reqwest::StatusCode;
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
    #[structopt(short)]
    device_unique_id: String,
}

impl From<Employee> for syn::Employee {
    fn from(e: Employee) -> Self {
        Self {
            username: e.username,
            password: e.password,
            device_unique_id: e.device_unique_id,
        }
    }
}

pub async fn run() -> Result<()> {
    check_app_status()
        .await
        .context("error: health check failed")?;

    let cmd = Command::from_args_safe()?;

    match cmd {
        Command::Punchin(e) => punchin(e).await.context("error: failed to punch in")?,
        Command::Punchout(e) => punchout(e).await.context("error: failed to punch out")?,
    }

    Ok(())
}

const BASE_URL: &str = "https://ozzelectric.synerionenterprise.com";

pub async fn punchin(e: Employee) -> Result<()> {
    let client = syn::Client::new(BASE_URL, e).await?;

    client.punchin().await?;

    // Sleep for 0-3 minutes before punch
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

pub async fn punchout(e: Employee) -> Result<()> {
    let _client = syn::Client::new(BASE_URL, e).await?;

    // Sleep for 0-3 minutes before punch
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

pub async fn check_app_status() -> Result<()> {
    let online = reqwest::get("https://syn-yp9ox.ondigitalocean.app/v1/syn").await?;

    match online.status() {
        StatusCode::OK => Ok(()),
        _ => Err(Error::msg("status code not 200")),
    }
}
