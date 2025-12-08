use std::sync::Arc;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::{
    bot::DI,
    commands::MenuAction,
    handlers::{class::*, command::main_menu_handler, daily_practice::*},
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
            Some(MenuAction::DeductClass) => {
                list_classes_for_deduction_handler(bot, msg, di).await?;
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
            Some(MenuAction::DailyPracticeLog) => {
                daily_practice_log_menu_handler(bot, msg).await?;
            }
            Some(MenuAction::AddDailyPracticeEntry) => {
                bot.send_message(
                    msg.chat.id,
                    "Будет добавлена запись о вашей практике за сегодня.\nВведите количество минут:",
                )
                .await?;
                dialogue
                    .update(State::AddingDailyPracticeReceiveMinutes)
                    .await?;
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
        Some(("deduct_class", _)) => {
            deduct_class_callback_handler(bot.clone(), &q, di).await?;
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
