#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AdminSignInJson(server_admin_contract::admin_sign_in_request::AdminSignInRequest);
impl<S> axum::extract::FromRequest<S> for AdminSignInJson
where
    S: Send + Sync,
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request(request: axum::extract::Request, s: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<server_admin_contract::admin_sign_in_request::AdminSignInRequest>::from_request(
            request, s,
        )
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
