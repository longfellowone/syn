use anyhow::Result;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    server::run(listener).await?;

    Ok(())
}
