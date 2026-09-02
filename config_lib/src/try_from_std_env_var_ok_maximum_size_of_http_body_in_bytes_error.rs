#[derive(Debug, thiserror::Error, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError {
    #[error("{maximum_size_of_http_body_in_bytes:?}")]
    MaximumSizeOfHttpBodyInBytes {
        maximum_size_of_http_body_in_bytes: crate::maximum_size_of_http_body_in_bytes_try_from_usize_error::MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    },
    #[error("{:?}", .usize_parsing)]
    UsizeParsing {
        usize_parsing: crate::usize_parse_int_error::UsizeParseIntError,
    },
}
