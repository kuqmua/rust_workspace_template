#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct TestPathStem<'stem_lt>(pub(super) &'stem_lt str);

impl<'stem_lt> TestPathStem<'stem_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<super::TestPathStemRef<'stem_lt>>,
    {
        Self::from(v.into().0)
    }
}
