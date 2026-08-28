#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum RequestIdTryFromHttpHeaderValueError {
    #[error(transparent)]
    Invalid(super::RequestIdTryFromStringError),
    #[error("request id is not a text header: {0}")]
    ToStr(super::HttpHeaderToStrError),
}
