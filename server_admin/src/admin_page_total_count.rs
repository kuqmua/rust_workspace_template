#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
    generate_accessor::Getters,
)]
pub(crate) struct AdminPageTotalCount(i64);
