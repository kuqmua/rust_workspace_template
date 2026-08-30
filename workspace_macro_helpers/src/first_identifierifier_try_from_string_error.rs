#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstIdentifierifierTryFromStringError(usize);
impl From<usize> for FirstIdentifierifierTryFromStringError {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for FirstIdentifierifierTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "first identifier length {} exceeds maximum {}",
            self.0,
            crate::first_ident_max_len::FIRST_IDENT_MAX_LEN
        )
    }
}
