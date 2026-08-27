#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize, serde::Serialize,
)]
#[serde(deny_unknown_fields)]
pub struct NotificationRequest {
    pub(super) message: super::NotificationMessage,
}

impl NotificationRequest {
    #[must_use]
    pub const fn new(message: super::NotificationMessage) -> Self {
        Self { message }
    }
}
