#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminMetricsError {
    #[error(transparent)]
    Render(server_runtime_http::metrics_response_body_error::MetricsResponseBodyError),
}
impl axum::response::IntoResponse for AdminMetricsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Render(_error) => axum::response::IntoResponse::into_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
