#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum CommonNotFoundError {
    #[error("common route was not found")]
    NotFound(crate::not_found_payload::NotFoundPayload),
}
impl axum::response::IntoResponse for CommonNotFoundError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound(payload) => axum::response::IntoResponse::into_response((
                axum::http::StatusCode::NOT_FOUND,
                axum::Json(payload),
            )),
        }
    }
}
