#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum FilterKind {
    Bool,
    Bytes,
    Date,
    IntervalOrInet,
    Mac,
    Money,
    Number,
    Range,
    String,
    Time,
    Timestamp,
    TimestampTz,
    Uuid,
}
