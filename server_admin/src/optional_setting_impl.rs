pub(crate) fn optional_setting_impl<Value, Error>(
    value: crate::AdminHtmlFormText,
) -> Result<Option<Value>, crate::AdminError>
where
    Value: TryFrom<String, Error = Error>,
{
    if value.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(
            bounded_types::bounded_string::BoundedString::<
                0,
                { constants_usize::VALUE_8_192 },
            >::from(value)
            .into_inner(),
        )
        .map(Some)
        .map_err(|_error| crate::AdminError::Validation)
    }
}
