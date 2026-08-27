#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum EqOperator {
    Eq,
    IsNull,
}

impl EqOperator {
    #[must_use]
    pub fn to_query_str(&self) -> crate::domain_types::EqOperatorQueryStr {
        match &self {
            Self::Eq => crate::domain_types::EqOperatorQueryStr::from(
                constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR,
            ),
            Self::IsNull => crate::domain_types::EqOperatorQueryStr::from(constants_str::IS_NULL),
        }
    }
}
