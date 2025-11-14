use dptree::case;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use teloxide::{
    dispatching::{HandlerExt, dialogue::InMemStorage},
    prelude::*,
    utils::command::BotCommands,
};

use crate::handlers::*;
use crate::{config::Config, handlers::Command};

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let config = Config::from_env();

    let mut sqlite_opts = sqlx::sqlite::SqliteConnectOptions::new();
    sqlite_opts = sqlite_opts.filename(&config.database.path);

    let db = SqlitePool::connect_with(sqlite_opts).await?;

    let bot = Bot::new(&config.bot_token);
    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Help].endpoint(help_handler))
                .branch(case![Command::Start].endpoint(start_handler))
                .branch(case![Command::MainMenu].endpoint(main_menu_handler)),
        )
        .branch(
            Update::filter_message().branch(
                dptree::filter(|msg: Message| {
                    msg.text().map(|t| t == "Добавить занятие").unwrap_or(false)
                })
                .enter_dialogue::<Message, InMemStorage<AddClassState>, AddClassState>()
                .branch(case![AddClassState::Idle].endpoint(add_class_start_handler)),
            ),
        )
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<AddClassState>, AddClassState>()
                .branch(dptree::case![AddClassState::ReceiveName].endpoint(receive_name))
                .branch(
                    dptree::case![AddClassState::ReceiveLimitCount { name }]
                        .endpoint(receive_limit_count),
                ),
        )
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![
            InMemStorage::<AddClassState>::new(),
            db.clone()
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
