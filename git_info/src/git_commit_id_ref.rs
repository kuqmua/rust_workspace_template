#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    Default,
    serde_derive::Serialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct GitCommitIdRef<'commit_lt>(&'commit_lt str);
impl PartialEq<&str> for GitCommitIdRef<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
