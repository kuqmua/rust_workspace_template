#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct HttpNotificationStatusCode(http::StatusCode);
impl axum::response::IntoResponse for HttpNotificationStatusCode {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}
