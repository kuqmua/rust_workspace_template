#[frontend_contract::domain_types::route_error(AdminHtmlRevokeSessionError)]
pub(super) async fn revoke_session(
    auth: super::super::super::AdminAuthReq,
    super::super::super::AxumAdminForm(form): super::super::super::AxumAdminForm<
        super::super::forms::RevokeSessionForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            super::super::super::AdminError::Validation,
        );
    }
    let session_id = form
        .session_id
        .to_string()
        .parse::<uuid::Uuid>()
        .map(super::super::super::super::UuidAdminValue::from)
        .map(super::super::super::super::AdminSessionId::from);
    let Ok(session_id) = session_id else {
        return axum::response::IntoResponse::into_response(
            super::super::super::AdminError::Validation,
        );
    };
    match super::super::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => {
            match super::super::super::sessions_revoke_session::sessions_revoke_session(
                auth,
                super::super::super::AdminSessionPath(session_id),
            )
            .await
            {
                Ok(_response) => super::super::success_redirect_impl::success_redirect_impl(
                    server_admin_contract::domain_types::AdminFrontendPath::Sessions,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[frontend_contract::domain_types::endpoint_registry(
    state = super::super::super::SharedAdminAuthSvcStateArc;
    (server_admin_contract::domain_types::AdminHtmlAction::SessionRevoke, revoke_session),
)]
struct AdminHtmlSessionActionRouteRegistry;

pub(super) fn router() -> super::super::super::AxumAdminStateRouter {
    super::super::super::AxumAdminStateRouter::from(AdminHtmlSessionActionRouteRegistry::router())
}
