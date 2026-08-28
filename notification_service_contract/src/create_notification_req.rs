#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    generate_constructor::New,
)]
#[serde(deny_unknown_fields)]
pub struct CreateNotificationReq {
    message: crate::domain_types::NotificationMessage,
}

impl CreateNotificationReq {
    #[must_use]
    pub fn into_message(self) -> crate::domain_types::NotificationMessage {
        self.message
    }
}
