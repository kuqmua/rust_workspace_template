pub(crate) trait AdminFilteredUpdateOperation {
    fn apply(
        self,
        admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    ) -> impl Future<
        Output = Result<
            crate::axum_admin_response::AxumAdminResponse,
            crate::admin_error::AdminError,
        >,
    > + Send;
}
impl AdminFilteredUpdateOperation
    for server_admin_contract::admin_update_roles_request::AdminUpdateRolesRequest
{
    async fn apply(
        self,
        admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    ) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
        crate::role_mutations_update_many::role_mutations_update_many(
            admin_auth_request,
            crate::admin_role_update_slice::AdminRoleUpdateSlice::from(self.updates()),
        )
        .await
    }
}
impl AdminFilteredUpdateOperation
    for server_admin_contract::admin_update_users_request::AdminUpdateUsersRequest
{
    async fn apply(
        self,
        admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    ) -> Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError> {
        crate::user_mutations_update_filtered::user_mutations_update_filtered(
            admin_auth_request,
            crate::admin_user_update_slice::AdminUserUpdateSlice::from(self.updates()),
        )
        .await
    }
}
