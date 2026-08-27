#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct FieldLocationLineNonZeroU32(pub(super) std::num::NonZeroU32);
