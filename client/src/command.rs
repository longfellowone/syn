use anyhow::{Context, Error, Result};
use chrono::Utc;
use chrono_tz::Canada;
use log::info;
use rand::Rng;
use reqwest::StatusCode;
use simplelog::{LevelFilter, WriteLogger};
use std::fs;
use std::fs::File;
use structopt::StructOpt;
use syn::PunchType;
use tokio::time::{sleep, Duration};

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short)]
    instant: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

// TODO: add flag for -variable and -fixed delay?
#[derive(StructOpt, Debug)]
pub enum Command {
    Punchin(Employee),
    Punchout(Employee),
}

// TODO: move location and randomization to Employee struct
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
    let time = Utc::now()
        .with_timezone(&Canada::Pacific)
        .format("%Y%m%d%H%M%S")
        .to_string();

    fs::create_dir_all("logs").ok();

    let file_name = format!("logs/{}.log", time);

    let logfile = File::create(file_name)?;

    let config = simplelog::ConfigBuilder::new()
        .set_time_to_local(true)
        .build();

    WriteLogger::init(LevelFilter::Info, config, logfile)?;

    info!("log file created");
    info!("checking health status...");

    check_app_status()
        .await
        .context("error: health check failed")?;

    info!("health check ok");
    info!("reading command line arguments...");

    let opt = Opt::from_args_safe()?;

    match opt.cmd {
        Command::Punchin(e) => punch(PunchType::In, opt.instant, e)
            .await
            .context("error: failed to punch in")?,
        Command::Punchout(e) => punch(PunchType::Out, opt.instant, e)
            .await
            .context("error: failed to punch out")?,
    }

    Ok(())
}

const BASE_URL: &str = "https://ozzelectric.synerionenterprise.com";

pub async fn punch(punch_type: PunchType, instant: bool, e: Employee) -> Result<()> {
    let client = syn::Client::new(BASE_URL, &e).await?;

    let name = e.username.clone();
    let punched = match &punch_type {
        PunchType::In => "in",
        PunchType::Out => "out",
    };

    let mut rng = rand::thread_rng();
    let mut delay = match &punch_type {
        PunchType::In => rng.gen_range(0..60) + 90,
        PunchType::Out => rng.gen_range(0..10) + 33,
    };

    if instant {
        delay = 0;
    }

    info!("starting punch {:?} for {:?}", &punched, &e);

    println!("{} seconds to punch {}", delay, punched);
    sleep(Duration::from_secs(delay)).await;

    info!("punching...");
    client.punch(punch_type).await?;
    info!("punched!");

    println!("Success! punched {}: {}", punched, name);
    sleep(Duration::from_secs(7200)).await;

    Ok(())
}

// TODO: use post request and filter names, check for enabled bool, use device ID from server
pub async fn check_app_status() -> Result<()> {
    let online = reqwest::get("https://syn-yp9ox.ondigitalocean.app/v1/syn").await?;

    match online.status() {
        StatusCode::OK => Ok(()),
        _ => Err(Error::msg("status code not 200")),
    }
}
