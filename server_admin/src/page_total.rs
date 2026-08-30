pub(crate) fn page_total(
    value: crate::admin_page_total_count::AdminPageTotalCount,
) -> Result<server_admin_contract::admin_page_total::AdminPageTotal, crate::admin_error::AdminError>
{
    crate::repository_page_total::repository_page_total(value)
        .map_err(|_error| crate::admin_error::AdminError::Validation)
}
