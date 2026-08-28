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
)]
pub struct CreateNotificationRes {
    id: super::UuidNotificationId,
}

impl CreateNotificationRes {
    #[must_use]
    pub const fn id(&self) -> super::UuidNotificationId {
        self.id
    }
    #[must_use]
    pub const fn new(id: super::UuidNotificationId) -> Self {
        Self { id }
    }
}
