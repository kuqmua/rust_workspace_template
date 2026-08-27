#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationFile(pub(super) &'static str);
