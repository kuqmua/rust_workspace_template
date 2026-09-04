#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstIdentifier(String);
impl
    From<crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError>
    for FirstIdentifier
{
    fn from(
        value: crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for FirstIdentifier {
    type Error =
        crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::first_ident_max_len::FIRST_IDENT_MAX_LEN {
            return Err(crate::first_identifierifier_try_from_string_error::FirstIdentifierifierTryFromStringError::from(value.len()));
        }
        Ok(Self(value))
    }
}
impl std::fmt::Display for FirstIdentifier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
