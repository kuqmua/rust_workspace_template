use super::*;

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
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
impl TryFrom<&PgType> for PgTypeInitializationTryNew {
    type Error = ();
    fn try_from(v: &PgType) -> Result<Self, Self::Error> {
        match v {
                PgType::I16AsInt2
                | PgType::I32AsInt4
                | PgType::I64AsInt8
                | PgType::F32AsFloat4
                | PgType::I16AsSmallSerialInitializationByPg
                | PgType::I32AsSerialInitializationByPg
                | PgType::I64AsBigSerialInitializationByPg
                | PgType::SqlxPgTypesPgMoneyAsMoney
                | PgType::BoolAsBool
                | PgType::StdVecVecU8AsBytea
                | PgType::SqlxPgTypesPgIntervalAsInterval
                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                PgType::F64AsFloat8 => Ok(Self::F64AsFloat8),
                PgType::StringAsText => Ok(Self::StringAsText),
                PgType::SqlxTypesChronoNaiveTimeAsTime => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
                PgType::SqlxTypesTimeTimeAsTime => Ok(Self::SqlxTypesTimeTimeAsTime),
                PgType::SqlxTypesChronoNaiveDateAsDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::SqlxPgTypesPgRangeI32AsInt4Range),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::SqlxPgTypesPgRangeI64AsInt8Range),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange),
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange),
            }
    }
}
impl From<&PgTypeInitializationTryNew> for PgType {
    fn from(v: &PgTypeInitializationTryNew) -> Self {
        match v {
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
