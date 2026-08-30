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
pub struct ValidateProjectCommitError(crate::project_git_commit_link_ref::ProjectGitCommitLinkRef);
