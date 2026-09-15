pub(crate) async fn dispatch_filtered_update<Error>(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_update_roles_request::AdminUpdateRolesRequest,
    >,
) -> Result<crate::axum_admin_response::AxumAdminResponse, Error>
where
    Error: From<crate::admin_error::AdminError>,
{
    crate::role_mutations_update_many::role_mutations_update_many(
        admin_auth_request,
        crate::admin_role_update_slice::AdminRoleUpdateSlice::from(
            axum_admin_json.into_inner().updates(),
        ),
    )
    .await
    .map_err(Error::from)
}
