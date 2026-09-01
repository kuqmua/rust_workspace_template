#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    generate_constructor::New,
)]
pub struct AdminPermissionSummary {
    #[getters(copy)]
    id: crate::admin_permission_id::AdminPermissionId,
    name: crate::admin_permission_value::AdminPermissionValue,
}
