#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminSessionView {
    created_at: super::admin_session_timestamp::AdminSessionTimestamp,
    expires_at: super::admin_session_timestamp::AdminSessionTimestamp,
    id: super::admin_session_identifier::AdminSessionIdentifier,
    #[getters(copy)]
    #[serde(default)]
    is_current: crate::admin_bool::AdminBool,
}
impl AdminSessionView {
    #[must_use]
    pub const fn new(
        created_at: super::admin_session_timestamp::AdminSessionTimestamp,
        expires_at: super::admin_session_timestamp::AdminSessionTimestamp,
        id: super::admin_session_identifier::AdminSessionIdentifier,
        is_current: crate::admin_bool::AdminBool,
    ) -> Self {
        Self {
            created_at,
            expires_at,
            id,
            is_current,
        }
    }
}
