#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DebugTransparent,
    thiserror::Error,
    proc_macro_newtype::FromInner,
)]
#[error(transparent)]
#[derive(proc_macro_getters::Getters)]
pub struct HttpAdminHeaderValueError(http::header::InvalidHeaderValue);
