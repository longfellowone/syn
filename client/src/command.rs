use anyhow::{Context, Error, Result};
use rand::Rng;
use reqwest::StatusCode;
use structopt::StructOpt;
use syn::PunchType;
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
    #[structopt(short)]
    new_device: bool,
}

impl From<&Employee> for syn::Employee {
    fn from(e: &Employee) -> Self {
        Self {
            username: e.username.clone(),
            password: e.password.clone(),
            device_unique_id: e.device_unique_id.clone(),
            new_device: e.new_device,
        }
    }
}

pub async fn run() -> Result<()> {
    check_app_status()
        .await
        .context("error: health check failed")?;

    let cmd = Command::from_args_safe()?;

    match cmd {
        Command::Punchin(e) => punch(PunchType::In, e)
            .await
            .context("error: failed to punch in")?,
        Command::Punchout(e) => punch(PunchType::Out, e)
            .await
            .context("error: failed to punch out")?,
    }

    Ok(())
}

const BASE_URL: &str = "https://ozzelectric.synerionenterprise.com";

pub async fn punch(punch_type: PunchType, e: Employee) -> Result<()> {
    let client = syn::Client::new(BASE_URL, &e).await?;

    let name = e.username;
    let punched = match &punch_type {
        PunchType::In => "in",
        PunchType::Out => "out",
    };

    let mut rng = rand::thread_rng();
    let random_delay = match &punch_type {
        PunchType::In => rng.gen_range(0..60) + 90,
        PunchType::Out => rng.gen_range(0..10) + 31,
    };

    println!("{} seconds to punch {}", random_delay, punched);
    sleep(Duration::from_secs(random_delay)).await;

    client.punch(punch_type).await?;

    println!("Success! punched {}: {}", punched, name);
    sleep(Duration::from_secs(14400)).await;

    Ok(())
}

pub async fn check_app_status() -> Result<()> {
    let online = reqwest::get("https://syn-yp9ox.ondigitalocean.app/v1/syn").await?;

    match online.status() {
        StatusCode::OK => Ok(()),
        _ => Err(Error::msg("status code not 200")),
    }
}
