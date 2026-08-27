#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgType {
    type TableType: crate::domain_types::TableTypeAlias;

    fn create_table_column_query_part(
        column: crate::domain_types::SqlColumnRef<'_>,
        is_primary_key: crate::domain_types::IsPrimaryKey,
    ) -> crate::domain_types::QueryPartFragment;

    type Create: crate::domain_types::CreateAlias;

    fn create_query_part(
        v: &Self::Create,
        increment: &mut dyn crate::domain_types::QueryPartIncrementMut,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError>;

    fn create_query_bind(
        v: Self::Create,
        query: crate::domain_types::SqlxPostgresQuery<'_>,
    ) -> Result<
        crate::domain_types::SqlxPostgresQuery<'_>,
        crate::domain_types::SqlxPostgresQueryBindError,
    >;

    type Select: crate::domain_types::SelectAlias;

    fn select_query_part(
        v: &Self::Select,
        column: crate::domain_types::SqlColumnRef<'_>,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError>;

    type Where: crate::domain_types::WhereAlias;
    type Read: crate::domain_types::ReadAlias
        + for<'value> sqlx::Decode<'value, sqlx::Postgres>
        + sqlx::Type<sqlx::Postgres>;

    fn normalize(v: Self::Read) -> Self::Read;

    type ReadIds: crate::domain_types::ReadIdsAlias;

    fn select_only_ids_query_part(
        column: crate::domain_types::SqlColumnRef<'_>,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError>;

    type ReadInner: crate::domain_types::ReadInnerAlias;

    fn into_inner(v: Self::Read) -> Self::ReadInner;

    type Update: crate::domain_types::UpdateAlias;
    type UpdateForQuery: crate::domain_types::UpdateForQueryAlias;

    fn update_query_part(
        v: &Self::UpdateForQuery,
        update_accumulator: crate::domain_types::SqlColumnRef<'_>,
        update_target: crate::domain_types::SqlColumnRef<'_>,
        update_path: crate::domain_types::SqlColumnRef<'_>,
        increment: &mut dyn crate::domain_types::QueryPartIncrementMut,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError>;

    fn update_query_bind(
        v: Self::UpdateForQuery,
        query: crate::domain_types::SqlxPostgresQuery<'_>,
    ) -> Result<
        crate::domain_types::SqlxPostgresQuery<'_>,
        crate::domain_types::SqlxPostgresQueryBindError,
    >;

    fn select_only_updated_ids_query_part(
        v: &Self::UpdateForQuery,
        column: crate::domain_types::SqlColumnRef<'_>,
        increment: &mut dyn crate::domain_types::QueryPartIncrementMut,
    ) -> Result<crate::domain_types::QueryPartFragment, crate::domain_types::QueryPartError>;

    fn select_only_updated_ids_query_bind<'value>(
        v: &'value Self::UpdateForQuery,
        query: crate::domain_types::SqlxPostgresQuery<'value>,
    ) -> Result<
        crate::domain_types::SqlxPostgresQuery<'value>,
        crate::domain_types::SqlxPostgresQueryBindError,
    >;
}
