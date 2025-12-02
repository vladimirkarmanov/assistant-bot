use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};

use crate::repositories::class::Class;

pub struct MainMenuButton {
    pub text: String,
}

struct InlineButton {
    text: String,
    callback_data: String,
}

pub fn make_main_menu_keyboard(buttons: Vec<MainMenuButton>, row_size: usize) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for buttons in buttons.chunks(row_size) {
        let row = buttons
            .iter()
            .map(|button| KeyboardButton::new(button.text.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard).persistent()
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

pub fn make_class_list_inline_keyboard(
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
    make_inline_keyboard(buttons, row_size)
}
