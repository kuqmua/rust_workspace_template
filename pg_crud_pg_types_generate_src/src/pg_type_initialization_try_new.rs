#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type initialization try new keeps declaration order aligned with generated layout or processing flow"
)]
#[derive(Debug, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
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
impl TryFrom<&crate::pg_type_catalog_kind::PgTypeCatalogKind> for PgTypeInitializationTryNew {
    type Error = ();
    fn try_from(
        value: &crate::pg_type_catalog_kind::PgTypeCatalogKind,
    ) -> Result<Self, Self::Error> {
        match crate::schema_wire_kind::schema_wire_kind(&(*value).spec()) {
            crate::wire_kind::WireKind::Date => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
            crate::wire_kind::WireKind::Float64 => Ok(Self::F64AsFloat8),
            crate::wire_kind::WireKind::RangeDate => {
                Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateAsDateRange)
            }
            crate::wire_kind::WireKind::RangeInt32 => Ok(Self::SqlxPgTypesPgRangeI32AsInt4Range),
            crate::wire_kind::WireKind::RangeInt64 => Ok(Self::SqlxPgTypesPgRangeI64AsInt8Range),
            crate::wire_kind::WireKind::RangeTimestamp => {
                Ok(Self::SqlxPgTypesPgRangeSqlxTypesChronoNaiveDateTimeAsTimestampRange)
            }
            crate::wire_kind::WireKind::RangeTimestampTz => Ok(
                Self::SqlxPgTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTzRange,
            ),
            crate::wire_kind::WireKind::String => Ok(Self::StringAsText),
            crate::wire_kind::WireKind::TimeChrono => Ok(Self::SqlxTypesChronoNaiveTimeAsTime),
            crate::wire_kind::WireKind::TimeTime => Ok(Self::SqlxTypesTimeTimeAsTime),
            crate::wire_kind::WireKind::Timestamp => {
                Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp)
            }
            crate::wire_kind::WireKind::TimestampTz => {
                Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz)
            }
            crate::wire_kind::WireKind::Bool
            | crate::wire_kind::WireKind::Bytes
            | crate::wire_kind::WireKind::Float32
            | crate::wire_kind::WireKind::Inet
            | crate::wire_kind::WireKind::Int16
            | crate::wire_kind::WireKind::Int32
            | crate::wire_kind::WireKind::Int64
            | crate::wire_kind::WireKind::Interval
            | crate::wire_kind::WireKind::Mac
            | crate::wire_kind::WireKind::Uuid => Err(()),
        }
    }
}
impl From<&PgTypeInitializationTryNew> for crate::pg_type_catalog_kind::PgTypeCatalogKind {
    fn from(value: &PgTypeInitializationTryNew) -> Self {
        match value {
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
