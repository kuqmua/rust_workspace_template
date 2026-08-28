#[path = "admin_html_session_action_route_registry.rs"]
mod admin_html_session_action_route_registry;

pub(in crate::domain_types::auth::html::actions) use admin_html_session_action_route_registry::AdminHtmlSessionActionRouteRegistry;

#[frontend_contract::domain_types::route_error(AdminHtmlRevokeSessionError)]
pub(in crate::domain_types::auth::html::actions) async fn revoke_session(
    auth: super::super::super::super::AdminAuthReq,
    super::super::super::super::AxumAdminForm(form): super::super::super::super::AxumAdminForm<
        super::super::super::forms::RevokeSessionForm,
    >,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Validation,
        );
    }
    let session_id = form
        .session_id
        .to_string()
        .parse::<uuid::Uuid>()
        .map(super::super::super::super::super::UuidAdminValue::from)
        .map(super::super::super::super::super::AdminSessionId::from);
    let Ok(session_id) = session_id else {
        return axum::response::IntoResponse::into_response(
            super::super::super::super::AdminError::Validation,
        );
    };
    match super::super::super::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => {
            match super::super::super::super::sessions_revoke_session::sessions_revoke_session(
                auth,
                super::super::super::super::AdminSessionPath(session_id),
            )
            .await
            {
                Ok(_response) => super::super::super::success_redirect_impl::success_redirect_impl(
                    server_admin_contract::domain_types::AdminFrontendPath::Sessions,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
