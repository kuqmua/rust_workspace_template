pub(crate) fn admin_route_path_url(
    admin_route_path: server_admin_contract::admin_route_path::AdminRoutePath,
) -> Result<
    crate::admin_csr_api_url::AdminCsrApiUrl,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    crate::admin_csr_api_url::AdminCsrApiUrl::try_from(admin_route_path.to_string())
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)
}
