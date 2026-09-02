#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
pub struct SqlxPostgresQuery<'query_lt>(
    sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
);

impl<'query_lt> SqlxPostgresQuery<'query_lt> {
    pub fn into_inner(
        self,
    ) -> sqlx::query::Query<'query_lt, sqlx::Postgres, sqlx::postgres::PgArguments> {
        self.0
    }
}

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::SQLXPOSTGRESQUERY).finish()
    }
}
