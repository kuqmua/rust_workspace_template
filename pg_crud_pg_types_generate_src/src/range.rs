#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub(super) enum Range {
    I32AsInt4,
    I64AsInt8,
    SqlxTypesChronoNaiveDateAsDate,
    SqlxTypesChronoNaiveDateTimeAsTimestamp,
    SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz,
}
impl TryFrom<&crate::pg_type_catalog_kind::PgTypeCatalogKind> for Range {
    type Error = ();
    fn try_from(
        value: &crate::pg_type_catalog_kind::PgTypeCatalogKind,
    ) -> Result<Self, Self::Error> {
        match &value {
                crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsInt2
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsInt4
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsInt8
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::F32AsFloat4
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::F64AsFloat8
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I16AsSmallSerialInitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I32AsSerialInitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::I64AsBigSerialInitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgMoneyAsMoney
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::BoolAsBool
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::StringAsText
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::StdVecVecU8AsBytea
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveTimeAsTime
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesTimeTimeAsTime
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgIntervalAsInterval
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateAsDate
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateTimeAsTimestamp
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidInitializationByClient
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesIpnetworkIpNetworkAsInet
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesMacAddressMacAddressAsMacAddr => Err(()),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI32AsInt4Range => Ok(Self::I32AsInt4),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI64AsInt8Range => Ok(Self::I64AsInt8),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange => Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz),
            }
    }
}
impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            naming::parameter::SelfNonNullUpperCamelCase::from_display(
                &crate::pg_type_catalog_kind::PgTypeCatalogKind::from(self)
            )
        )
    }
}
impl quote::ToTokens for Range {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(tokens);
    }
}
