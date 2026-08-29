#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum RegexRegexTryFromStringError {
    #[error("regular expression pattern is invalid")]
    Regex(#[from] crate::regex_error::RegexError),
    #[error("regular expression pattern exceeds the size limit")]
    TooLong,
}
