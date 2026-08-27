#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct LocationFileRef<'file_lt>(pub(super) &'file_lt str);
