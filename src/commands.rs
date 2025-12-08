use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case", description = "Доступные команды:")]
pub enum Command {
    #[command(description = "Перезапустить бота ♻️")]
    Start,
    #[command(description = "Перейти в главное меню 🏠")]
    MainMenu,
    #[command(description = "Отменить операцию ❌")]
    CancelOperation,
    #[command(description = "Помощь ℹ️")]
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    Classes,
    AddClass,
    DeductClass,
    ClassSettings,
    ListClasses,
    ClassesDeductionHistory,
    UpdateQuantity,
    MainMenu,
}

impl MenuAction {
    pub fn label(self) -> &'static str {
        match self {
            MenuAction::Classes => "Занятия",
            MenuAction::AddClass => "Добавить занятие",
            MenuAction::DeductClass => "Списать занятие",
            MenuAction::ClassSettings => "Настройка занятий",
            MenuAction::ListClasses => "Список занятий",
            MenuAction::ClassesDeductionHistory => "История списаний",
            MenuAction::UpdateQuantity => "Обновить количество",
            MenuAction::MainMenu => "Главное меню",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "Занятия" => Some(MenuAction::Classes),
            "Добавить занятие" => Some(MenuAction::AddClass),
            "Списать занятие" => Some(MenuAction::DeductClass),
            "Настройка занятий" => Some(MenuAction::ClassSettings),
            "Список занятий" => Some(MenuAction::ListClasses),
            "История списаний" => Some(MenuAction::ClassesDeductionHistory),
            "Обновить количество" => Some(MenuAction::UpdateQuantity),
            "Главное меню" => Some(MenuAction::MainMenu),
            _ => None,
        }
    }
}
