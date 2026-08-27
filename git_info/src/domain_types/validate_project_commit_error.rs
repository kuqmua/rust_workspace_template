use super::ProjectGitCommitLinkRef;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct ValidateProjectCommitError(pub(super) ProjectGitCommitLinkRef);
