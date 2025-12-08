use std::sync::Arc;

use teloxide::{
    Bot,
    dispatching::dialogue::InMemStorage,
    payloads::SendMessageSetters,
    prelude::{Dialogue, Requester},
    types::Message,
};

use crate::{
    bot::DI,
    commands::MenuAction,
    keyboards::{self, MainMenuButton},
    services::daily_practice_log::add_daily_practice_entry,
    state::State,
};

pub async fn daily_practice_log_menu_handler(
    bot: Bot,
    msg: Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let buttons = vec![
        MainMenuButton {
            text: MenuAction::AddDailyPracticeEntry.label().to_string(),
        },
        MainMenuButton {
            text: MenuAction::MainMenu.label().to_string(),
        },
    ];
    bot.send_message(msg.chat.id, "Переход в раздел Дневник практик")
        .reply_markup(keyboards::make_main_menu_keyboard(buttons, 2))
        .await?;
    Ok(())
}

pub async fn receive_minutes(
    bot: Bot,
    dialogue: Dialogue<State, InMemStorage<State>>,
    msg: Message,
    di: Arc<DI>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.text().map(|text| text.parse::<u16>()) {
        Some(Ok(minutes)) => {
            let class =
                match add_daily_practice_entry(di.db_pool.clone(), minutes, msg.chat.id.0).await {
                    Ok(_) => "✅ Запись успешно добавлена!".to_string(),
                    Err(err) => err.to_string(),
                };
            bot.send_message(msg.chat.id, class).await?;
            dialogue.exit().await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Отправьте целое число")
                .await?;
        }
    }

    Ok(())
}
