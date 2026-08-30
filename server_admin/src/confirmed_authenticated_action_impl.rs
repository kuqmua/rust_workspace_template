pub(crate) async fn confirmed_authenticated_action_impl<Action, ActionFuture>(
    auth: crate::admin_auth_req::AdminAuthReq,
    confirmation: server_admin_contract::admin_bool::AdminBool,
    path: server_admin_contract::admin_frontend_path::AdminFrontendPath,
    action: Action,
) -> axum::response::Response
where
    Action: FnOnce(crate::admin_auth_req::AdminAuthReq) -> ActionFuture,
    ActionFuture: Future<
        Output = Result<
            crate::axum_admin_response::AxumAdminResponse,
            crate::admin_error::AdminError,
        >,
    >,
{
    if !bool::from(confirmation) {
        return axum::response::IntoResponse::into_response(
            crate::admin_error::AdminError::Validation,
        );
    }
    crate::authenticated_action_impl::authenticated_action_impl(auth, path, action).await
}
