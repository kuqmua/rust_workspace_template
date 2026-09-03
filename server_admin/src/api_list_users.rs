#[proc_macro_frontend_contract_route_openapi::route_openapi(
    params(server_admin_contract::admin_table_query::AdminTableQuery),
    tag = "admin_users"
)]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_list_users(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_query: crate::axum_admin_query::AxumAdminQuery<
        server_admin_contract::admin_table_query::AdminTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminListUsersError,
> {
    crate::user_queries_list::user_queries_list(admin_auth_request, axum_admin_query)
        .await
        .map_err(crate::application_auth::AdminListUsersError::from)
}
