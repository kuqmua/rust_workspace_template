pub(crate) fn make_json_response<T>(t: T) -> crate::json_response::JsonResponse<T> {
    crate::json_response::JsonResponse::from(crate::axum_json_payload::AxumJsonPayload::from(
        axum::Json(t),
    ))
}
