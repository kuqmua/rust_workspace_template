#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::FromInner,
    newtype::GetInner,
)]
pub(crate) struct ReplacementsRef<'replacements_lt>(
    pub(super) &'replacements_lt [(&'replacements_lt str, String)],
);
