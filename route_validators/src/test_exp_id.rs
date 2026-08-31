#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct TestExpId(&'static str);
