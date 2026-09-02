#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::FromInner,
)]
pub struct ValidateProjectCommitError(crate::project_git_commit_link_ref::ProjectGitCommitLinkRef);
