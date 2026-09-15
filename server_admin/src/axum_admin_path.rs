#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_into_inner::IntoInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AxumAdminPath<Value>(Value);
impl<S, Value> axum::extract::FromRequestParts<S> for AxumAdminPath<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        s: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<Value>::from_request_parts(parts, s)
            .await
            .map(|axum::extract::Path(value)| Self::from(value))
            .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
