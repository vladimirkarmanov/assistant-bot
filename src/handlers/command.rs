use sqlx::{Pool, Sqlite};
use std::{error::Error, sync::Arc};
use teloxide::{
    dispatching::dialogue::{InMemStorage, Storage},
    payloads::SendMessageSetters,
    prelude::*,
    utils::command::BotCommands,
};

use crate::{
    commands::{Command, MenuAction},
    keyboards::{self, MainMenuButton},
    services::user::*,
    state::State,
};
use teloxide::{Bot, types::Message};

pub async fn start_handler(
    bot: Bot,
    msg: Message,
    db_pool: Arc<Pool<Sqlite>>,
) -> anyhow::Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Я бот помощник. Посмотри что я умею: /help")
        .await?;
    add_user(db_pool.clone(), msg.chat.id.0, msg.chat.username().unwrap_or("")).await?;
    Ok(())
}

pub async fn help_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn main_menu_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let buttons = vec![MainMenuButton {
        text: MenuAction::Classes.label().to_string(),
    }];
    let keyboard = keyboards::make_main_menu_keyboard(buttons, 2);
    bot.send_message(msg.chat.id, "Переход в главное меню")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn cancel_handler(
    bot: Bot,
    msg: Message,
    storage: Arc<InMemStorage<State>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = msg.chat.id;
    let _ = storage.remove_dialogue(chat_id).await;
    bot.send_message(chat_id, "Отмена операции").await?;
    Ok(())
}
