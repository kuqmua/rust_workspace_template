pub(crate) fn optional_setting_impl<Value, Error>(
    value: crate::admin_html_form_text::AdminHtmlFormText,
) -> Result<Option<Value>, crate::admin_error::AdminError>
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
            .into_string(),
        )
        .map(Some)
        .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
