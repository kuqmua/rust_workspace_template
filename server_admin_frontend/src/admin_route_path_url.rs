pub(crate) fn admin_route_path_url(
    admin_route_path: server_admin_contract::admin_route_path::AdminRoutePath,
) -> Result<super::AdminCsrApiUrl, crate::admin_table_load_error::AdminTableLoadError> {
    super::AdminCsrApiUrl::try_from(admin_route_path.to_string())
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)
}
