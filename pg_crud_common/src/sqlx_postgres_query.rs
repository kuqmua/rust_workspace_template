#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
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
