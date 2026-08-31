pub(crate) fn optional_setting_impl<Value, Error>(
    value: crate::admin_html_form_text::AdminHtmlFormText,
) -> Result<Option<Value>, crate::admin_error::AdminError>
where
    Value: TryFrom<String, Error = Error>,
{
    if value.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(String::from(value))
            .map(Some)
            .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
