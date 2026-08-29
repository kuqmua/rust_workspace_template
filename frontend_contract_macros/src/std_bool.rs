#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Default,
    newtype::FromInner,
    newtype::GetInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct StdBool(bool);
