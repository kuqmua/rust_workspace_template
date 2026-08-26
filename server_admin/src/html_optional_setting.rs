pub(super) fn optional_setting<Value, Error>(
    value: super::forms::AdminHtmlFormText,
) -> Result<Option<Value>, super::super::AdminError>
where
    Value: TryFrom<String, Error = Error>,
{
    if value.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(
            bounded_types::domain_types::bounded_string::BoundedString::<
                0,
                { constants_usize::VALUE_8_192 },
            >::from(value)
            .into_inner(),
        )
        .map(Some)
        .map_err(|_error| super::super::AdminError::Validation)
    }
}
