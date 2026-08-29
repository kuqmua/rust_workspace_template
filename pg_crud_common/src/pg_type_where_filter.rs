pub trait PgTypeWhereFilter<'query_lt> {
    fn query_bind(
        self,
        query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    >;

    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        column: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;
}
