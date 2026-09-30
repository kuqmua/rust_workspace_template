#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeImplNewForDeserializeOrTryNewForDeserialize {
    NewForDeserialize,
    TryNewForDeserialize(
        crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize,
    ),
}
