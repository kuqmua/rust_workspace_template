#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
#[serde(deny_unknown_fields)]
pub struct CreateNotificationReq {
    message: super::NotificationMessage,
}

impl CreateNotificationReq {
    #[must_use]
    pub fn into_message(self) -> super::NotificationMessage {
        self.message
    }
    #[must_use]
    pub const fn new(message: super::NotificationMessage) -> Self {
        Self { message }
    }
}
