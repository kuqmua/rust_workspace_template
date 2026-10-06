#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct SqlxPostgresQuery<'query_lt>(
    sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
);

impl<'query_lt> AsMut<sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments>>
    for SqlxPostgresQuery<'query_lt>
{
    fn as_mut(
        &mut self,
    ) -> &mut sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments> {
        &mut self.0
    }
}

impl std::fmt::Debug for SqlxPostgresQuery<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(constants_str::SQLXPOSTGRESQUERY)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_postgres_query_debug_hides_sql_and_bound_values() {
        let sqlx_postgres_query = crate::sqlx_postgres_query::SqlxPostgresQuery::from(
            sqlx::query(constants_str::TEST_READ_QUERY_BASE).bind(constants_str::TEST_SECRET_TEXT),
        );
        assert_eq!(
            format!("{sqlx_postgres_query:?}"),
            constants_str::SQLXPOSTGRESQUERY,
        );
        assert_eq!(
            format!("{sqlx_postgres_query:#?}"),
            constants_str::SQLXPOSTGRESQUERY,
        );
    }
}
