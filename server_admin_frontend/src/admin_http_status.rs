#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::Display,
    newtype::FromInner,
)]
pub(crate) struct AdminHttpStatus(u16);
