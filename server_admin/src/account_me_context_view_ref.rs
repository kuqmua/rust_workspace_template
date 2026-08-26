#![allow(clippy::single_call_fn)] // account context has one API and HTML composition owner

pub(super) async fn account_me_context_view_ref(
    auth: &super::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::domain_types::AuthenticatedAdmin,
        super::super::AdminPasswordChangeRequired,
    ),
    super::AdminError,
> {
    super::authorization_authenticate::authorization_authenticate(
        auth.state.as_ref(),
        super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await
    .and_then(|authenticated| {
        let password_change_required = authenticated.password_change_required();
        super::authenticated_admin_contract(&authenticated)
            .map(|contract| (contract, password_change_required))
    })
}
