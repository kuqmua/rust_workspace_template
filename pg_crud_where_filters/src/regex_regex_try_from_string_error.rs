#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error,
)]
pub enum RegexRegexTryFromStringError {
    #[error("regular expression pattern exceeds the size limit")]
    TooLong {
        #[source]
        source: bounded_types::bounded_string_error::BoundedStringError,
    },
}
