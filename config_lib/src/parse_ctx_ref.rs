#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::Display,
    newtype::FromInner,
)]
pub(super) struct ParseCtxRef(&'static str);
