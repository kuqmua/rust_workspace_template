pub(in crate::domain_types::auth) fn page_total(
    value: crate::adapters::repository::AdminPageTotalCount,
) -> Result<server_admin_contract::domain_types::AdminPageTotal, super::super::AdminError> {
    u64::try_from(value.get())
        .map(server_admin_contract::domain_types::AdminPageTotal::from)
        .map_err(|_error| super::super::AdminError::Validation)
}
