#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TrustedProxyRangeParseError {
    #[error("trusted proxy address is invalid")]
    InvalidAddress {
        #[source]
        source: super::ClientAddrParseError,
    },
    #[error("trusted proxy prefix is invalid")]
    InvalidPrefix {
        #[source]
        source: super::ParseIntError,
    },
    #[error("trusted proxy range must use address/prefix notation")]
    MissingPrefix,
    #[error("trusted proxy prefix exceeds address width")]
    PrefixExceedsAddressWidth,
}
