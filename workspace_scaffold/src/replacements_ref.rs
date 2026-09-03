#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(crate) struct ReplacementsRef<'replacements_lt>(
    &'replacements_lt [(&'replacements_lt str, String)],
);
