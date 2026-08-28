#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd,
)]
pub enum DbObjectKind {
    Check,
    Default,
    Extension,
    ForeignKey,
    Function,
    Index,
    PrimaryKey,
    Trigger,
    Unique,
    View,
}
