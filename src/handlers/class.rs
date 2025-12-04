use std::{error::Error, sync::Arc};
use teloxide::{
    dispatching::dialogue::InMemStorage, payloads::SendMessageSetters, prelude::*, types::ParseMode,
};

use crate::{
    bot::DI,
    commands::MenuAction,
    handlers::command::main_menu_handler,
    keyboards::{self, MainMenuButton},
    services::class::*,
    state::State,
};

pub async fn idle_message_handler(
    bot: Bot,
    dialogue: Dialogue<State, InMemStorage<State>>,
    msg: Message,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match MenuAction::parse(text) {
            Some(MenuAction::Classes) => {
                classes_menu_handler(bot, msg).await?;
            }
            Some(MenuAction::AddClass) => {
                bot.send_message(msg.chat.id, "Введите назввание:").await?;
                dialogue.update(State::AddingClassReceiveName).await?;
            }
            Some(MenuAction::ChargeClass) => {
                list_classes_for_charging_handler(bot, msg, di).await?;
            }
            Some(MenuAction::ClassSettings) => {
                class_settings_handler(bot, msg).await?;
            }
            Some(MenuAction::ListClasses) => {
                list_classes_handler(bot, msg, di).await?;
            }
            Some(MenuAction::ClassesDeductionHistory) => {
                list_classes_deduction_history_handler(bot, msg, di).await?;
            }
            Some(MenuAction::UpdateQuantity) => {
                update_quantity_handler(bot, msg, di).await?;
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

pub async fn idle_callback_handler(
    bot: Bot,
    dialogue: Dialogue<State, InMemStorage<State>>,
    q: CallbackQuery,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let Some(data) = q.data.as_deref() else {
        return Ok(());
    };

    match data.split_once(':') {
        Some(("charge_class", _)) => {
            charge_class_callback_handler(bot.clone(), &q, di).await?;
        }
        Some(("update_quantity", _)) => {
            update_class_quantity_callback_handler(bot.clone(), &q, &dialogue).await?;
        }
        Some(("class_deduction_history", _)) => {
            list_classes_deduction_history_callback_handler(bot.clone(), &q, di).await?;
        }
        _ => {}
    }

    Ok(())
}

pub async fn receive_name(
    bot: Bot,
    dialogue: Dialogue<State, InMemStorage<State>>,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "Введите количество занятий")
                .await?;
            dialogue
                .update(State::AddingClassReceiveQuantity { name: text.into() })
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
    dialogue: Dialogue<State, InMemStorage<State>>,
    name: String,
    msg: Message,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(quantity)) => {
            let class = match add_class(di.db_pool.clone(), name, quantity, msg.chat.id.0).await {
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

pub async fn receive_quantity_handler(
    bot: Bot,
    msg: Message,
    dialogue: Dialogue<State, InMemStorage<State>>,
    di: Arc<DI>,
    class_id: i64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(quantity)) => {
            let output =
                match update_class_quantity(di.db_pool.clone(), class_id, msg.chat.id.0, quantity)
                    .await
                {
                    Ok(class) => {
                        format!(
                            "✅ Занятие {name} успешно обновлено! Остаток: {quantity}",
                            name = class.name,
                            quantity = class.quantity
                        )
                    }
                    Err(err) => err.to_string(),
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

async fn classes_menu_handler(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let buttons = vec![
        MainMenuButton {
            text: MenuAction::ChargeClass.label().to_string(),
        },
        MainMenuButton {
            text: MenuAction::ClassSettings.label().to_string(),
        },
        MainMenuButton {
            text: MenuAction::ListClasses.label().to_string(),
        },
        MainMenuButton {
            text: MenuAction::ClassesDeductionHistory.label().to_string(),
        },
        MainMenuButton {
            text: MenuAction::MainMenu.label().to_string(),
        },
    ];
    bot.send_message(msg.chat.id, "Переход в раздел Занятия")
        .reply_markup(keyboards::make_main_menu_keyboard(buttons, 2))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn class_settings_handler(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let buttons = vec![
        MainMenuButton {
            text: MenuAction::AddClass.label().to_string(),
        },
        MainMenuButton {
            text: MenuAction::UpdateQuantity.label().to_string(),
        },
        MainMenuButton {
            text: MenuAction::MainMenu.label().to_string(),
        },
    ];
    bot.send_message(msg.chat.id, "Настройки занятий")
        .reply_markup(keyboards::make_main_menu_keyboard(buttons, 2))
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn list_classes_handler(
    bot: Bot,
    msg: Message,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let classes = get_classes_by_user_id(di.db_pool.clone(), msg.chat.id.0).await?;
    if classes.is_empty() {
        bot.send_message(msg.chat.id, "У вас нет добавленных занятий")
            .parse_mode(ParseMode::Html)
            .await?;
        return Ok(());
    }

    let formatted_classes: Vec<String> = classes.iter().map(|s| s.to_string()).collect();
    let output = formatted_classes.join("\n");
    bot.send_message(msg.chat.id, output)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn list_classes_for_charging_handler(
    bot: Bot,
    msg: Message,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let classes = get_classes_by_user_id(di.db_pool.clone(), msg.chat.id.0).await?;
    if classes.is_empty() {
        bot.send_message(msg.chat.id, "У вас нет занятий для списания")
            .parse_mode(ParseMode::Html)
            .await?;
        return Ok(());
    }

    let keyboard = keyboards::make_class_list_inline_keyboard(classes, 2, "charge_class:");
    let output = "Выберите занятие для списания";
    bot.send_message(msg.chat.id, output)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn charge_class_callback_handler(
    bot: Bot,
    q: &CallbackQuery,
    di: Arc<DI>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(ref data) = q.data
        && let Some((_, id)) = data.split_once(':')
    {
        let class_id: i64 = id.parse()?;
        let telegram_user_id: i64 = q.from.id.0.try_into().unwrap();
        bot.answer_callback_query(q.id.clone()).await?;

        let output = match charge_class(di.db_pool.clone(), class_id, telegram_user_id).await {
            Ok(class) => {
                add_class_deduction_history(di.db_pool.clone(), class_id, telegram_user_id).await?;
                format!(
                    "✅ Занятие {name} успешно списано! Остаток: {quantity}",
                    name = class.name,
                    quantity = class.quantity
                )
            }
            Err(err) => err.to_string(),
        };

        if let Some(message) = q.regular_message() {
            bot.edit_message_text(message.chat.id, message.id, output)
                .await?;
        }
    }

    Ok(())
}

async fn update_quantity_handler(
    bot: Bot,
    msg: Message,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let classes = get_classes_by_user_id(di.db_pool.clone(), msg.chat.id.0).await?;
    let keyboard = keyboards::make_class_list_inline_keyboard(classes, 2, "update_quantity:");
    let output = "Выберите занятие для обновления";
    bot.send_message(msg.chat.id, output)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn update_class_quantity_callback_handler(
    bot: Bot,
    q: &CallbackQuery,
    dialogue: &Dialogue<State, InMemStorage<State>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.answer_callback_query(q.id.clone()).await?;

    if let Some(ref data) = q.data
        && let Some((_, id)) = data.split_once(':')
    {
        let class_id: i64 = id.parse()?;
        dialogue
            .update(State::UpdatingClassReceiveQuantity { class_id })
            .await?;

        if let Some(message) = q.regular_message() {
            bot.edit_message_text(message.chat.id, message.id, "Введите количество:")
                .await?;
        }
    } else {
        bot.send_message(q.from.id, "Ошибка").await?;
    }

    Ok(())
}

async fn list_classes_deduction_history_handler(
    bot: Bot,
    msg: Message,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let classes = get_classes_by_user_id(di.db_pool.clone(), msg.chat.id.0).await?;
    if classes.is_empty() {
        bot.send_message(msg.chat.id, "У вас нет добавленных занятий")
            .parse_mode(ParseMode::Html)
            .await?;
        return Ok(());
    }

    let keyboard =
        keyboards::make_class_list_inline_keyboard(classes, 2, "class_deduction_history:");
    let output = "Выберите занятие для просмотра истории списаний";
    bot.send_message(msg.chat.id, output)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await?;
    Ok(())
}

async fn list_classes_deduction_history_callback_handler(
    bot: Bot,
    q: &CallbackQuery,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    bot.answer_callback_query(q.id.clone()).await?;
    let Some(message) = q.regular_message() else {
        return Ok(());
    };

    if let Some(ref data) = q.data
        && let Some((_, id)) = data.split_once(':')
    {
        let class_id: i64 = id.parse()?;
        let telegram_user_id: i64 = q.from.id.0.try_into().unwrap();

        let histories =
            get_class_deduction_histories(di.db_pool.clone(), class_id, telegram_user_id).await?;
        if histories.is_empty() {
            bot.edit_message_text(message.chat.id, message.id, "История списаний пуста")
                .await?;
            return Ok(());
        }

        let formatted_histories: Vec<String> = histories.iter().map(|s| s.to_string()).collect();
        let output = formatted_histories.join("\n");
        bot.edit_message_text(message.chat.id, message.id, output)
            .await?;
    } else {
        bot.edit_message_text(message.chat.id, message.id, "Произошла ошибка")
            .await?;
    }
    Ok(())
}
