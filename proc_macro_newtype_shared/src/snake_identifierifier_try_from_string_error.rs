#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_foundation_foundation_from_inner::FromInner,
)]
pub(crate) struct SnakeIdentifierifierTryFromStringError(
    crate::snake_identifierifier_len::SnakeIdentifierifierLen,
);
impl std::fmt::Display for SnakeIdentifierifierTryFromStringError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "snake identifier length {} exceeds maximum {}",
            self.0.get(),
            crate::snake_ident_max_len::SNAKE_IDENT_MAX_LEN
        )
    }
}
