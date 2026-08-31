#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub(crate) struct TestPathStem<'stem_lt>(&'stem_lt str);

impl TestPathStem<'_> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<Self>,
    {
        v.into()
    }
}
