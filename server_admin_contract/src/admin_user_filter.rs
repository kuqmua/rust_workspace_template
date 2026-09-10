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
pub struct AdminUserFilter {
    user_id: Option<crate::admin_user_id::AdminUserId>,
    login: Option<crate::admin_login::AdminLogin>,
    display_name: Option<crate::admin_display_name::AdminDisplayName>,
    is_banned: Option<crate::admin_bool::AdminBool>,
}
