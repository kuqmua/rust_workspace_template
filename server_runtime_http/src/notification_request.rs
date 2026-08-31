#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    generate_constructor::New,
)]
#[serde(deny_unknown_fields)]
pub struct NotificationRequest {
    message: crate::runtime_notification_message::RuntimeNotificationMessage,
}

impl From<NotificationRequest> for crate::runtime_notification_message::RuntimeNotificationMessage {
    fn from(value: NotificationRequest) -> Self {
        value.message
    }
}
