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
    handlers::class::*,
    keyboards::{self, MainMenuButton},
    services::user::*,
};
use teloxide::{Bot, types::Message};

pub async fn start_handler(
    bot: Bot,
    msg: Message,
    db: Arc<Pool<Sqlite>>,
) -> anyhow::Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Я бот помощник. Посмотри что я умею: /help")
        .await?;
    add_user(db.clone(), msg.chat.id.0, msg.chat.username().unwrap_or("")).await?;
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
    add_storage: Arc<InMemStorage<AddClassState>>,
    upd_storage: Arc<InMemStorage<UpdateClassQuantityState>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let chat_id = msg.chat.id;

    let _ = add_storage.remove_dialogue(chat_id).await;
    let _ = upd_storage.remove_dialogue(chat_id).await;

    bot.send_message(chat_id, "Отмена операции").await?;
    Ok(())
}

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    db: Arc<Pool<Sqlite>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match MenuAction::parse(text) {
            Some(MenuAction::Classes) => {
                classes_menu_handler(bot, msg).await?;
            }
            Some(MenuAction::AddClass) => {}
            Some(MenuAction::ChargeClass) => {
                list_classes_for_charging_handler(bot, msg, db).await?;
            }
            Some(MenuAction::ClassSettings) => {
                class_settings_handler(bot, msg).await?;
            }
            Some(MenuAction::ListClasses) => {
                list_classes_handler(bot, msg, db).await?;
            }
            Some(MenuAction::UpdateQuantity) => {
                update_quantity_handler(bot, msg, db).await?;
            }
            Some(MenuAction::MainMenu) => {
                main_menu_handler(bot, msg).await?;
            }
            None => {
                bot.send_message(msg.chat.id, "Команда не найдена!").await?;
            }
        }
    }
    Ok(())
}
