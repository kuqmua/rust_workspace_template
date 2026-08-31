#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(super) struct NoRouteMessageCapacity(usize);
