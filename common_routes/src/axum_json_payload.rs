#[derive(
    Debug,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
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
