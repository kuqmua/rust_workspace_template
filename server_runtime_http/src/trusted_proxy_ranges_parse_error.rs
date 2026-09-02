#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TrustedProxyRangesParseError {
    #[error("trusted proxy range is invalid: {0}")]
    Range(crate::trusted_proxy_range_parse_error::TrustedProxyRangeParseError),
    #[error("trusted proxy range list is invalid: {0}")]
    Ranges(crate::trusted_proxy_ranges_error::TrustedProxyRangesError),
}
