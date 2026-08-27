#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::ToErrString, newtype::FromInner,
)]
pub struct AxumCommitToStrConversionError(axum::http::header::ToStrError);
