#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstIdentifier(
    bounded_string_core::bounded_string_storage::BoundedStringStorage<
        0usize,
        { crate::first_ident_max_len::FIRST_IDENT_MAX_LEN },
        false,
    >,
);
impl
    From<crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError>
    for FirstIdentifier
{
    fn from(
        value: crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError,
    ) -> Self {
        bounded_string_core::try_from_error_text::try_from_error_text(value)
    }
}
impl TryFrom<String> for FirstIdentifier {
    type Error =
        crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::first_ident_max_len::FIRST_IDENT_MAX_LEN {
            return Err(crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError::from(value.len()));
        }
        bounded_string_core::bounded_string_storage::BoundedStringStorage::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_string_core::bounded_string_storage_error::BoundedStringStorageError::AboveMaximum {
                    actual_length,
                    ..
                }
                | bounded_string_core::bounded_string_storage_error::BoundedStringStorageError::BelowMinimum {
                    actual_length,
                    ..
                } => Self::Error::from(actual_length),
            })
    }
}
impl std::fmt::Display for FirstIdentifier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
