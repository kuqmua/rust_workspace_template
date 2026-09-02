#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    thiserror::Error,
)]
pub enum BoundedSecretTextError {
    #[error("secret text length is outside the allowed range")]
    InvalidLength,
    #[error("secret text repeats one byte")]
    RepeatedByte,
    #[error("secret text contains surrounding whitespace")]
    SurroundingWhitespace,
}
