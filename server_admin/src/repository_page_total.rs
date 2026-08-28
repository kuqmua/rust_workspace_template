// The owner module retains lint-sensitive semantics from the original implementation.

pub(crate) fn repository_page_total(
    value: crate::AdminPageTotalCount,
) -> Result<server_admin_contract::domain_types::AdminPageTotal, crate::AdminRepositoryError> {
    u64::try_from(value.get())
        .map(server_admin_contract::domain_types::AdminPageTotal::from)
        .map_err(|_error| crate::AdminRepositoryError::InvalidStoredValue)
}
