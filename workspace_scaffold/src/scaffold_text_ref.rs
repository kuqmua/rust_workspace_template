#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ScaffoldTextRef<'text_lt>(&'text_lt str);
