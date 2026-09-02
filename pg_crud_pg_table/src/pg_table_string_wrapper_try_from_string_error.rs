#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum PgTableStringWrapperTryFromStringError {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for PgTableStringWrapperTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(
                    f,
                    "pg table string wrapper length {len} exceeds maximum {max}"
                )
            }
        }
    }
}
