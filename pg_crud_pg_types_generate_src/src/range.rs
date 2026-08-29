#[derive(optimal_memory_layout::OptimalMemoryLayout)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub(super) enum Range {
    I32AsInt4,
    I64AsInt8,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxTypesChronoNaiveDateTimeAsTimestamp,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
}
impl TryFrom<&crate::pg_type::PgType> for Range {
    type Error = ();
    fn try_from(v: &crate::pg_type::PgType) -> Result<Self, Self::Error> {
        match &v {
                crate::pg_type::PgType::I16AsInt2
                | crate::pg_type::PgType::I32AsInt4
                | crate::pg_type::PgType::I64AsInt8
                | crate::pg_type::PgType::F32AsFloat4
                | crate::pg_type::PgType::F64AsFloat8
                | crate::pg_type::PgType::I16AsSmallSerialInitializationByPg
                | crate::pg_type::PgType::I32AsSerialInitializationByPg
                | crate::pg_type::PgType::I64AsBigSerialInitializationByPg
                | crate::pg_type::PgType::SqlxPgTypesPgMoneyAsMoney
                | crate::pg_type::PgType::BoolAsBool
                | crate::pg_type::PgType::StringAsText
                | crate::pg_type::PgType::StdVecVecU8AsBytea
                | crate::pg_type::PgType::SqlxTypesChronoNaiveTimeAsTime
                | crate::pg_type::PgType::SqlxTypesTimeTimeAsTime
                | crate::pg_type::PgType::SqlxPgTypesPgIntervalAsInterval
                | crate::pg_type::PgType::SqlxTypesChronoNaiveDateAsDate
                | crate::pg_type::PgType::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | crate::pg_type::PgType::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | crate::pg_type::PgType::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | crate::pg_type::PgType::SqlxTypesUuidUuidAsUuidInitializationByClient
                | crate::pg_type::PgType::SqlxTypesIpnetworkIpNetworkAsInet
                | crate::pg_type::PgType::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                crate::pg_type::PgType::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::I32AsInt4),
                crate::pg_type::PgType::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::I64AsInt8),
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                crate::pg_type::PgType::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
            }
    }
}
impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            naming::parameter::SelfNonNullUpperCamelCase::from_display(
                &crate::pg_type::PgType::from(self)
            )
        )
    }
}
impl quote::ToTokens for Range {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(tokens);
    }
}
