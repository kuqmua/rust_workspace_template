pub(crate) async fn authenticated_action_impl<Action, ActionFuture>(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_frontend_path: server_admin_contract::admin_frontend_path::AdminFrontendPath,
    action: Action,
) -> axum::response::Response
where
    Action: FnOnce(crate::admin_auth_request::AdminAuthRequest) -> ActionFuture,
    ActionFuture: Future<
        Output = Result<
            crate::axum_admin_response::AxumAdminResponse,
            crate::admin_error::AdminError,
        >,
    >,
{
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(admin_auth_request) else {
        return axum::response::IntoResponse::into_response(crate::admin_error::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(action(auth).await, admin_frontend_path)
}
