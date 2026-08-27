// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
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
