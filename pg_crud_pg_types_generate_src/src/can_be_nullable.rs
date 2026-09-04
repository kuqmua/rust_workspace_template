#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum CanBeNullable {
    False,
    True,
}
impl quote::ToTokens for crate::pg_type_catalog_kind::PgTypeCatalogKind {
    fn to_tokens(&self, token_stream: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(token_stream);
    }
}
impl From<&crate::range::Range> for crate::pg_type_catalog_kind::PgTypeCatalogKind {
    fn from(value: &crate::range::Range) -> Self {
        match value {
            crate::range::Range::I32AsInt4 => Self::I32AsInt4,
            crate::range::Range::I64AsInt8 => Self::I64AsInt8,
            crate::range::Range::SqlxTypesChronoNaiveDateAsDate => {
                Self::SqlxTypesChronoNaiveDateAsDate
            }
            crate::range::Range::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
            }
            crate::range::Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
            }
        }
    }
}
