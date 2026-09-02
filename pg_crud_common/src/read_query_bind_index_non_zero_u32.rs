#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct ReadQueryBindIndexNonZeroU32(std::num::NonZeroU32);
