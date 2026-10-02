#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminSessionReadPath {
    Access(crate::admin_access_session_id::AdminAccessSessionId),
    Refresh(crate::admin_refresh_token_id::AdminRefreshTokenId),
}
