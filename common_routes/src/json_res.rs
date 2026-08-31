#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct JsonRes<T> {
    payload: crate::axum_json_payload::AxumJsonPayload<T>,
}
impl<T> From<crate::axum_json_payload::AxumJsonPayload<T>> for JsonRes<T> {
    fn from(value: crate::axum_json_payload::AxumJsonPayload<T>) -> Self {
        Self { payload: value }
    }
}
impl<T> AsRef<crate::axum_json_payload::AxumJsonPayload<T>> for JsonRes<T> {
    fn as_ref(&self) -> &crate::axum_json_payload::AxumJsonPayload<T> {
        &self.payload
    }
}
impl<T> axum::response::IntoResponse for JsonRes<T>
where
    crate::axum_json_payload::AxumJsonPayload<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.payload.into_response()
    }
}
