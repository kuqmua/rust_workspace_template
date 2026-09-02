// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, strum_macros::Display, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum RustTypeName {
    I16,
    I32,
    I64,
    F32,
    F64,
    SqlxPgTypesPgMoney,
    Bool,
    String,
    StdVecVecU8,
    SqlxTypesChronoNaiveTime,
    SqlxTypesTimeTime,
    SqlxPgTypesPgInterval,
    SqlxTypesChronoNaiveDate,
    SqlxTypesChronoNaiveDateTime,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
    SqlxTypesUuidUuid,
    SqlxTypesIpnetworkIpNetwork,
    SqlxTypesMacAddressMacAddress,
    SqlxPgTypesPgRangeI32,
    SqlxPgTypesPgRangeI64,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime,
    SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
}
impl From<&crate::pg_type_catalog_kind::PgTypeCatalogKind> for RustTypeName {
    fn from(pg_type_catalog_kind: &crate::pg_type_catalog_kind::PgTypeCatalogKind) -> Self {
        match &pg_type_catalog_kind {
                crate::pg_type_catalog_kind::PgTypeCatalogKind::F32AsFloat4 => Self::F32,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::F64AsFloat8 => Self::F64,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsInt2 | crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsSmallSerialInitializationByPg => Self::I16,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsInt4 | crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsSerialInitializationByPg => Self::I32,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsInt8 | crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsBigSerialInitializationByPg => Self::I64,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgMoneyAsMoney => Self::SqlxPgTypesPgMoney,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::BoolAsBool => Self::Bool,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::StringAsText => Self::String,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::StdVecVecU8AsBytea => Self::StdVecVecU8,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTime,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTime,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgIntervalAsInterval => Self::SqlxPgTypesPgInterval,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDate,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTime,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidV4InitializationByPg | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidInitializationByClient => Self::SqlxTypesUuidUuid,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesIpnetworkIpNetworkAsInet => Self::SqlxTypesIpnetworkIpNetwork,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesMacAddressMacAddressAsMacAddr => Self::SqlxTypesMacAddressMacAddress,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            }
    }
}
