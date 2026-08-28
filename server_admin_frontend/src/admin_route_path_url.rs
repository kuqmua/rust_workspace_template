pub(crate) fn admin_route_path_url(
    path: server_admin_contract::domain_types::AdminRoutePath,
) -> Result<
    super::AdminCsrApiUrl,
    crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError,
> {
    super::AdminCsrApiUrl::try_from(path.to_string()).map_err(|_error| {
        crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Query
    })
}
