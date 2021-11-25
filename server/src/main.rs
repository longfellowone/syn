use anyhow::Result;
use server::configuration::Configuration;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<()> {
    // TODO: move to App
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    let config = Configuration::new()?;

    // TODO: let app = App::new(config) -> app.run()
    server::run(listener)?.await?;

    Ok(())
}
