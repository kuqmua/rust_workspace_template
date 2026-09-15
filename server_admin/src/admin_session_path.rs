#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_getters::Getters,
)]
pub(crate) struct AdminSessionPath(crate::admin_session_id::AdminSessionId);
impl
    axum::extract::FromRequestParts<
        crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    > for AdminSessionPath
{
    type Rejection = crate::admin_error::AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        shared_admin_auth_svc_state_arc: &crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<uuid::Uuid>::from_request_parts(
            parts,
            shared_admin_auth_svc_state_arc,
        )
        .await
        .map(|axum::extract::Path(value)| {
            Self::from(crate::admin_session_id::AdminSessionId::from(
                server_admin_core::uuid_admin_value::UuidAdminValue::from(value),
            ))
        })
        .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
