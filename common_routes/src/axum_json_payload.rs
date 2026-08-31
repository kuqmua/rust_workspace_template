#[derive(
    Debug, optimal_memory_layout::OptimalMemoryLayout, newtype::DerefInner, newtype::FromInner,
)]
pub(super) struct AxumJsonPayload<T>(axum::Json<T>);
impl<T> axum::response::IntoResponse for AxumJsonPayload<T>
where
    axum::Json<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
