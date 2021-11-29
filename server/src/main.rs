use anyhow::Result;
use server::configuration::Configuration;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<()> {
    // TODO: move to App
    let config = Configuration::new()?;
    let listener = TcpListener::bind(config.server.address())?;

    println!("Starting server on {}...", config.server.address());

    // TODO: let app = App::new(config) -> app.run()
    server::run(listener)?.await?;

    Ok(())
}
