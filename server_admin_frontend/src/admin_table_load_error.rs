#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, thiserror::Error)]
pub(crate) enum AdminTableLoadError {
    #[error("The table request failed.")]
    Fetch,
    #[error("The server returned status {0} for {1}.")]
    Http(
        crate::http::url::AdminHttpStatus,
        crate::http::url::AdminCsrApiUrl,
    ),
    #[error("The table query is invalid.")]
    Query,
    #[error("The table response was invalid.")]
    Response,
}
