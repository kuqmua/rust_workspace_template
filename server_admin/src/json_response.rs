pub(crate) fn json_response<Value>(value: Value) -> crate::axum_admin_response::AxumAdminResponse
where
    Value: serde::Serialize,
{
    crate::axum_admin_response::AxumAdminResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(value)),
    )
}
