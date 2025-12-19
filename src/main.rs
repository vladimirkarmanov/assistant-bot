mod bot;
mod commands;
mod config;
mod errors;
mod handlers;
mod keyboards;
mod middlewares;
mod rate_limiter;
mod repositories;
mod services;
mod state;
mod uow;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot::run().await?;
    Ok(())
}
