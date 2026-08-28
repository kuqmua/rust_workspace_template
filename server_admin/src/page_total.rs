pub(crate) fn page_total(
    value: crate::repository::AdminPageTotalCount,
) -> Result<server_admin_contract::domain_types::AdminPageTotal, crate::AdminError> {
    u64::try_from(value.get())
        .map(server_admin_contract::domain_types::AdminPageTotal::from)
        .map_err(|_error| crate::AdminError::Validation)
}
