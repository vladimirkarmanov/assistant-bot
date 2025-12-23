use std::sync::Arc;

use teloxide::payloads::AnswerCallbackQuerySetters;
use teloxide::{Bot, prelude::Requester};

use crate::bot::DI;

use crate::utils;
use std::error::Error;
use teloxide::dispatching::UpdateHandler;
use teloxide::types::{Update, UpdateKind};

pub trait Middlewares {
    fn with_rate_limit(self) -> Self;
}

impl Middlewares for UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    fn with_rate_limit(self) -> Self {
        self.filter_async(|bot: Bot, update: Update, di: Arc<DI>| async move {
            if di.config.debug {
                return true;
            }

            let user_id = match utils::get_user(&update) {
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
