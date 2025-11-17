use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};

use crate::services::class::Class;

pub fn make_main_menu_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    let main_menu_commands = ["Списать занятие", "Занятия (настройка)", "Список занятий"];

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

    let main_menu_commands = ["Добавить занятие", "Обновить количество", "Главное меню"];

    for commands in main_menu_commands.chunks(2) {
        let row = commands
            .iter()
            .map(|&command| KeyboardButton::new(command.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard).persistent()
}

#[derive(Debug)]
struct InlineButton {
    text: String,
    callback_data: String,
}

fn make_inline_keyboard(buttons: Vec<InlineButton>, row_size: usize) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for buttons in buttons.chunks(row_size) {
        let row = buttons
            .iter()
            .map(|button| {
                InlineKeyboardButton::callback(
                    button.text.to_owned(),
                    button.callback_data.to_owned(),
                )
            })
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

pub fn make_class_list_keyboard(
    elements: Vec<Class>,
    row_size: usize,
    callback_data_prefix: &str,
) -> InlineKeyboardMarkup {
    let buttons = elements
        .into_iter()
        .map(|element| InlineButton {
            text: format!("{} ({})", element.name, element.quantity),
            callback_data: format!("{}{}", callback_data_prefix, element.class_id),
        })
        .collect();
    let keyboard = make_inline_keyboard(buttons, row_size);
    keyboard
}
