#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::ToErrString, newtype::FromInner,
)]
pub struct AxumBodySizeError(axum::Error);
