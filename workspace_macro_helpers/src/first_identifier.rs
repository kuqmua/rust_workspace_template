use super::{FIRST_IDENT_MAX_LEN, FirstIdentifierifierTryFromStringError};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstIdentifier(String);
impl From<FirstIdentifierifierTryFromStringError> for FirstIdentifier {
    fn from(value: FirstIdentifierifierTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for FirstIdentifier {
    type Error = FirstIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > FIRST_IDENT_MAX_LEN {
            return Err(FirstIdentifierifierTryFromStringError(value.len()));
        }
        Ok(Self(value))
    }
}
impl std::fmt::Display for FirstIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
