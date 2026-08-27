#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefInner,
    newtype::Display,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ProjectGitCommitLinkRef(pub(super) &'static str);
