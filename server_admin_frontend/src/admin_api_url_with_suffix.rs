pub(crate) fn admin_api_url_with_suffix(
    route: server_admin_contract::admin_route::AdminRoute,
    suffix: super::AdminCsrApiUrlSuffixRef<'_>,
) -> Result<super::AdminCsrApiUrl, crate::admin_table_load_error::AdminTableLoadError> {
    super::AdminCsrApiUrl::try_from(format!("{}{}", route.path(), suffix.as_ref()))
        .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)
}
