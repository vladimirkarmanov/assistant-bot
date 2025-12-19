use std::sync::Arc;

use teloxide::{Bot, prelude::Requester, types::Message};

use crate::bot::DI;

pub async fn throttling_middleware(msg: Message, bot: Bot, di: Arc<DI>) -> bool {
    let user_id = match msg.from {
        Some(u) => u.id.0 as i64,
        None => {
            return true;
        }
    };

    match di.rate_limiter.get_user_current_limit(user_id).await {
        Ok(user_limit) => {
            // Notify users only once, when they exceed the limit
            if user_limit.should_notify_user {
                let _ = bot
                    .send_message(
                        msg.chat.id,
                        "Вы отправляете слишком много запросов. Подождите несколько секунд.",
                    )
                    .await;
            }
            user_limit.can_proceed_request
        }
        Err(_) => false,
    }
}
