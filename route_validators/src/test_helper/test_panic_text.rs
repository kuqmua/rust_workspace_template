#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::Display, newtype::FromInner)]
pub(super) struct TestPanicText(pub(super) &'static str);
