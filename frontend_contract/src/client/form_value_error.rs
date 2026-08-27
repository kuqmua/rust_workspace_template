pub(super) fn form_value_error(
    error: impl std::fmt::Display,
) -> crate::domain_types::FormValueError {
    crate::domain_types::FormValueError::try_from(error.to_string()).unwrap_or_default()
}
