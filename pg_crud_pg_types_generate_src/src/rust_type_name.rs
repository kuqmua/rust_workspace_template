#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "rust type name keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, strum_macros::Display, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
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
impl From<&crate::pg_type_catalog_kind::PgTypeCatalogKind> for RustTypeName {
    fn from(value: &crate::pg_type_catalog_kind::PgTypeCatalogKind) -> Self {
        if matches!(
            value,
            crate::pg_type_catalog_kind::PgTypeCatalogKind::SqlxPgTypesPgMoneyAsMoney
        ) {
            return Self::SqlxPgTypesPgMoney;
        }
        match crate::rust_type_wire_kind::rust_type_wire_kind(&(*value).spec()) {
            crate::wire_kind::WireKind::Bool => Self::Bool,
            crate::wire_kind::WireKind::Bytes => Self::StdVecVecU8,
            crate::wire_kind::WireKind::Date => Self::SqlxTypesChronoNaiveDate,
            crate::wire_kind::WireKind::Float32 => Self::F32,
            crate::wire_kind::WireKind::Float64 => Self::F64,
            crate::wire_kind::WireKind::Inet => Self::SqlxTypesIpnetworkIpNetwork,
            crate::wire_kind::WireKind::Int16 => Self::I16,
            crate::wire_kind::WireKind::Int32 => Self::I32,
            crate::wire_kind::WireKind::Int64 => Self::I64,
            crate::wire_kind::WireKind::Interval => Self::SqlxPgTypesPgInterval,
            crate::wire_kind::WireKind::Mac => Self::SqlxTypesMacAddressMacAddress,
            crate::wire_kind::WireKind::RangeDate => {
                Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDate
            }
            crate::wire_kind::WireKind::RangeInt32 => Self::SqlxPgTypesPgRangeI32,
            crate::wire_kind::WireKind::RangeInt64 => Self::SqlxPgTypesPgRangeI64,
            crate::wire_kind::WireKind::RangeTimestamp => {
                Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTime
            }
            crate::wire_kind::WireKind::RangeTimestampTz => {
                Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtc
            }
            crate::wire_kind::WireKind::String => Self::String,
            crate::wire_kind::WireKind::TimeChrono => Self::SqlxTypesChronoNaiveTime,
            crate::wire_kind::WireKind::TimeTime => Self::SqlxTypesTimeTime,
            crate::wire_kind::WireKind::Timestamp => Self::SqlxTypesChronoNaiveDateTime,
            crate::wire_kind::WireKind::TimestampTz => {
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtc
            }
            crate::wire_kind::WireKind::Uuid => Self::SqlxTypesUuidUuid,
        }
    }
}
