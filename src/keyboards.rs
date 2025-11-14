use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn make_main_menu_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    let main_menu_commands = [
        "Списать занятие",
        "Занятия (настройка)"
    ];

    for commands in main_menu_commands.chunks(2) {
        let row = commands
            .iter()
            .map(|&command| KeyboardButton::new(command.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard).persistent()
}

pub fn class_settings_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    let main_menu_commands = [
        "Добавить занятие",
        "Начислить количество"
    ];

    for commands in main_menu_commands.chunks(2) {
        let row = commands
            .iter()
            .map(|&command| KeyboardButton::new(command.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard).persistent()
}