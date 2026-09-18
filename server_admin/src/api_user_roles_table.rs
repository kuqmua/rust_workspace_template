#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "admin_tables")]
#[allow(
    clippy::single_call_fn,
    reason = "typed route registration requires a named endpoint function"
)]
pub(crate) async fn api_user_roles_table(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    axum_admin_json: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_data_table_query::AdminDataTableQuery,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminUserRolesTableError,
> {
    crate::data_tables_get::data_tables_get(
        admin_auth_request,
        server_admin_contract::admin_data_table::AdminDataTable::UserRoles,
        crate::axum_admin_query::AxumAdminQuery::from(axum_admin_json.into_inner()),
    )
    .await
    .map_err(crate::application_auth::AdminUserRolesTableError::from)
}
