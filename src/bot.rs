use std::{sync::Arc, time::Duration};

use crate::{
    commands::Command,
    config::Config,
    handlers::{
        class::*,
        command::*,
        common::{idle_callback_handler, idle_message_handler},
        daily_practice_log::receive_minutes,
    },
    middlewares::throttling_middleware,
    rate_limiter::RedisRateLimiter,
    state::State,
};
use dptree::case;
use sqlx::SqlitePool;
use teloxide::{
    dispatching::{HandlerExt, dialogue::InMemStorage},
    prelude::*,
    utils::command::BotCommands,
};
pub struct DI {
    pub config: Config,
    pub db_pool: Arc<SqlitePool>,
    pub rate_limiter: Arc<RedisRateLimiter>,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let config = Config::from_env();

    let mut sqlite_opts = sqlx::sqlite::SqliteConnectOptions::new();
    sqlite_opts = sqlite_opts.filename(&config.database.path);
    let db_pool = SqlitePool::connect_with(sqlite_opts).await?;

    let rate_limiter = RedisRateLimiter::new(
        &config.redis.url,
        config.redis.rate_limit,
        Duration::from_secs(config.redis.rate_interval_secs),
        "assistant-bot",
    )
    .await?;

    let bot = Bot::new(&config.bot_token);
    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    let di = Arc::new(DI {
        config,
        db_pool: Arc::new(db_pool),
        rate_limiter: Arc::new(rate_limiter),
    });

    let handler = dptree::entry()
        .chain(Update::filter_message().filter_async(throttling_middleware))
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Help].endpoint(help_handler))
                .branch(case![Command::Start].endpoint(start_handler))
                .branch(case![Command::MainMenu].endpoint(main_menu_handler))
                .branch(case![Command::CancelOperation].endpoint(cancel_handler)),
        )
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .branch(case![State::Idle].endpoint(idle_message_handler))
                .branch(case![State::AddingClassReceiveName].endpoint(receive_name))
                .branch(
                    case![State::AddingClassReceiveQuantity { name }].endpoint(receive_quantity),
                )
                .branch(
                    case![State::UpdatingClassReceiveQuantity { class_id }]
                        .endpoint(receive_quantity_handler),
                )
                .branch(case![State::AddingDailyPracticeReceiveMinutes].endpoint(receive_minutes)),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
                .branch(case![State::Idle].endpoint(idle_callback_handler)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), di])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
