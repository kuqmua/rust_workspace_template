#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub enum PgFilterBindValue {
    Bool(crate::domain_types::PgFilterBool),
    I64(crate::domain_types::PgFilterI64),
    Text(crate::domain_types::PgFilterText),
}
