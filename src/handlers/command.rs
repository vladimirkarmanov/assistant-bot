use sqlx::{Pool, Sqlite};
use std::{error::Error, sync::Arc};
use teloxide::{
    dispatching::dialogue::{InMemStorage, Storage},
    payloads::SendMessageSetters,
    prelude::*,
    utils::command::BotCommands,
};

use crate::{handlers::class::*, keyboards, services::user::*};
use teloxide::{Bot, types::Message};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Перезапустить бота ♻️")]
    Start,
    #[command(description = "Перейти в главное меню 🏠")]
    MainMenu,
    #[command(description = "Отменить операцию ❌")]
    CancelOperation,
    #[command(description = "Помощь ℹ️")]
    Help,
}

pub async fn start_handler(
    bot: Bot,
    msg: Message,
    db: Pool<Sqlite>,
) -> anyhow::Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(
        msg.chat.id,
        format!("Я бот помощник. Посмотри что я умею: /help"),
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

pub async fn cancel_handler(
    bot: Bot,
    msg: Message,
    add_storage: Arc<InMemStorage<AddClassState>>,
    upd_storage: Arc<InMemStorage<UpdateClassQuantityState>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = msg.chat.id;

    let _ = add_storage.remove_dialogue(chat_id).await;
    let _ = upd_storage.remove_dialogue(chat_id).await;

    bot.send_message(chat_id, "Отмена успешна").await?;
    Ok(())
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    db: Pool<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match text {
            "Списать занятие" => {
                list_classes_for_charging_handler(bot, msg, db).await?;
            }
            "Занятия (настройка)" => {
                class_settings_handler(bot, msg).await?;
            }
            "Список занятий" => {
                list_classes_handler(bot, msg, db).await?;
            }
            "Обновить количество" => {
                update_quantity_handler(bot, msg, db).await?;
            }
            "Главное меню" => {
                main_menu_handler(bot, msg).await?;
            }
            _ => {
                bot.send_message(msg.chat.id, "Команда не найдена!").await?;
            }
        }
    }
    Ok(())
}
