pub(crate) fn admin_api_url(
    route: server_admin_contract::admin_route::AdminRoute,
) -> Result<super::AdminCsrApiUrl, crate::admin_table_load_error::AdminTableLoadError> {
    crate::admin_route_path_url::admin_route_path_url(route.path())
}
