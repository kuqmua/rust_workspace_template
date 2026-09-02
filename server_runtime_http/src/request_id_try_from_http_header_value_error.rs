#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum RequestIdTryFromHttpHeaderValueError {
    #[error(transparent)]
    Invalid(crate::request_id_try_from_string_error::RequestIdTryFromStringError),
    #[error("request id is not a text header: {0}")]
    ToStr(crate::http_header_to_str_error::HttpHeaderToStrError),
}
