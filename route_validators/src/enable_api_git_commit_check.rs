#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct EnableApiGitCommitCheck(bool);

impl EnableApiGitCommitCheck {
    pub(crate) const fn is_enabled(self) -> bool {
        self.0
    }
}
