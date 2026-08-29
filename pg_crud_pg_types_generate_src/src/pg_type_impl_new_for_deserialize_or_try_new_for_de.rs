#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeImplNewForDeserializeOrTryNewForDe {
    NewForDeserialize,
    TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe),
}
