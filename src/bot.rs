use dptree::case;
use sqlx::SqlitePool;
use teloxide::{
    dispatching::{HandlerExt, dialogue::InMemStorage},
    prelude::*,
    utils::command::BotCommands,
};

use crate::{
    config::Config,
    handlers::{
        class::{
            AddClassState, UpdateClassQuantityState, add_class_start_handler,
            charge_class_callback_handler, receive_name, receive_quantity,
            receive_quantity_handler, update_class_quantity_start_handler,
        },
        command::{Command, help_handler, main_menu_handler, message_handler, start_handler},
    },
};

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
                    dptree::case![AddClassState::ReceiveQuantity { name }]
                        .endpoint(receive_quantity),
                ),
        )
        .branch(
            Update::filter_callback_query()
                .filter(|q: CallbackQuery| {
                    q.data
                        .as_deref()
                        .is_some_and(|d| d.starts_with("charge_class:"))
                })
                .endpoint(charge_class_callback_handler),
        )
        .branch(
            Update::filter_callback_query()
                .filter(|q: CallbackQuery| {
                    q.data
                        .as_deref()
                        .is_some_and(|d| d.starts_with("update_quantity:"))
                })
                .enter_dialogue::<CallbackQuery, InMemStorage<UpdateClassQuantityState>, UpdateClassQuantityState>()
                .branch(case![UpdateClassQuantityState::GetClassId].endpoint(update_class_quantity_start_handler)),
        )
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, InMemStorage<UpdateClassQuantityState>, UpdateClassQuantityState>()
                .branch(dptree::case![UpdateClassQuantityState::ReceiveQuantity {class_id}].endpoint(receive_quantity_handler)),
        )
        .branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![
            InMemStorage::<AddClassState>::new(),
            InMemStorage::<UpdateClassQuantityState>::new(),
            db.clone()
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}
