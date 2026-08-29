#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct JsonRes<T> {
    pub(super) payload: crate::axum_json_payload::AxumJsonPayload<T>,
}
impl<T> axum::response::IntoResponse for JsonRes<T>
where
    crate::axum_json_payload::AxumJsonPayload<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.payload.into_response()
    }
}
