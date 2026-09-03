#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct AxumNotificationResponse(axum::response::Response);
impl axum::response::IntoResponse for AxumNotificationResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}
