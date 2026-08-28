use super::*;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) enum CanBeNullable {
    False,
    True,
}
impl quote::ToTokens for PgType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::format_ident!("{}", self.to_string()).to_tokens(tokens);
    }
}
impl From<&Range> for PgType {
    fn from(v: &Range) -> Self {
        match v {
            Range::I32AsInt4 => Self::I32AsInt4,
            Range::I64AsInt8 => Self::I64AsInt8,
            Range::SqlxTypesChronoNaiveDateAsDate => Self::SqlxTypesChronoNaiveDateAsDate,
            Range::SqlxTypesChronoNaiveDateTimeAsTimestamp => {
                Self::SqlxTypesChronoNaiveDateTimeAsTimestamp
            }
            Range::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz => {
                Self::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsTimestampTz
            }
        }
    }
}
