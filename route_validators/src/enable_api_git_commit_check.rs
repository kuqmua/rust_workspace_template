#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct EnableApiGitCommitCheck(bool);

impl EnableApiGitCommitCheck {
    pub(crate) const fn is_enabled(self) -> bool {
        self.0
    }
}
