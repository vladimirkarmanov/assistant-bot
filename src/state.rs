#[derive(Clone, Default)]
pub enum State {
    #[default]
    Idle,

    // Add class states
    AddingClassReceiveName,
    AddingClassReceiveQuantity {
        name: String,
    },

    // Update class quantity states
    UpdatingClassReceiveQuantity {
        class_id: i64,
    },

    // Add daily practice states
    AddingDailyPracticeReceiveMinutes,
}
