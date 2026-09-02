#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_foundation::FromInner,
)]
pub struct FirstIdentifierifierTryFromStringError(usize);
impl std::fmt::Display for FirstIdentifierifierTryFromStringError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "first identifier length {} exceeds maximum {}",
            self.0,
            crate::first_ident_max_len::FIRST_IDENT_MAX_LEN
        )
    }
}
