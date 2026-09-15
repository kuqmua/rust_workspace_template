#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    proc_macro_new::New,
)]
#[serde(deny_unknown_fields)]
pub struct NotificationRequest {
    message: crate::runtime_notification_message::RuntimeNotificationMessage,
}
impl NotificationRequest {
    pub(crate) fn into_message(
        self,
    ) -> crate::runtime_notification_message::RuntimeNotificationMessage {
        self.message
    }
}
