#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct CreateNotificationRes {
    #[getters(copy)]
    id: crate::uuid_notification_id::UuidNotificationId,
}
