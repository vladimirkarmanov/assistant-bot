use std::sync::Arc;

use teloxide::payloads::AnswerCallbackQuerySetters;
use teloxide::{Bot, prelude::Requester};

use crate::bot::DI;

use std::error::Error;
use teloxide::dispatching::UpdateHandler;
use teloxide::types::{Update, UpdateKind, User};

fn get_user(update: &Update) -> Option<&User> {
    match &update.kind {
        UpdateKind::Message(msg)
        | UpdateKind::EditedMessage(msg)
        | UpdateKind::ChannelPost(msg) => msg.from.as_ref(),
        UpdateKind::CallbackQuery(q) => Some(&q.from),
        UpdateKind::InlineQuery(q) => Some(&q.from),
        UpdateKind::MyChatMember(m) => Some(&m.from),
        UpdateKind::ChatMember(m) => Some(&m.from),
        _ => None,
    }
}

pub trait Middlewares {
    fn with_rate_limit(self) -> Self;
}

impl Middlewares for UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    fn with_rate_limit(self) -> Self {
        self.filter_async(|bot: Bot, update: Update, di: Arc<DI>| async move {
            if di.config.debug {
                return true;
            }

            let user_id = match get_user(&update) {
                Some(u) => u.id,
                None => return true,
            };

            match di.rate_limiter.get_user_current_limit(user_id.0).await {
                Ok(user_limit) => {
                    // Notify users only once, when they exceed the limit
                    if user_limit.should_notify_user {
                        match update.kind {
                                UpdateKind::CallbackQuery(q) => {
                                    let _ = bot.answer_callback_query(q.id)
                                        .text("Вы отправляете слишком много запросов. Подождите несколько секунд.")
                                       .show_alert(true)
                                       .await;
                                },
                                _ => {
                                    let _ = bot
                                        .send_message(user_id, "Вы отправляете слишком много запросов. Подождите несколько секунд.")
                                        .await;
                                }
                            }
                    }
                    user_limit.can_proceed_request
                }
                Err(_) => false,
            }
        })
    }
}
