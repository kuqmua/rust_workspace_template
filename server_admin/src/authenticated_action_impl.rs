pub(crate) async fn authenticated_action_impl<Action, ActionFuture>(
    auth: crate::AdminAuthReq,
    path: server_admin_contract::domain_types::AdminFrontendPath,
    action: Action,
) -> axum::response::Response
where
    Action: FnOnce(crate::AdminAuthReq) -> ActionFuture,
    ActionFuture: Future<Output = Result<crate::AxumAdminResponse, crate::AdminError>>,
{
    let Ok(auth) = crate::form_auth_impl::form_auth_impl(auth) else {
        return axum::response::IntoResponse::into_response(crate::AdminError::Csrf);
    };
    crate::action_result_impl::action_result_impl(action(auth).await, path)
}
