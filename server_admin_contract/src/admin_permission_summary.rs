#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct AdminPermissionSummary {
    #[getters(copy)]
    id: crate::admin_permission_id::AdminPermissionId,
    name: crate::admin_permission_value::AdminPermissionValue,
}
