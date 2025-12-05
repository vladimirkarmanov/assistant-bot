use chrono::Weekday;

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
