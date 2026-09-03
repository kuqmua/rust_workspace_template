#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub(crate) enum BoundedStringOption {
    Chars,
    NulFree,
    Serde,
    Trim,
    Utoipa,
    WriteOnly,
}
