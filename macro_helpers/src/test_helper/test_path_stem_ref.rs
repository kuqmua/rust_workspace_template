#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct TestPathStemRef<'stem_lt>(pub(super) &'stem_lt str);
