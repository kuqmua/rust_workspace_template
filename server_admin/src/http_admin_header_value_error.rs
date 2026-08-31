#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugTransparent,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(transparent)]
#[derive(generate_accessor::Getters)]
pub struct HttpAdminHeaderValueError(http::header::InvalidHeaderValue);
