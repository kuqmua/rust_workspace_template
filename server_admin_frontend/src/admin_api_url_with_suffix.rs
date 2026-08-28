pub(crate) fn admin_api_url_with_suffix(
    route: server_admin_contract::domain_types::AdminRoute,
    suffix: super::AdminCsrApiUrlSuffixRef<'_>,
) -> Result<
    super::AdminCsrApiUrl,
    crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError,
> {
    super::AdminCsrApiUrl::try_from(format!("{}{}", route.path(), suffix.as_ref())).map_err(
        |_error| {
            crate::domain_types::start::state::admin_table_load_error::AdminTableLoadError::Query
        },
    )
}
