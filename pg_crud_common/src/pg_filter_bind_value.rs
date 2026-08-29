#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum PgFilterBindValue {
    Bool(crate::pg_filter_bool::PgFilterBool),
    I64(crate::pg_filter_i64::PgFilterI64),
    Text(crate::pg_filter_text::PgFilterText),
}
