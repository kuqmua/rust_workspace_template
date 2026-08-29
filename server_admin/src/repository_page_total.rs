// The owner module retains lint-sensitive semantics from the original implementation.

pub(crate) fn repository_page_total(
    value: crate::admin_page_total_count::AdminPageTotalCount,
) -> Result<
    server_admin_contract::admin_page_total::AdminPageTotal,
    crate::admin_repository_error::AdminRepositoryError,
> {
    u64::try_from(value.get())
        .map(server_admin_contract::admin_page_total::AdminPageTotal::from)
        .map_err(|_error| crate::admin_repository_error::AdminRepositoryError::InvalidStoredValue)
}
