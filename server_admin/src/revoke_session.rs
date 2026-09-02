#[proc_macro_frontend_contract::route_error(AdminHtmlRevokeSessionError)]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn revoke_session(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_form: crate::axum_admin_form::AxumAdminForm<
        crate::revoke_session_form::RevokeSessionForm,
    >,
) -> axum::response::Response {
    if !bool::from(*axum_admin_form.get_confirmation()) {
        return axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::Validation,
        );
    }
    let session_id = axum_admin_form
        .get_session_id()
        .to_string()
        .parse::<uuid::Uuid>()
        .map(server_admin_core::uuid_admin_value::UuidAdminValue::from)
        .map(crate::admin_session_id::AdminSessionId::from);
    let Ok(session_id) = session_id else {
        return axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::Validation,
        );
    };
    match crate::form_auth_impl::form_auth_impl(admin_auth_request) {
        Ok(auth) => {
            match crate::sessions_revoke_session::sessions_revoke_session(
                auth,
                crate::admin_session_path::AdminSessionPath::from(session_id),
            )
            .await
            {
                Ok(_response) => crate::success_redirect_impl::success_redirect_impl(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions,
                ),
                Err(error) => axum::response::IntoResponse::into_response(error),
            }
        }
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

// Root-owned module compatibility wrappers.
