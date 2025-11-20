mod bot;
mod commands;
mod config;
mod errors;
mod handlers;
mod keyboards;
mod services;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot::run().await?;
    Ok(())
}
