pub(in crate::domain_types::auth) async fn authorize_custom(
    auth: &super::super::AdminAuthReq,
    permission: super::super::super::AdminPermission,
) -> Result<super::super::AuthenticatedAdmin, super::super::AdminError> {
    super::super::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        super::super::super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        permission.as_str(),
        super::super::super::StdAdminBool::from(true),
    )
    .await
}
