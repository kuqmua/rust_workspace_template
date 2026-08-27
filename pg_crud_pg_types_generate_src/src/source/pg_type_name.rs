use super::*;

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, strum_macros::Display, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeName {
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    SmallSerialInitializationByPg,
    SerialInitializationByPg,
    BigSerialInitializationByPg,
    Money,
    Bool,
    Text,
    Bytea,
    Time,
    Interval,
    Date,
    Timestamp,
    TimestampTz,
    UuidV4InitializationByPg,
    UuidInitializationByClient,
    Inet,
    MacAddr,
    Int4Range,
    Int8Range,
    DateRange,
    TimestampRange,
    TimestampTzRange,
}
impl From<&PgType> for PgTypeName {
    fn from(v: &PgType) -> Self {
        match &v {
                PgType::I16AsInt2 => Self::Int2,
                PgType::I32AsInt4 => Self::Int4,
                PgType::I64AsInt8 => Self::Int8,
                PgType::F32AsFloat4 => Self::Float4,
                PgType::F64AsFloat8 => Self::Float8,
                PgType::I16AsSmallSerialInitializationByPg => Self::SmallSerialInitializationByPg,
                PgType::I32AsSerialInitializationByPg => Self::SerialInitializationByPg,
                PgType::I64AsBigSerialInitializationByPg => Self::BigSerialInitializationByPg,
                PgType::SqlxPgTypesPgMoneyAsMoney => Self::Money,
                PgType::BoolAsBool => Self::Bool,
                PgType::StringAsText => Self::Text,
                PgType::StdVecVecU8AsBytea => Self::Bytea,
                PgType::SqlxTypesChronoNaiveTimeAsTime | PgType::SqlxTypesTimeTimeAsTime => Self::Time,
                PgType::SqlxPgTypesPgIntervalAsInterval => Self::Interval,
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::Date,
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Self::Timestamp,
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Self::TimestampTz,
                PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg => Self::UuidV4InitializationByPg,
                PgType::SqlxTypesUuidUuidAsUuidInitializationByClient => Self::UuidInitializationByClient,
                PgType::SqlxTypesIpnetworkIpNetworkAsInet => Self::Inet,
                PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Self::MacAddr,
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::Int4Range,
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::Int8Range,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Self::DateRange,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Self::TimestampRange,
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::TimestampTzRange,
            }
    }
}
