use super::*;

#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeImplNewForDeserializeOrTryNewForDe {
    NewForDeserialize,
    TryNewForDe(PgTypeImplTryNewForDe),
}
