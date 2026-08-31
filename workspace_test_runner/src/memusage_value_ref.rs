#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct MemusageValueRef<'lt>(&'lt str);
