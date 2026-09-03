#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type keeps declaration order aligned with generated layout or processing flow"
)]
pub trait PgType {
    type TableType: crate::domain_types::TableTypeAlias;

    fn create_table_column_query_part(
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        pg_is_primary_key: crate::pg_is_primary_key::PgIsPrimaryKey,
    ) -> crate::query_part_fragment::QueryPartFragment;

    type Create: crate::domain_types::CreateAlias;

    fn create_query_part(
        create: &Self::Create,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    fn create_query_bind(
        create: Self::Create,
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    >;

    type Select: crate::domain_types::SelectAlias;

    fn select_query_part(
        select: &Self::Select,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    type Where: crate::domain_types::WhereAlias;
    type Read: crate::domain_types::ReadAlias
        + for<'value> sqlx::Decode<'value, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>;

    fn normalize(read: Self::Read) -> Self::Read;

    type ReadIds: crate::domain_types::ReadIdsAlias;

    fn select_only_ids_query_part(
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    type ReadInner: crate::domain_types::ReadInnerAlias;

    fn into_inner(read: Self::Read) -> Self::ReadInner;

    type Update: crate::domain_types::UpdateAlias;
    type UpdateForQuery: crate::domain_types::UpdateForQueryAlias;

    fn update_query_part(
        update_for_query: &Self::UpdateForQuery,
        update_accumulator: crate::sql_column_ref::SqlColumnRef<'_>,
        update_target: crate::sql_column_ref::SqlColumnRef<'_>,
        update_path: crate::sql_column_ref::SqlColumnRef<'_>,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    fn update_query_bind(
        update_for_query: Self::UpdateForQuery,
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'_>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    >;

    fn select_only_updated_ids_query_part(
        update_for_query: &Self::UpdateForQuery,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    >;

    fn select_only_updated_ids_query_bind<'value>(
        update_for_query: &'value Self::UpdateForQuery,
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'value>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'value>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    >;
}
