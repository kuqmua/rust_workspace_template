use super::*;

#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeDeserialize {
    Derive,
    ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe),
}
impl From<&PgType> for PgTypeDeserialize {
    fn from(v: &PgType) -> Self {
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
                | PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                | PgType::F64AsFloat8 => Self::Derive,
                PgType::StringAsText => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::StringAsText)),
                PgType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime)),
                PgType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime)),
                PgType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate)),
                PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range)),
                PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range)),
                PgType::SqlxPgTypesPgIntervalAsInterval |
                PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDe(PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize),
            }
    }
}
