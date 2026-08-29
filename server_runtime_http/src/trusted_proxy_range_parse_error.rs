#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum TrustedProxyRangeParseError {
    #[error("trusted proxy address is invalid")]
    InvalidAddress {
        #[source]
        source: crate::client_addr_parse_error::ClientAddrParseError,
    },
    #[error("trusted proxy prefix is invalid")]
    InvalidPrefix {
        #[source]
        source: crate::parse_int_error::ParseIntError,
    },
    #[error("trusted proxy range must use address/prefix notation")]
    MissingPrefix,
    #[error("trusted proxy prefix exceeds address width")]
    PrefixExceedsAddressWidth,
}
