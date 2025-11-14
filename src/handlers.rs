use sqlx::{Pool, Sqlite};
use std::error::Error;
use teloxide::{
    dispatching::dialogue::InMemStorage, payloads::SendMessageSetters, prelude::*, types::Me,
    utils::command::BotCommands,
};

use crate::keyboards;
use teloxide::{Bot, types::Message, types::ParseMode};


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

pub async fn start_handler(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(
        msg.chat.id,
        format!("Я семейный бот. Посмотри что я умею: /help"),
    )
    .await?;
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
                // handle_write_off_class(bot, msg, db.clone()).await?;
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

pub async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(ref version) = q.data {
        let text = format!("You chose: {version}");

        bot.answer_callback_query(q.id.clone()).await?;

        // Edit text of the message to which the buttons were attached
        if let Some(message) = q.regular_message() {
            bot.edit_message_text(message.chat.id, message.id, text)
                .await?;
        }

        log::info!("You chose: {version}");
    }

    Ok(())
}

async fn class_settings_handler(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let keyboard = keyboards::class_settings_keyboard();
    let output = "Настройки занятий";
    // let output = match add_reminder(&db).await {
    //     Ok(_) => "✅ Напоминание успешно добавлено!".to_string(),
    //     Err(err) => err.to_string(),
    // };
    bot.send_message(msg.chat.id, output)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

type MyDialogue = Dialogue<AddClassState, InMemStorage<AddClassState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum AddClassState {
    #[default]
    Idle,
    ReceiveName,
    ReceiveLimitCount {
        name: String,
    },
}

pub async fn add_class_start_handler(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Введите назввание:").await?;
    dialogue.update(AddClassState::ReceiveName).await?;
    Ok(())
}

pub async fn receive_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Введите количество занятий")
                .await?;
            dialogue
                .update(AddClassState::ReceiveLimitCount { name: text.into() })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Отправьте текст").await?;
        }
    }

    Ok(())
}

pub async fn receive_limit_count(
    bot: Bot,
    dialogue: MyDialogue,
    name: String,
    msg: Message,
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(limit_count)) => {
            let report = format!("Name: {name}\nLimit count: {limit_count}");
            bot.send_message(msg.chat.id, report).await?;
            dialogue.exit().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Отправьте число").await?;
        }
    }

    Ok(())
}
