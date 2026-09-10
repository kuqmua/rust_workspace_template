#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_new::New,
    proc_macro_getters::Getters,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
)]
#[getters(bare)]
#[serde(deny_unknown_fields)]
pub struct AdminUpdateRolesRequest {
    updates: crate::admin_role_updates::AdminRoleUpdates,
}
