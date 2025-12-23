use chrono::Weekday;

use teloxide::types::{Update, UpdateKind, User};

pub fn get_russian_weekday_name(weekday: Weekday, short_form: bool) -> &'static str {
    if short_form {
        return match weekday {
            Weekday::Mon => "Пн",
            Weekday::Tue => "Вт",
            Weekday::Wed => "Ср",
            Weekday::Thu => "Чт",
            Weekday::Fri => "Пт",
            Weekday::Sat => "Сб",
            Weekday::Sun => "Вс",
        };
    }

    match weekday {
        Weekday::Mon => "Понедельник",
        Weekday::Tue => "Вторник",
        Weekday::Wed => "Среда",
        Weekday::Thu => "Четверг",
        Weekday::Fri => "Пятница",
        Weekday::Sat => "Суббота",
        Weekday::Sun => "Воскресенье",
    }
}

pub fn get_user(update: &Update) -> Option<&User> {
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
