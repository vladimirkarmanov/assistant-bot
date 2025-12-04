#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[error("Произошла непредвиденная ошибка")]
pub struct SomethingWentWrongError;

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[error("Не удалось найти пользователя")]
pub struct UserNotFoundError;

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[error("Не удалось найти занятие")]
pub struct ClassNotFoundError;

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[error("Не удалось списать занятие. Количество доступных занятий {0}")]
pub struct NotEnoughClassQuantityToChargeError(pub u8);

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
#[error("Занятие с таким именем же существует. Пожалуйста, выберите другое имя.")]
pub struct DuplicateClassNameError;
