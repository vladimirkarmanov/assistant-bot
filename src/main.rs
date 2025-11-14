mod bot;
mod config;
mod handlers;
mod keyboards;
mod services;

#[tokio::main]
async fn main() {
    bot::run().await;
}
