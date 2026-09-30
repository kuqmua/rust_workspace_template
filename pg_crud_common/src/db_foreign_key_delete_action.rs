#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
)]
pub enum DbForeignKeyDeleteAction {
    Cascade,
    NoAction,
    Restrict,
    SetDefault,
    SetNull,
}
