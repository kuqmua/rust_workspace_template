#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
#[serde(deny_unknown_fields)]
pub struct CreateNotificationRequest {
    message: crate::notification_message::NotificationMessage,
}

impl CreateNotificationRequest {
    #[must_use]
    pub fn into_message(self) -> crate::notification_message::NotificationMessage {
        self.message
    }
}
