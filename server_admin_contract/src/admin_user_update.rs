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
pub struct AdminUserUpdate {
    changes: crate::admin_update_user_request::AdminUpdateUserRequest,
    #[getters(copy)]
    user_id: crate::admin_user_id::AdminUserId,
}
