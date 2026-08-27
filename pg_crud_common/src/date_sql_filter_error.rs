#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum DateSqlFilterError {
    #[error("date SQL filter bind index overflowed")]
    BindIndexOverflow,
    #[error("date SQL filter exceeds the query fragment limit")]
    FragmentTooLong,
}
