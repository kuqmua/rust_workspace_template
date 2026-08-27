#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DuplicateCandidates<T>(pub(crate) Vec<T>);
