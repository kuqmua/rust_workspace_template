// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::single_call_fn)]

pub(crate) fn page_total(
    value: super::AdminPageTotalCount,
) -> Result<server_admin_contract::domain_types::AdminPageTotal, super::AdminRepositoryError> {
    u64::try_from(value.get())
        .map(server_admin_contract::domain_types::AdminPageTotal::from)
        .map_err(|_error| super::AdminRepositoryError::InvalidStoredValue)
}
