pub(crate) fn optional_setting_impl<Value, Error>(
    admin_html_form_text: crate::admin_html_form_text::AdminHtmlFormText,
) -> Result<Option<Value>, crate::admin_error::AdminError>
where
    Value: TryFrom<String, Error = Error>,
{
    if admin_html_form_text.trim().is_empty() {
        Ok(None)
    } else {
        Value::try_from(String::from(admin_html_form_text))
            .map(Some)
            .map_err(|_error| crate::admin_error::AdminError::Validation)
    }
}
