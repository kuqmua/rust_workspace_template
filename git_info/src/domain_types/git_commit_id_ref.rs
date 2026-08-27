#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Default,
    serde_derive::Serialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct GitCommitIdRef<'commit_lt>(pub(super) &'commit_lt str);
impl PartialEq<&str> for GitCommitIdRef<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
