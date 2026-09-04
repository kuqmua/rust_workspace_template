#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "range keeps declaration order aligned with generated layout or processing flow"
)]
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
        match crate::schema_wire_kind::schema_wire_kind(&(*value).spec()) {
            crate::wire_kind::WireKind::RangeDate => Ok(Self::SqlxTypesChronoNaiveDateAsDate),
            crate::wire_kind::WireKind::RangeInt32 => Ok(Self::I32AsInt4),
            crate::wire_kind::WireKind::RangeInt64 => Ok(Self::I64AsInt8),
            crate::wire_kind::WireKind::RangeTimestamp => {
                Ok(Self::SqlxTypesChronoNaiveDateTimeAsTimestamp)
            }
            crate::wire_kind::WireKind::RangeTimestampTz => {
                Ok(Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz)
            }
            crate::wire_kind::WireKind::Bool
            | crate::wire_kind::WireKind::Bytes
            | crate::wire_kind::WireKind::Date
            | crate::wire_kind::WireKind::Float32
            | crate::wire_kind::WireKind::Float64
            | crate::wire_kind::WireKind::Inet
            | crate::wire_kind::WireKind::Int16
            | crate::wire_kind::WireKind::Int32
            | crate::wire_kind::WireKind::Int64
            | crate::wire_kind::WireKind::Interval
            | crate::wire_kind::WireKind::Mac
            | crate::wire_kind::WireKind::String
            | crate::wire_kind::WireKind::TimeChrono
            | crate::wire_kind::WireKind::TimeTime
            | crate::wire_kind::WireKind::Timestamp
            | crate::wire_kind::WireKind::TimestampTz
            | crate::wire_kind::WireKind::Uuid => Err(()),
        }
    }
}
impl std::fmt::Display for Range {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            naming::parameter::SelfNonNullUpperCamelCase::from_display(
                &crate::pg_type_catalog_kind::PgTypeCatalogKind::from(self)
            )
        )
    }
}
impl quote::ToTokens for Range {
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(token_stream);
    }
}
