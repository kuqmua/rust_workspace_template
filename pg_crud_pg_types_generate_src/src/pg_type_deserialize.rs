#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(super) enum PgTypeDeserialize {
    Derive,
    ImplNewForDeserializeOrTryNewForDeserialize(crate::pg_type_impl_new_for_deserialize_or_try_new_for_deserialize::PgTypeImplNewForDeserializeOrTryNewForDeserialize),
}
impl From<&crate::pg_type_catalog_kind::PgTypeCatalogKind> for PgTypeDeserialize {
    fn from(value: &crate::pg_type_catalog_kind::PgTypeCatalogKind) -> Self {
        match crate::schema_wire_kind::schema_wire_kind(&(*value).spec()) {
            crate::wire_kind::WireKind::String => Self::try_new_for_deserialize(
                crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize::StringAsText,
            ),
            crate::wire_kind::WireKind::TimeChrono => Self::try_new_for_deserialize(
                crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveTimeAsTime,
            ),
            crate::wire_kind::WireKind::TimeTime => Self::try_new_for_deserialize(
                crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize::SqlxTypesTimeTimeAsTime,
            ),
            crate::wire_kind::WireKind::Date => Self::try_new_for_deserialize(
                crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize::SqlxTypesChronoNaiveDateAsDate,
            ),
            crate::wire_kind::WireKind::RangeInt32 => Self::try_new_for_deserialize(
                crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI32AsInt4Range,
            ),
            crate::wire_kind::WireKind::RangeInt64 => Self::try_new_for_deserialize(
                crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize::SqlxPgTypesPgRangeI64AsInt8Range,
            ),
            crate::wire_kind::WireKind::Interval
            | crate::wire_kind::WireKind::RangeDate
            | crate::wire_kind::WireKind::RangeTimestamp
            | crate::wire_kind::WireKind::RangeTimestampTz
            | crate::wire_kind::WireKind::Timestamp
            | crate::wire_kind::WireKind::TimestampTz => Self::ImplNewForDeserializeOrTryNewForDeserialize(
                crate::pg_type_impl_new_for_deserialize_or_try_new_for_deserialize::PgTypeImplNewForDeserializeOrTryNewForDeserialize::NewForDeserialize,
            ),
            crate::wire_kind::WireKind::Bool
            | crate::wire_kind::WireKind::Bytes
            | crate::wire_kind::WireKind::Float32
            | crate::wire_kind::WireKind::Float64
            | crate::wire_kind::WireKind::Inet
            | crate::wire_kind::WireKind::Int16
            | crate::wire_kind::WireKind::Int32
            | crate::wire_kind::WireKind::Int64
            | crate::wire_kind::WireKind::Mac
            | crate::wire_kind::WireKind::Uuid => Self::Derive,
        }
    }
}
impl PgTypeDeserialize {
    const fn try_new_for_deserialize(
        value: crate::pg_type_impl_try_new_for_deserialize::PgTypeImplTryNewForDeserialize,
    ) -> Self {
        Self::ImplNewForDeserializeOrTryNewForDeserialize(
            crate::pg_type_impl_new_for_deserialize_or_try_new_for_deserialize::PgTypeImplNewForDeserializeOrTryNewForDeserialize::TryNewForDeserialize(value),
        )
    }
}
