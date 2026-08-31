#[derive(Debug, Clone, Copy, optimal_memory_layout::OptimalMemoryLayout)]
pub enum EqOperator {
    Eq,
    IsNull,
}

impl EqOperator {
    #[must_use]
    pub fn to_query_str(&self) -> crate::eq_operator_query_str::EqOperatorQueryStr {
        match &self {
            Self::Eq => crate::eq_operator_query_str::EqOperatorQueryStr::from(
                constants_str::PG_CRUD_EQUALITY_SQL_OPERATOR,
            ),
            Self::IsNull => {
                crate::eq_operator_query_str::EqOperatorQueryStr::from(constants_str::IS_NULL)
            }
        }
    }
}
