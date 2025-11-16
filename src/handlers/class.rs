use sqlx::{Pool, Sqlite};
use std::error::Error;
use teloxide::{
    dispatching::dialogue::InMemStorage, payloads::SendMessageSetters, prelude::*, types::ParseMode,
};

use crate::{
    keyboards,
    services::class::{add_class, charge_class, get_classes_by_user_id, update_class_quantity},
};

#[derive(Clone, Default)]
pub enum AddClassState {
    #[default]
    Idle,
    ReceiveName,
    ReceiveQuantity {
        name: String,
    },
}

pub async fn class_settings_handler(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Настройки занятий")
        .reply_markup(keyboards::class_settings_keyboard())
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn add_class_start_handler(
    bot: Bot,
    dialogue: Dialogue<AddClassState, InMemStorage<AddClassState>>,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.send_message(msg.chat.id, "Введите назввание:").await?;
    dialogue.update(AddClassState::ReceiveName).await?;
    Ok(())
}

pub async fn receive_name(
    bot: Bot,
    dialogue: Dialogue<AddClassState, InMemStorage<AddClassState>>,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Введите количество занятий")
                .await?;
            dialogue
                .update(AddClassState::ReceiveQuantity { name: text.into() })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Отправьте текст").await?;
        }
    }

    Ok(())
}

pub async fn receive_quantity(
    bot: Bot,
    dialogue: Dialogue<AddClassState, InMemStorage<AddClassState>>,
    name: String,
    msg: Message,
    db: Pool<Sqlite>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(quantity)) => {
            let class = match add_class(&db, name, quantity, msg.chat.id.0).await {
                Ok(_) => "✅ Занятие успешно добавлено!".to_string(),
                Err(err) => err.to_string(),
            };
            bot.send_message(msg.chat.id, class).await?;
            dialogue.exit().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Отправьте число").await?;
        }
    }

    Ok(())
}

pub async fn list_classes_handler(
    bot: Bot,
    msg: Message,
    db: Pool<Sqlite>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let classes = get_classes_by_user_id(&db, msg.chat.id.0).await?;
    let keyboard = keyboards::make_class_list_keyboard(classes, 2, "charge_class:");
    let output = "Выберите занятие для списания";
    bot.send_message(msg.chat.id, output)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

pub async fn charge_class_callback_handler(
    bot: Bot,
    q: CallbackQuery,
    db: Pool<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(ref data) = q.data {
        if let Some((_, id)) = data.split_once(':') {
            let class_id: i64 = id.parse()?;
            bot.answer_callback_query(q.id.clone()).await?;

            let output: String;
            match charge_class(&db, class_id, q.from.id.0.try_into().unwrap()).await {
                Ok(class) => {
                    output = format!(
                        "✅ Занятие {name} успешно списано! Остаток: {quantity}",
                        name = class.name,
                        quantity = class.quantity
                    );
                }
                Err(err) => {
                    output = err.to_string();
                }
            };

            // Edit text of the message to which the buttons were attached
            if let Some(message) = q.regular_message() {
                bot.edit_message_text(message.chat.id, message.id, output)
                    .await?;
            }
        }
    }

    Ok(())
}

pub async fn update_quantity_handler(
    bot: Bot,
    msg: Message,
    db: Pool<Sqlite>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let classes = get_classes_by_user_id(&db, msg.chat.id.0).await?;
    let keyboard = keyboards::make_class_list_keyboard(classes, 2, "update_quantity:");
    let output = "Выберите занятие для обновления";
    bot.send_message(msg.chat.id, output)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

#[derive(Clone, Default)]
pub enum UpdateClassQuantityState {
    #[default]
    GetClassId,
    ReceiveQuantity {
        class_id: i64,
    },
}

pub async fn update_class_quantity_start_handler(
    bot: Bot,
    q: CallbackQuery,
    dialogue: Dialogue<UpdateClassQuantityState, InMemStorage<UpdateClassQuantityState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.answer_callback_query(q.id.clone()).await?;

    if let Some(ref data) = q.data {
        if let Some((_, id)) = data.split_once(':') {
            let class_id: i64 = id.parse()?;
            dialogue
                .update(UpdateClassQuantityState::ReceiveQuantity { class_id: class_id })
                .await?;

            bot.send_message(q.from.id, "Введите количество:").await?;
        } else {
            bot.send_message(q.from.id, "Ошибка").await?;
        }
    }

    Ok(())
}

pub async fn receive_quantity_handler(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<UpdateClassQuantityState, InMemStorage<UpdateClassQuantityState>>,
    db: Pool<Sqlite>,
    class_id: i64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(quantity)) => {
            let output: String;
            match update_class_quantity(&db, class_id, msg.chat.id.0, quantity).await {
                Ok(class) => {
                    output = format!(
                        "✅ Занятие {name} успешно обновлено! Остаток: {quantity}",
                        name = class.name,
                        quantity = class.quantity
                    );
                }
                Err(err) => {
                    output = err.to_string();
                }
            };
            dialogue.exit().await?;
            bot.send_message(msg.chat.id, output)
                .parse_mode(ParseMode::Html)
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Отправьте число").await?;
        }
    }
    Ok(())
}
