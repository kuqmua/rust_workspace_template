#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_into_inner::IntoInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AxumAdminJson<Value>(Value);
impl<S, Value> axum::extract::FromRequest<S> for AxumAdminJson<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request(request: axum::extract::Request, s: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<Value>::from_request(request, s)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|error| {
                crate::admin_error::AdminError::body_rejection(
                    server_admin_core::std_admin_bool::StdAdminBool::from(
                        error.status() == http::StatusCode::PAYLOAD_TOO_LARGE,
                    ),
                )
            })
    }
}
