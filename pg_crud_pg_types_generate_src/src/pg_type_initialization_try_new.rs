#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type initialization try new keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeInitializationTryNew {
    F64AsFloat8,
    StringAsText,
    SqlxTypesChronoNaiveTimeAsTime,
    SqlxTypesTimeTimeAsTime,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxTypesChronoNaiveDateTimeAsTimestamp,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
    SqlxPgTypesPgRangeI32AsInt4Range,
    SqlxPgTypesPgRangeI64AsInt8Range,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
    SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
    SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
}
impl TryFrom<&crate::pg_type_catalog_kind::PgTypeCatalogKind> for PgTypeInitializationTryNew {
    type Error = ();
    fn try_from(
        pg_type_catalog_kind: &crate::pg_type_catalog_kind::PgTypeCatalogKind,
    ) -> Result<Self, Self::Error> {
        match pg_type_catalog_kind {
                crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsInt2
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsInt4
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsInt8
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::F32AsFloat4
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsSmallSerialInitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsSerialInitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsBigSerialInitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgMoneyAsMoney
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::BoolAsBool
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::StdVecVecU8AsBytea
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgIntervalAsInterval
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidInitializationByClient
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesIpnetworkIpNetworkAsInet
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::F64AsFloat8 => Ok(Self::F64AsFloat8),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::StringAsText => Ok(Self::StringAsText),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveTimeAsTime => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesTimeTimeAsTime => Ok(Self::SqlxTypesTimeTimeAsTime),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::SqlxPgTypesPgRangeI32AsInt4Range),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::SqlxPgTypesPgRangeI64AsInt8Range),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange),
            }
    }
}
impl From<&PgTypeInitializationTryNew> for crate::pg_type_catalog_kind::PgTypeCatalogKind {
    fn from(pg_type_initialization_try_new: &PgTypeInitializationTryNew) -> Self {
        match pg_type_initialization_try_new {
                PgTypeInitializationTryNew::F64AsFloat8 => Self::F64AsFloat8,
                PgTypeInitializationTryNew::StringAsText => Self::StringAsText,
                PgTypeInitializationTryNew::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTimeAsTime,
                PgTypeInitializationTryNew::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTimeAsTime,
                PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
                PgTypeInitializationTryNew::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTimeAsTimestamp,
                PgTypeInitializationTryNew::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32AsInt4Range,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64AsInt8Range,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange,
                PgTypeInitializationTryNew::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
            }
    }
}
