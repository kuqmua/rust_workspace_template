#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    generate_constructor::New,
)]
pub struct CreateNotificationRes {
    id: super::UuidNotificationId,
}

impl CreateNotificationRes {
    #[must_use]
    pub const fn id(&self) -> super::UuidNotificationId {
        self.id
    }
}
