#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::ToErrStringAsRefStr,
    newtype::FromInner,
)]
pub struct CommitToUse(pub(super) &'static str);
