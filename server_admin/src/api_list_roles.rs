// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(
    delegate = crate::role_queries_list::role_queries_list,
    params(server_admin_contract::admin_table_query::AdminTableQuery),
    tag = "admin_roles"
)]
pub(crate) async fn api_list_roles(
    auth: crate::admin_auth_req::AdminAuthReq,
    query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminListRolesError,
> {
}
