use sqlx::{Pool, Sqlite};
use std::error::Error;
use teloxide::{payloads::SendMessageSetters, prelude::*, types::Me, utils::command::BotCommands};

use crate::{
    handlers::class::{list_classes_handler, class_settings_handler},
    keyboards,
    services::user::add_user,
};
use teloxide::{Bot, types::Message};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Помощь ℹ️")]
    Help,
    #[command(description = "Перезапустить бота ♻️")]
    Start,
    #[command(description = "Перейти в главное меню 🏠")]
    MainMenu,
}

pub async fn start_handler(
    bot: Bot,
    msg: Message,
    db: Pool<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(
        msg.chat.id,
        format!("Я семейный бот. Посмотри что я умею: /help"),
    )
    .await?;
    add_user(&db, msg.chat.id.0, msg.chat.username().unwrap_or("")).await?;
    Ok(())
}

pub async fn help_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn main_menu_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let keyboard = keyboards::make_main_menu_keyboard();
    bot.send_message(msg.chat.id, "Переход в главное меню")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
    db: Pool<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match text {
            "Списать занятие" => {
                list_classes_handler(bot, msg, db).await?;
            }
            "Занятия (настройка)" => {
                class_settings_handler(bot, msg).await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Команда не найдена!").await?;
            }
        }
    }
    Ok(())
}
