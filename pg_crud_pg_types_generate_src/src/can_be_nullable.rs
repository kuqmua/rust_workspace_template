#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum CanBeNullable {
    False,
    True,
}
impl quote::ToTokens for crate::pg_type_catalog_kind::PgTypeCatalogKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(tokens);
    }
}
impl From<&crate::range::Range> for crate::pg_type_catalog_kind::PgTypeCatalogKind {
    fn from(v: &crate::range::Range) -> Self {
        match v {
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
