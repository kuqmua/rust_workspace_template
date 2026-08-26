#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, thiserror::Error)]
pub(in crate::domain_types::start) enum AdminTableLoadError {
    #[error("The table request failed.")]
    Fetch,
    #[error("The server returned status {0} for {1}.")]
    Http(
        super::super::http::url::AdminHttpStatus,
        super::super::http::url::AdminCsrApiUrl,
    ),
    #[error("The table query is invalid.")]
    Query,
    #[error("The table response was invalid.")]
    Response,
}
