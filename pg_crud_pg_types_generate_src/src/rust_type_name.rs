// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, strum_macros::Display, optimal_memory_layout::OptimalMemoryLayout)]
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
impl From<&crate::pg_type::PgType> for RustTypeName {
    fn from(v: &crate::pg_type::PgType) -> Self {
        match &v {
                crate::pg_type::PgType::F32AsFloat4 => Self::F32,
                crate::pg_type::PgType::F64AsFloat8 => Self::F64,
                crate::pg_type::PgType::I16AsInt2 | crate::pg_type::PgType::I16AsSmallSerialInitializationByPg => Self::I16,
                crate::pg_type::PgType::I32AsInt4 | crate::pg_type::PgType::I32AsSerialInitializationByPg => Self::I32,
                crate::pg_type::PgType::I64AsInt8 | crate::pg_type::PgType::I64AsBigSerialInitializationByPg => Self::I64,
                crate::pg_type::PgType::SqlxPgTypesPgMoneyAsMoney => Self::SqlxPgTypesPgMoney,
                crate::pg_type::PgType::BoolAsBool => Self::Bool,
                crate::pg_type::PgType::StringAsText => Self::String,
                crate::pg_type::PgType::StdVecVecU8AsBytea => Self::StdVecVecU8,
                crate::pg_type::PgType::SqlxTypesChronoNaiveTimeAsTime => Self::SqlxTypesChronoNaiveTime,
                crate::pg_type::PgType::SqlxTypesTimeTimeAsTime => Self::SqlxTypesTimeTime,
                crate::pg_type::PgType::SqlxPgTypesPgIntervalAsInterval => Self::SqlxPgTypesPgInterval,
                crate::pg_type::PgType::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDate,
                crate::pg_type::PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::SqlxTypesChronoNaiveDateTime,
                crate::pg_type::PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc,
                crate::pg_type::PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg | crate::pg_type::PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => Self::SqlxTypesUuidUuid,
                crate::pg_type::PgType::SqlxTypesIpnetworkIpNetworkAsInet => Self::SqlxTypesIpnetworkIpNetwork,
                crate::pg_type::PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::SqlxTypesMacAddressMacAddress,
                crate::pg_type::PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::SqlxPgTypesPgRangeI32,
                crate::pg_type::PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::SqlxPgTypesPgRangeI64,
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate,
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime,
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc,
            }
    }
}
