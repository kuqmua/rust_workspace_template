#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub(super) enum PgDuplicateIdentifierPresence {
    Absent,
    Present,
}
