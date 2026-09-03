#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type impl try new for de keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeImplTryNewForDe {
    StringAsText,
    SqlxTypesChronoNaiveTimeAsTime,
    SqlxTypesTimeTimeAsTime,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxPgTypesPgRangeI32AsInt4Range,
    SqlxPgTypesPgRangeI64AsInt8Range,
    SqlxTypesUuidUuidAsUuidV4InitializationByPg,
    SqlxTypesUuidUuidAsUuidInitializationByClient,
}
