pub(crate) fn json_response<Value>(value: Value) -> crate::AxumAdminResponse
where
    Value: serde::Serialize,
{
    crate::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
        value,
    )))
}
