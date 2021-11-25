use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    server::run().await?;

    Ok(())
}
