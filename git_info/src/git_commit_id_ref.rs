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
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct GitCommitIdRef<'commit_lt>(&'commit_lt str);
impl PartialEq<&str> for GitCommitIdRef<'_> {
    fn eq(&self, str: &&str) -> bool {
        self.0 == *str
    }
}
