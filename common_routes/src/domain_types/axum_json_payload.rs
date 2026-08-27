#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct AxumJsonPayload<T>(pub(super) axum::Json<T>);
impl<T> axum::response::IntoResponse for AxumJsonPayload<T>
where
    axum::Json<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
