#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct TestPathStem<'stem_lt>(&'stem_lt str);

impl TestPathStem<'_> {
    pub(crate) fn new<T>(t: T) -> Self
    where
        T: Into<Self>,
    {
        t.into()
    }
}
