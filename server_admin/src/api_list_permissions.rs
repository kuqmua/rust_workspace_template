// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(
    delegate = crate::queries_list_permissions::queries_list_permissions,
    params(server_admin_contract::admin_table_query::AdminTableQuery),
    tag = "admin_roles"
)]
pub(crate) async fn api_list_permissions(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminListPermissionsError,
> {
}
