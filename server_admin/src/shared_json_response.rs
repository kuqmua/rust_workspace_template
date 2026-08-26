pub(in crate::domain_types::auth) fn json_response<Value>(
    value: Value,
) -> super::super::AxumAdminResponse
where
    Value: serde::Serialize,
{
    super::super::AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
        value,
    )))
}
