#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum GitInfoStringTryFromStringError {
    TooLong { len: usize, max: usize },
}
impl std::fmt::Display for GitInfoStringTryFromStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong { len, max } => {
                write!(f, "git info string length {len} exceeds maximum {max}")
            }
        }
    }
}
