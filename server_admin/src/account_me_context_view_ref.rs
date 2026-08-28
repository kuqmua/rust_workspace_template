pub(crate) async fn account_me_context_view_ref(
    auth: &crate::AdminAuthReq,
) -> Result<
    (
        server_admin_contract::domain_types::AuthenticatedAdmin,
        crate::AdminPasswordChangeRequired,
    ),
    crate::AdminError,
> {
    crate::authorization_authenticate::authorization_authenticate(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
    )
    .await
    .and_then(|authenticated| {
        let password_change_required = authenticated.password_change_required();
        crate::authenticated_admin_contract(&authenticated)
            .map(|contract| (contract, password_change_required))
    })
}
