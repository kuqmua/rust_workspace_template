#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeDeserialize {
    Derive,
    ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe),
}
impl From<&crate::pg_type_catalog_kind::PgTypeCatalogKind> for PgTypeDeserialize {
    fn from(v: &crate::pg_type_catalog_kind::PgTypeCatalogKind) -> Self {
        match v {
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
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidV4InitializationByPg
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesUuidUuidAsUuidInitializationByClient
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesIpnetworkIpNetworkAsInet
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesMacAddressMacAddressAsMacAddr
                | crate::pg_type_catalog_kind::PgTypeCatalogKind::F64AsFloat8 => Self::Derive,
                crate::pg_type_catalog_kind::PgTypeCatalogKind::StringAsText => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::StringAsText)),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxTypesChronoNaiveTimeAsTime)),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesTimeTimeAsTime => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxTypesTimeTimeAsTime)),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateAsDate => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxTypesChronoNaiveDateAsDate)),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI32AsInt4Range => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI32AsInt4Range)),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeI64AsInt8Range => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::TryNewForDe(crate::pg_type_impl_try_new_for_de::PgTypeImplTryNewForDe::SqlxPgTypesPgRangeI64AsInt8Range)),
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgIntervalAsInterval |
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoNaiveDateTimeAsTimestamp |
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz |
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange |
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange |
                crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange => Self::ImplNewForDeserializeOrTryNewForDe(crate::pg_type_impl_new_for_deserialize_or_try_new_for_de::PgTypeImplNewForDeserializeOrTryNewForDe::NewForDeserialize),
            }
    }
}
