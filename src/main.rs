mod bot;
mod config;
mod handlers;
mod keyboards;
mod services;
mod error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot::run().await?;
    Ok(())
}
