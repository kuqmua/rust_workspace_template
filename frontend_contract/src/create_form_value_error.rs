pub(super) fn create_form_value_error(error: impl std::fmt::Display) -> crate::FormValueError {
    crate::FormValueError::try_from(error.to_string()).unwrap_or_default()
}
