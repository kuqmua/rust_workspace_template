// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgType {
    type TableType: crate::domain_types::TableTypeAlias;

    fn create_table_column_query_part(
        column: crate::sql_column_ref::SqlColumnRef<'_>,
        is_primary_key: crate::pg_is_primary_key::PgIsPrimaryKey,
    ) -> crate::query_part_fragment::QueryPartFragment;

    type Create: crate::domain_types::CreateAlias;

    fn create_query_part(
        v: &Self::Create,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    fn create_query_bind(
        v: Self::Create,
        query: crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    >;

    type Select: crate::domain_types::SelectAlias;

    fn select_query_part(
        v: &Self::Select,
        column: crate::sql_column_ref::SqlColumnRef<'_>,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    type Where: crate::domain_types::WhereAlias;
    type Read: crate::domain_types::ReadAlias
        + for<'value> sqlx::Decode<'value, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>;

    fn normalize(v: Self::Read) -> Self::Read;

    type ReadIds: crate::domain_types::ReadIdsAlias;

    fn select_only_ids_query_part(
        column: crate::sql_column_ref::SqlColumnRef<'_>,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    type ReadInner: crate::domain_types::ReadInnerAlias;

    fn into_inner(v: Self::Read) -> Self::ReadInner;

    type Update: crate::domain_types::UpdateAlias;
    type UpdateForQuery: crate::domain_types::UpdateForQueryAlias;

    fn update_query_part(
        v: &Self::UpdateForQuery,
        update_accumulator: crate::sql_column_ref::SqlColumnRef<'_>,
        update_target: crate::sql_column_ref::SqlColumnRef<'_>,
        update_path: crate::sql_column_ref::SqlColumnRef<'_>,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    fn update_query_bind(
        v: Self::UpdateForQuery,
        query: crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    >;

    fn select_only_updated_ids_query_part(
        v: &Self::UpdateForQuery,
        column: crate::sql_column_ref::SqlColumnRef<'_>,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    fn select_only_updated_ids_query_bind<'value>(
        v: &'value Self::UpdateForQuery,
        query: crate::sqlx_postgres_query::SqlxPostgresQuery<'value>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'value>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    >;
}
