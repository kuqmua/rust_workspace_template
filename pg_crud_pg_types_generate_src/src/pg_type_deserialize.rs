#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeDeserialize {
    Derive,
    ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe),
}
impl From<&crate::pg_type::PgType> for PgTypeDeserialize {
    fn from(v: &crate::pg_type::PgType) -> Self {
        match v {
                crate::pg_type::PgType::I16AsInt2
                | crate::pg_type::PgType::I32AsInt4
                | crate::pg_type::PgType::I64AsInt8
                | crate::pg_type::PgType::F32AsFloat4
                | crate::pg_type::PgType::I16AsSmallSerialInitializationByPg
                | crate::pg_type::PgType::I32AsSerialInitializationByPg
                | crate::pg_type::PgType::I64AsBigSerialInitializationByPg
                | crate::pg_type::PgType::SqlxPgTypesPgMoneyAsMoney
                | crate::pg_type::PgType::BoolAsBool
                | crate::pg_type::PgType::StdVecVecU8AsBytea
                | crate::pg_type::PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | crate::pg_type::PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | crate::pg_type::PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | crate::pg_type::PgType::SqlxTypesMacAddressMacAddressAsMacAddr
                | crate::pg_type::PgType::F64AsFloat8 => Self::Derive,
                crate::pg_type::PgType::StringAsText => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::StringAsText)),
                crate::pg_type::PgType::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime)),
                crate::pg_type::PgType::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime)),
                crate::pg_type::PgType::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate)),
                crate::pg_type::PgType::SqlxPgTypesPgRangeI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range)),
                crate::pg_type::PgType::SqlxPgTypesPgRangeI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range)),
                crate::pg_type::PgType::SqlxPgTypesPgIntervalAsInterval |
                crate::pg_type::PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                crate::pg_type::PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize),
            }
    }
}
