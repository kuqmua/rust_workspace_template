#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::Display, newtype::FromInner)]
pub(crate) struct TestExpId(pub(super) &'static str);
