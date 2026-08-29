#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(super) enum AdminAssetsError {
    #[error("administrator asset read failed: {0}")]
    Read(to_err_string::error_text::ErrorText),
}

impl axum::response::IntoResponse for AdminAssetsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Read(_error) => axum::response::IntoResponse::into_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
