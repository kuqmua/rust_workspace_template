pub(crate) fn make_json_response<T>(payload: T) -> crate::json_res::JsonRes<T> {
    crate::json_res::JsonRes {
        payload: crate::axum_json_payload::AxumJsonPayload::from(axum::Json(payload)),
    }
}
