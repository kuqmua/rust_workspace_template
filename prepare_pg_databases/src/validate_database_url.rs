#[allow(clippy::single_call_fn)] // named validation boundary is consumed by the Newtype derive
pub(super) fn validate_database_url<Value>(
    value: &Value,
) -> Result<(), crate::domain_types::DatabaseUrlError>
where
    Value: AsRef<str>,
{
    let value_ref = value.as_ref();
    if value_ref.trim().is_empty() {
        Err(crate::domain_types::DatabaseUrlError::Empty)
    } else if value_ref.len() > constants_usize::VALUE_8_192 {
        Err(crate::domain_types::DatabaseUrlError::TooLong)
    } else {
        Ok(())
    }
}
