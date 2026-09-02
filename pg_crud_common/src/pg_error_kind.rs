#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum PgErrorKind {
    CheckViolation,
    Connection,
    Deadlock,
    ForeignKeyViolation,
    InvalidTextRepresentation,
    NotNullViolation,
    NumericValueOutOfRange,
    PoolTimedOut,
    SerializationFailure,
    StringDataRightTruncation,
    UniqueViolation,
    Unknown,
}
