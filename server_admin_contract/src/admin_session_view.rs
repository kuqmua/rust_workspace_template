#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct AdminSessionView {
    created_at: super::admin_session_timestamp::AdminSessionTimestamp,
    expires_at: super::admin_session_timestamp::AdminSessionTimestamp,
    id: super::admin_session_identifier::AdminSessionIdentifier,
    #[getters(copy)]
    #[serde(default)]
    is_current: crate::admin_bool::AdminBool,
}
