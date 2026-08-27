pub trait PgTypeWhereFilter<'query_lt> {
    fn query_bind(
        self,
        query: crate::domain_types::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::domain_types::SqlxPostgresQuery<'query_lt>,
        crate::domain_types::SqlxPostgresQueryBindError,
    >;

    fn query_part(
        &self,
        increment: &mut dyn crate::domain_types::QueryPartIncrementMut,
        column: crate::domain_types::SqlColumnRef<'_>,
        add_operator: crate::domain_types::AddOperator,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError>;
}
