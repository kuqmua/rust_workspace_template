pub(crate) fn admin_api_url_with_suffix(
    admin_route: server_admin_contract::admin_route::AdminRoute,
    admin_csr_api_url_suffix_ref: crate::admin_csr_api_url_suffix_ref::AdminCsrApiUrlSuffixRef<'_>,
) -> Result<
    crate::admin_csr_api_url::AdminCsrApiUrl,
    crate::admin_table_load_error::AdminTableLoadError,
> {
    crate::admin_csr_api_url::AdminCsrApiUrl::try_from(format!(
        "{}{}",
        admin_route.path(),
        admin_csr_api_url_suffix_ref.as_ref()
    ))
    .map_err(|_error| crate::admin_table_load_error::AdminTableLoadError::Query)
}
