#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct GitCommitLinkOutputRefMut<'output_lt>(pub(super) &'output_lt mut String);
