pub(super) async fn authenticated_action<Action, ActionFuture>(
    auth: super::super::AdminAuthReq,
    path: server_admin_contract::domain_types::AdminFrontendPath,
    action: Action,
) -> axum::response::Response
where
    Action: FnOnce(super::super::AdminAuthReq) -> ActionFuture,
    ActionFuture:
        Future<Output = Result<super::super::AxumAdminResponse, super::super::AdminError>>,
{
    let Ok(auth) = super::form_auth_impl::form_auth(auth) else {
        return axum::response::IntoResponse::into_response(super::super::AdminError::Csrf);
    };
    super::action_result_impl::action_result(action(auth).await, path)
}
