pub(crate) fn make_json_response<T>(payload: T) -> crate::json_res::JsonRes<T> {
    crate::json_res::JsonRes::from(crate::axum_json_payload::AxumJsonPayload::from(axum::Json(
        payload,
    )))
}
