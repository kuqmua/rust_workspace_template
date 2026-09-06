pub(crate) fn admin_api_url(
    admin_route: server_admin_contract::admin_route::AdminRoute,
) -> Result<
    crate::admin_csr_api_url::AdminCsrApiUrl,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    crate::admin_route_path_url::admin_route_path_url(&admin_route.path())
}
