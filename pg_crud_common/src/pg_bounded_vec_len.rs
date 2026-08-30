#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    newtype::Display,
    newtype::FromInner,
    newtype::GetInner,
)]
#[allow(clippy::module_name_repetitions)] // the public name remains explicit when imported outside this module
pub struct PgBoundedVecLen(usize);
