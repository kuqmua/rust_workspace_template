// The owner module retains lint-sensitive semantics from the original implementation.

#[proc_macro_frontend_contract::route_openapi(
    delegate = crate::role_queries_list::role_queries_list,
    params(server_admin_contract::admin_table_query::AdminTableQuery),
    tag = "admin_roles"
)]
pub(crate) async fn api_list_roles(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminListRolesError,
> {
}
