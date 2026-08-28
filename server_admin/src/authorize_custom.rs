pub(crate) async fn authorize_custom(
    auth: &crate::AdminAuthReq,
    permission: crate::AdminPermission,
) -> Result<crate::AuthenticatedAdmin, crate::AdminError> {
    crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        permission.as_str(),
        crate::StdAdminBool::from(true),
    )
    .await
}
