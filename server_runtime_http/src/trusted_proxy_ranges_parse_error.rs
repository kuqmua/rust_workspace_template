#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TrustedProxyRangesParseError {
    #[error("trusted proxy range is invalid: {0}")]
    Range(super::TrustedProxyRangeParseError),
    #[error("trusted proxy range list is invalid: {0}")]
    Ranges(super::TrustedProxyRangesError),
}
