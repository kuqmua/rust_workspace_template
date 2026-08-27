#[derive(Debug, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError {
    #[error("{maximum_size_of_http_body_in_bytes:?}")]
    MaximumSizeOfHttpBodyInBytes {
        maximum_size_of_http_body_in_bytes: super::MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    },
    #[error("{:?}", .usize_parsing)]
    UsizeParsing {
        usize_parsing: super::super::UsizeParseIntError,
    },
}
