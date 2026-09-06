#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, thiserror::Error)]
pub(crate) enum AdminTableLoadError {
    #[error("{}", constants_str::AUTHENTICATION_REQUIRED)]
    MissingCsrf,
    #[error("The table request failed.")]
    Fetch,
    #[error("The server returned status {0} for {1}.")]
    Http(
        crate::admin_http_status::AdminHttpStatus,
        crate::admin_csr_api_url::AdminCsrApiUrl,
    ),
    #[error("The table query is invalid.")]
    Query,
    #[error("The table response was invalid.")]
    Response,
}

impl AdminTableLoadError {
    pub(crate) fn requires_session_refresh(&self) -> server_admin_contract::admin_bool::AdminBool {
        server_admin_contract::admin_bool::AdminBool::from(match self {
            Self::MissingCsrf => true,
            Self::Http(status, _) => {
                *status == crate::admin_http_status::AdminHttpStatus::from(401u16)
            }
            Self::Fetch | Self::Query | Self::Response => false,
        })
    }
}
