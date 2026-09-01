#[derive(generate_accessor::Getters)]
#[getters(bare)]
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
    #[getters(copy)]
    id: crate::uuid_notification_id::UuidNotificationId,
}
