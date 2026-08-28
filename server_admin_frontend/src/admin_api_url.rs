pub(crate) fn admin_api_url(
    route: server_admin_contract::domain_types::AdminRoute,
) -> Result<
    super::AdminCsrApiUrl,
    crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError,
> {
    super::admin_route_path_url(route.path())
}
