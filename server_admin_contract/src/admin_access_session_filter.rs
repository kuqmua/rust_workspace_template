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
pub struct AdminAccessSessionFilter {
    session_id: Option<crate::admin_session_identifier::AdminSessionIdentifier>,
    user_id: Option<crate::admin_user_id::AdminUserId>,
}
