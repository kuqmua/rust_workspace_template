#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct JsonResponse<T> {
    payload: crate::axum_json_payload::AxumJsonPayload<T>,
}
impl<T> From<crate::axum_json_payload::AxumJsonPayload<T>> for JsonResponse<T> {
    fn from(axum_json_payload: crate::axum_json_payload::AxumJsonPayload<T>) -> Self {
        Self {
            payload: axum_json_payload,
        }
    }
}
impl<T> AsRef<crate::axum_json_payload::AxumJsonPayload<T>> for JsonResponse<T> {
    fn as_ref(&self) -> &crate::axum_json_payload::AxumJsonPayload<T> {
        &self.payload
    }
}
impl<T> axum::response::IntoResponse for JsonResponse<T>
where
    crate::axum_json_payload::AxumJsonPayload<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        self.payload.into_response()
    }
}
