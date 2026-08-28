#[allow(clippy::single_call_fn)] // named validation boundary is consumed by the Newtype derive
pub(super) fn validate_migrations_source<Value>(
    value: &Value,
) -> Result<(), super::MigrationsSourceError>
where
    Value: AsRef<str>,
{
    if value.as_ref().len() > 4_096usize {
        Err(super::MigrationsSourceError::TooLong)
    } else {
        Ok(())
    }
}
