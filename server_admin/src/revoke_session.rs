pub(crate) use admin_html_session_action_route_registry::AdminHtmlSessionActionRouteRegistry;

#[frontend_contract::domain_types::route_error(AdminHtmlRevokeSessionError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn revoke_session(
    auth: crate::AdminAuthReq,
    crate::AxumAdminForm(form): crate::AxumAdminForm<crate::RevokeSessionForm>,
) -> axum::response::Response {
    if !bool::from(form.confirmation) {
        return axum::response::IntoResponse::into_response(crate::AdminError::Validation);
    }
    let session_id = form
        .session_id
        .to_string()
        .parse::<uuid::Uuid>()
        .map(crate::UuidAdminValue::from)
        .map(crate::AdminSessionId::from);
    let Ok(session_id) = session_id else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Validation);
    };
    match crate::form_auth_impl::form_auth_impl(auth) {
        Ok(auth) => {
            match crate::sessions_revoke_session::sessions_revoke_session(
                auth,
                crate::AdminSessionPath(session_id),
            )
            .await
            {
                Ok(_response) => crate::success_redirect_impl::success_redirect_impl(
                    server_admin_contract::domain_types::AdminFrontendPath::Sessions,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

// Root-owned module compatibility wrappers.
mod admin_html_session_action_route_registry {
    pub use crate::admin_html_session_action_route_registry::*;
}
