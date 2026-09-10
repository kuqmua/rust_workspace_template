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
#[serde(deny_unknown_fields)]
pub struct AdminRoleFilter {
    role_id: Option<crate::admin_role_id::AdminRoleId>,
    name: Option<crate::admin_role_name::AdminRoleName>,
    is_system: Option<crate::admin_bool::AdminBool>,
}
