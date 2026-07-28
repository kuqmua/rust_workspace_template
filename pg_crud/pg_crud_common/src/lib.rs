mod advisory_lock;
mod batch_validation;
mod bind_index;
pub mod bounded_btree_map;
pub mod bounded_unique_vec;
pub mod bounded_vec;
mod cardinality;
mod cursor;
mod date_sql_filter;
mod db_schema_conformance;
mod errors;
mod filter_bind_plan;
mod finite_f64;
mod invariants;
mod list_total;
mod operation_budget;
mod operational_invariants;
mod order_preserving_deduplication;
mod pagination;
mod patch_field;
mod pg_error;
mod query_fragment;
mod read_query_plan;
mod rollback;
mod sql_identifier;
mod sql_like_pattern;
pub use advisory_lock::{
    PgRelationCapacityError, PgRelationCapacityMaximum, PgRelationLockError,
    PgRelationLockNamespace, PgRelationResourceId, PgRelationResourceIds, PgRelationRowCount,
    SqlxPgRelationLockConnectionRef, SqlxPgRelationLockError, lock_pg_relation_resources,
    validate_pg_relation_capacity,
};
pub use batch_validation::{
    BatchDuplicatePolicy, BatchInvalidItemCount, BatchInvalidItems, BatchProcessedItemCount,
    BatchStoppedEarly, BatchValidationReport, StdBatchRecords, validate_batch_by_key,
};
pub use bind_index::{
    QueryPartIncrement, QueryPartIncrementMut, increment_checked_add_one_returning_increment,
};
pub use cardinality::{
    DuplicateCandidates, DuplicateIdx, first_duplicate_idx, first_duplicate_idx_by_hash,
    take_fst_dup, take_fst_dup_by_hash,
};
pub use cursor::{
    CursorCodec, CursorCodecBuildError, CursorDecodeError, CursorEncodeError, CursorMaximumLength,
    CursorPaginationUsage, CursorPayload, CursorPayloadError, CursorSigningKey,
    CursorSigningKeyError, OffsetPaginationPresence, SignedCursor, SignedCursorError,
    SignedCursorPresence,
};
pub use date_sql_filter::{
    ChronoUtcDateTimeRef, ChronoUtcDateTimes, DateFilterBounds, DateSqlFilter, DateSqlFilterError,
    StdDateSqlBindStart, build_date_sql_filter,
};
pub use db_schema_conformance::{
    DbCatalogSnapshot, DbColumnContractSnapshot, DbColumnContractSnapshots,
    DbColumnHasServerDefault, DbColumnNullable, DbColumnSnapshot, DbColumnSnapshots, DbColumnSpec,
    DbColumnSpecs, DbDefaultSpec, DbDefaultSpecs, DbExtendedTableSchema, DbKeyContractSnapshot,
    DbKeyContractSnapshots, DbKeySpec, DbKeySpecs, DbObjectKind, DbObjectSnapshot,
    DbObjectSnapshots, DbObjectSpec, DbObjectSpecs, DbSchemaConformanceError, DbSchemaNameRef,
    DbSchemaText, DbSchemaTextError, DbSchemaTextTryFromStringError, DbSchemaTexts,
    DbStaticSchemaText, DbStaticSchemaTexts, DbTableNameRef, DbTableSchema, DbTableSnapshot,
    PgColumnSchema, SqlxDbSchemaInspectionError, SqlxPgPoolRef, inspect_postgres_catalog,
    inspect_postgres_table, validate_generated_postgres_table, validate_postgres_catalog,
    validate_postgres_table_extensions, validate_postgres_table_schema,
};
pub use errors::{
    PgCrudStringWrapperTryFromStringError, QueryPartError, QueryPartErrorWithSerde,
    SqlxPostgresQueryBindError, mk_query_bind_err,
};
pub use filter_bind_plan::{
    FilterBindPlan, PgFilterBindValue, PgFilterBool, PgFilterI64, PgFilterText, PgFilterTextError,
};
pub use finite_f64::{
    FiniteF64, FiniteF64Error, PositiveFiniteF64, PositiveFiniteF64Error, UnitIntervalF64,
    UnitIntervalF64Error,
};
pub use invariants::{
    BulkMutationOutcome, DataInvariantViolation, PaginationTotal, validate_bulk_atomicity,
    validate_migration_idempotency, validate_pagination_invariants,
};
pub use list_total::{
    ListItems, ListOffset, ListPage, ListRows, ListRowsPresence, ListTotal, ListTotalError,
    ListTotalSource, WindowTotalPresence, list_total_source, run_list_with_total,
};
pub use operation_budget::{
    OperationBudget, OperationBudgetExceeded, OperationCount, validate_operation_budget,
};
pub use operational_invariants::{
    PgCounterReconciliation, PgCounterValue, PgOperationalLimit, PgOperationalLimitError,
    PgOperationalLimitUpdateAuthority, PgScopedForeignKey, PgScopedForeignKeyError,
    PgScopedForeignKeyOnDelete, PgSqlIdentifiers, build_pg_scoped_foreign_key_clause,
    reconcile_pg_counter, resolve_pg_operational_limit_update,
};
pub use order_preserving_deduplication::{
    OrderPreservingValues, SliceOrdering, classify_slice_ordering,
    deduplicate_preserving_order_by_key,
};
pub use pagination::{
    PaginationEnd, PaginationLimit, PaginationOffset, PaginationPolicy, PaginationStart,
};
pub use patch_field::PatchField;
pub use pg_error::{PgErrorKind, SqlxPgErrorRef, classify_pg_error};
pub use query_fragment::{QueryPartFragment, SqlColumnRef};
pub use read_query_plan::{
    QuerySortOrder, ReadQueryPlan, ReadQueryPlanError, StdReadQueryBindIndex,
    build_stable_read_query_plan,
};
pub use rollback::TransactionFailure;
pub use sql_identifier::{
    SqlIdentifier, SqlIdentifierError, SqlIdentifiers, SqlQualifiedIdentifier, SqlSelectBuilder,
};
pub use sql_like_pattern::{
    SqlLikeInputRef, SqlLikeMatchMode, SqlLikePattern, SqlLikePatternError, build_sql_like_pattern,
};
pub(crate) const PG_CRUD_STRING_WRAPPER_MAX_LEN: usize = 1_048_576;
const NOT_EMPTY_UNIQUE_VEC_MAX_LEN: usize = 10_000usize;
#[derive(Clone, Debug, Eq, PartialEq, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct AllEnumVariants<T>(Vec<T>);
pub trait AllEnumVariantsArrayDefaultSomeOneElement: Sized {
    fn all_variants_default_some_one_element() -> AllEnumVariants<Self>;
}
pub trait AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize: Sized {
    fn all_variants_default_some_one_element_max_page_size() -> AllEnumVariants<Self>;
}
pub trait DefaultSomeOneElement: Sized {
    fn default_some_one_element() -> Self;
}
pub trait DefaultSomeOneElementMaxPageSize: Sized {
    fn default_some_one_element_max_page_size() -> Self;
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::DebugDisplay,
)]
pub enum Operator {
    And,
    AndNot,
    #[default]
    Or,
    OrNot,
}
impl DefaultSomeOneElement for Operator {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}
impl Operator {
    #[must_use]
    pub fn to_query_part(&self, add_operator: AddOperator) -> QueryPartFragment {
        let fragment = match (bool::from(add_operator), *self) {
            (false, Self::And | Self::Or) => {
                return match QueryPartFragment::try_from(String::new()) {
                    Ok(value) => value,
                    Err(error) => QueryPartFragment::from(error),
                };
            }
            (false, Self::AndNot | Self::OrNot) => str_constants::NOT,
            (true, Self::And) => str_constants::AND_ALT,
            (true, Self::AndNot) => str_constants::AND_NOT,
            (true, Self::Or) => str_constants::OR,
            (true, Self::OrNot) => str_constants::OR_NOT,
        };
        match QueryPartFragment::try_from(String::from(fragment)) {
            Ok(v) => v,
            Err(error) => QueryPartFragment::from(error),
        }
    }
}
#[cfg(test)]
mod tests_operator_to_query_part {
    #[test]
    fn to_query_part_includes_operator_when_requested() {
        assert_eq!(
            super::Operator::And
                .to_query_part(super::AddOperator::from(true))
                .as_ref(),
            format!("{} ", naming::AndSnakeCase)
        );
        assert_eq!(
            super::Operator::Or
                .to_query_part(super::AddOperator::from(true))
                .as_ref(),
            format!("{} ", naming::OrSnakeCase)
        );
    }
    #[test]
    fn to_query_part_includes_not_suffix_for_negative_variants() {
        assert_eq!(
            super::Operator::AndNot
                .to_query_part(super::AddOperator::from(true))
                .as_ref(),
            format!("{} {} ", naming::AndSnakeCase, naming::NotSnakeCase)
        );
        assert_eq!(
            super::Operator::OrNot
                .to_query_part(super::AddOperator::from(true))
                .as_ref(),
            format!("{} {} ", naming::OrSnakeCase, naming::NotSnakeCase)
        );
    }
    #[test]
    fn to_query_part_omits_operator_when_disabled_and_keeps_not_only_for_negative_variants() {
        assert_eq!(
            super::Operator::And
                .to_query_part(super::AddOperator::from(false))
                .as_ref(),
            ""
        );
        assert_eq!(
            super::Operator::Or
                .to_query_part(super::AddOperator::from(false))
                .as_ref(),
            ""
        );
        assert_eq!(
            super::Operator::AndNot
                .to_query_part(super::AddOperator::from(false))
                .as_ref(),
            format!("{} ", naming::NotSnakeCase)
        );
        assert_eq!(
            super::Operator::OrNot
                .to_query_part(super::AddOperator::from(false))
                .as_ref(),
            format!("{} ", naming::NotSnakeCase)
        );
    }
}
#[derive(Debug, Clone, Copy, PartialEq, optml::Optml)]
pub enum PgTypeGreaterThanVariant {
    EqNotGreaterThan,
    GreaterThan,
    NotGreaterThan,
}
impl PgTypeGreaterThanVariant {
    #[must_use]
    pub const fn operator(&self) -> Operator {
        match *self {
            Self::GreaterThan => Operator::Or,
            Self::NotGreaterThan | Self::EqNotGreaterThan => Operator::OrNot,
        }
    }
}
pg_crud_common_macros::trait_alias!(DebugClonePartialEqAlias = std::fmt::Debug + Clone + PartialEq);
pg_crud_common_macros::trait_alias!(
    DebugClonePartialEqSerializeAlias = DebugClonePartialEqAlias + serde::Serialize
);
pg_crud_common_macros::trait_alias!(DebugClonePartialEqSerdeAlias = DebugClonePartialEqSerializeAlias + for<'__> serde::Deserialize<'__>);
pg_crud_common_macros::trait_alias!(
    DebugClonePartialEqSerdeDefaultSomeOneAlias =
        DebugClonePartialEqSerdeAlias + DefaultSomeOneElement
);
pg_crud_common_macros::trait_alias!(SqlxEncodePgSqlxTypePgAlias = for<'__> sqlx::Encode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>);
pg_crud_common_macros::trait_alias!(
    UtoipaToSchemaAndSchemarsJsonSchemaAlias = for<'__> utoipa::ToSchema<'__> + schemars::JsonSchema
);
pg_crud_common_macros::trait_alias!(TableTypeAlias = DebugClonePartialEqSerdeDefaultSomeOneAlias);
pg_crud_common_macros::trait_alias!(CreateAlias = DebugClonePartialEqSerdeDefaultSomeOneAlias);
pg_crud_common_macros::trait_alias!(
    CreateForQueryAlias = DebugClonePartialEqSerializeAlias + SqlxEncodePgSqlxTypePgAlias
);
pg_crud_common_macros::trait_alias!(SelectAlias = DebugClonePartialEqSerdeDefaultSomeOneAlias);
pg_crud_common_macros::trait_alias!(WhereAlias = DebugClonePartialEqSerdeAlias + for<'__> PgTypeWhereFilter<'__>);
pg_crud_common_macros::trait_alias!(ReadAlias = DebugClonePartialEqSerdeAlias);
pg_crud_common_macros::trait_alias!(ReadIdsAlias = DebugClonePartialEqSerdeAlias);
pg_crud_common_macros::trait_alias!(ReadInnerAlias = DebugClonePartialEqAlias);
pg_crud_common_macros::trait_alias!(UpdateAlias = DebugClonePartialEqSerdeDefaultSomeOneAlias);
pg_crud_common_macros::trait_alias!(UpdateForQueryAlias = DebugClonePartialEqSerializeAlias);
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgType {
    //difference between Create and TableType - Create may not contain generated by pg id
    type TableType: TableTypeAlias;
    fn create_table_column_query_part(
        column: SqlColumnRef<'_>,
        is_primary_key: IsPrimaryKey,
    ) -> QueryPartFragment;
    type Create: CreateAlias;
    fn create_query_part(
        v: &Self::Create,
        increment: &mut dyn QueryPartIncrementMut,
    ) -> Result<QueryPartFragment, QueryPartError>;
    fn create_query_bind(
        v: Self::Create,
        query: SqlxPostgresQuery<'_>,
    ) -> Result<SqlxPostgresQuery<'_>, SqlxPostgresQueryBindError>;
    type Select: SelectAlias;
    fn select_query_part(
        v: &Self::Select,
        column: SqlColumnRef<'_>,
    ) -> Result<QueryPartFragment, QueryPartError>;
    type Where: WhereAlias;
    type Read: ReadAlias + for<'__> sqlx::Decode<'__, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>;
    fn normalize(v: Self::Read) -> Self::Read;
    type ReadIds: ReadIdsAlias;
    fn select_only_ids_query_part(
        column: SqlColumnRef<'_>,
    ) -> Result<QueryPartFragment, QueryPartError>;
    type ReadInner: ReadInnerAlias;
    fn into_inner(v: Self::Read) -> Self::ReadInner;
    type Update: UpdateAlias;
    type UpdateForQuery: UpdateForQueryAlias;
    fn update_query_part(
        v: &Self::UpdateForQuery,
        update_accumulator: SqlColumnRef<'_>,
        update_target: SqlColumnRef<'_>,
        update_path: SqlColumnRef<'_>,
        increment: &mut dyn QueryPartIncrementMut,
    ) -> Result<QueryPartFragment, QueryPartError>;
    fn update_query_bind(
        v: Self::UpdateForQuery,
        query: SqlxPostgresQuery<'_>,
    ) -> Result<SqlxPostgresQuery<'_>, SqlxPostgresQueryBindError>;
    fn select_only_updated_ids_query_part(
        v: &Self::UpdateForQuery,
        column: SqlColumnRef<'_>,
        increment: &mut dyn QueryPartIncrementMut,
    ) -> Result<QueryPartFragment, QueryPartError>;
    fn select_only_updated_ids_query_bind<'lt>(
        v: &'lt Self::UpdateForQuery,
        query: SqlxPostgresQuery<'lt>,
    ) -> Result<SqlxPostgresQuery<'lt>, SqlxPostgresQueryBindError>;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgTypePrimaryKey {
    type PgType: PgType;
    type TableType: TableTypeAlias + PartialOrd;
    fn read_ids_into_table_type(
        v: <Self::PgType as PgType>::ReadIds,
    ) -> <Self::PgType as PgType>::TableType;
    fn read_ids_into_read(v: <Self::PgType as PgType>::ReadIds) -> <Self::PgType as PgType>::Read;
    fn read_ids_into_update(
        v: <Self::PgType as PgType>::ReadIds,
    ) -> <Self::PgType as PgType>::Update;
    fn read_into_table_type(
        v: <Self::PgType as PgType>::Read,
    ) -> <Self::PgType as PgType>::TableType;
}
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgTypeNotPrimaryKey {
    type PgType: PgType;
    type Create: CreateAlias + SqlxEncodePgSqlxTypePgAlias;
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
pub trait PgTypeTestCases {
    type PgType: PgType;
    type Select: SelectAlias + DefaultSomeOneElementMaxPageSize;
    #[must_use]
    fn optional_vec_create() -> Option<Vec<<Self::PgType as PgType>::Create>> {
        None
    }
    fn read_ids_to_2_dimensions_vec_read_inner(
        read_ids: &<Self::PgType as PgType>::ReadIds,
    ) -> Vec<Vec<<Self::PgType as PgType>::ReadInner>>;
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        v: <Self::PgType as PgType>::ReadInner,
    ) -> <Self::PgType as PgType>::Read;
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        v: <Self::PgType as PgType>::ReadInner,
    ) -> <Self::PgType as PgType>::Update;
    fn update_to_read_ids(
        v: &<Self::PgType as PgType>::Update,
    ) -> <Self::PgType as PgType>::ReadIds;
    fn read_ids_to_optional_v_read_default_some_one_element(
        _v: &<Self::PgType as PgType>::ReadIds,
    ) -> Option<V<<Self::PgType as PgType>::Read>> {
        None
    }
    fn previous_read_and_optional_update_into_read(
        read: <Self::PgType as PgType>::Read,
        optional_update: Option<<Self::PgType as PgType>::Update>,
    ) -> <Self::PgType as PgType>::Read;
    fn read_ids_and_create_into_read(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> <Self::PgType as PgType>::Read;
    fn read_ids_and_create_into_optional_v_read(
        _read_ids: <Self::PgType as PgType>::ReadIds,
        _create: <Self::PgType as PgType>::Create,
    ) -> Option<V<<Self::PgType as PgType>::Read>> {
        None
    }
    fn read_ids_and_create_into_table_type(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> <Self::PgType as PgType>::TableType;
    fn read_ids_and_create_into_where_eq(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> <Self::PgType as PgType>::Where;
    fn read_ids_and_create_into_vec_where_eq_using_fields(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> NotEmptyUniqueVec<<Self::PgType as PgType>::Where>;
    fn read_ids_and_create_into_optional_vec_where_eq_to_field(
        _read_ids: <Self::PgType as PgType>::ReadIds,
        _create: <Self::PgType as PgType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PgType as PgType>::Where>> {
        None
    }
    fn create_into_pg_type_optional_vec_where_dimension_one_eq(
        _create: <Self::PgType as PgType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PgType as PgType>::Where>> {
        None
    }
    #[must_use]
    fn pg_type_optional_vec_where_greater_than_test()
    -> Option<NotEmptyUniqueVec<PgTypeGreaterThanTest<Self::PgType>>> {
        None
    }
    fn read_ids_and_table_type_into_pg_type_optional_where_greater_than(
        _greater_than_variant: PgTypeGreaterThanVariant,
        _read_ids: <Self::PgType as PgType>::ReadIds,
        _table_type: <Self::PgType as PgType>::TableType,
    ) -> Option<<Self::PgType as PgType>::Where> {
        None
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, optml::Optml)]
pub struct PgTypeGreaterThanTest<T: PgType> {
    pub greater_than: <T as PgType>::TableType,
    pub create: <T as PgType>::Create,
    pub variant: PgTypeGreaterThanVariant,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optml::Optml)]
pub struct PgTypeLenGreaterThanTest<T: PgType> {
    pub create: <T as PgType>::Create,
    pub variant: PgTypeGreaterThanVariant,
    pub len_greater_than: UnsignedPartOfI32,
}
#[derive(newtype::FromInner)]
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
        f.debug_tuple(str_constants::SQLXPOSTGRESQUERY).finish()
    }
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct AddOperator(bool);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct IsPrimaryKey(bool);
pub trait PgTypeWhereFilter<'query_lt> {
    fn query_bind(
        self,
        query: SqlxPostgresQuery<'query_lt>,
    ) -> Result<SqlxPostgresQuery<'query_lt>, SqlxPostgresQueryBindError>;
    fn query_part(
        &self,
        increment: &mut dyn QueryPartIncrementMut,
        column: SqlColumnRef<'_>,
        add_operator: AddOperator,
    ) -> Result<QueryPartFragment, QueryPartError>;
}
//todo custom deserialization - must not contain more than one element
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
    newtype::FromInner,
)]
#[serde(from = "Option<NotEmptyUniqueVec<T>>")]
pub struct NullableJsonObjPgTypeWhereFilter<
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'lt> PgTypeWhereFilter<'lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
>(Option<NotEmptyUniqueVec<T>>);
impl<T> NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    #[must_use]
    pub const fn as_ref(&self) -> Option<&NotEmptyUniqueVec<T>> {
        self.0.as_ref()
    }
    #[must_use]
    pub fn into_option(self) -> Option<NotEmptyUniqueVec<T>> {
        self.0
    }
}
impl<'query_lt, T> PgTypeWhereFilter<'query_lt> for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn query_bind(
        self,
        query: SqlxPostgresQuery<'query_lt>,
    ) -> Result<SqlxPostgresQuery<'query_lt>, SqlxPostgresQueryBindError> {
        match self.into_option() {
            Some(v) => v.query_bind(query),
            None => Ok(query), //todo maybe wrong
        }
    }
    fn query_part(
        &self,
        increment: &mut dyn QueryPartIncrementMut,
        column: SqlColumnRef<'_>,
        add_operator: AddOperator,
    ) -> Result<QueryPartFragment, QueryPartError> {
        self.as_ref().map_or_else(
            || {
                let mut query_part = String::with_capacity(16);
                if std::fmt::Write::write_fmt(&mut query_part, format_args!("{column} = 'null'"))
                    .is_err()
                {
                    return Err(QueryPartError::WriteIntoBuffer {
                        location: location_macros::location!(),
                    });
                }
                Ok(QueryPartFragment::try_from(query_part).unwrap_or_else(QueryPartFragment::from))
            },
            |v| v.query_part(increment, column, add_operator),
        )
    }
}
impl<T> to_err_string::ToErrString for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn to_err_string(&self) -> to_err_string::ErrorText {
        to_err_string::ErrorText::try_from(format!("{self:#?}"))
            .unwrap_or_else(to_err_string::ErrorText::from)
    }
}
impl<T> AllEnumVariantsArrayDefaultSomeOneElement for NullableJsonObjPgTypeWhereFilter<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn all_variants_default_some_one_element() -> AllEnumVariants<Self> {
        vec![Self(
            Some(DefaultSomeOneElement::default_some_one_element()),
        )]
        .into()
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, schemars::JsonSchema, optml::Optml)]
pub struct PgTypeWhere<T> {
    v: NotEmptyUniqueVec<T>,
    operator: Operator,
}
impl<'schema_lt, T: utoipa::ToSchema<'schema_lt>> utoipa::ToSchema<'schema_lt> for PgTypeWhere<T> {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::PG_CRUD_PG_TYPE_WHERE_SCHEMA_NAME,
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    str_constants::PG_CRUD_V_FIELD,
                    <NotEmptyUniqueVec<T> as utoipa::ToSchema>::schema().1,
                )
                .property(
                    str_constants::PG_CRUD_OPERATOR_FIELD,
                    <Operator as utoipa::ToSchema>::schema().1,
                )
                .required(str_constants::PG_CRUD_V_FIELD)
                .required(str_constants::PG_CRUD_OPERATOR_FIELD)
                .build()
                .into(),
        )
    }
}
impl<T: PartialEq + Clone> PgTypeWhere<T> {
    #[must_use]
    pub const fn get_operator(&self) -> &Operator {
        &self.operator
    }
    #[must_use]
    pub const fn new(operator: Operator, v: NotEmptyUniqueVec<T>) -> Self {
        Self { v, operator }
    }
    pub fn try_new(
        operator: Operator,
        v: DuplicateCandidates<T>,
    ) -> Result<Self, NotEmptyUniqueVecTryNewError<T>> {
        match NotEmptyUniqueVec::try_new(v) {
            Ok(v0) => Ok(Self { operator, v: v0 }),
            Err(error) => Err(error),
        }
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
        serde::Deserialize<'de> for PgTypeWhere<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                f0,
                f1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    _serde::__private228::Formatter::write_str(
                        __f,
                        str_constants::PG_CRUD_FIELD_IDENTIFIER,
                    )
                }
                fn visit_u64<__E>(self, v: u64) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        0u64 => Ok(__Field::f0),
                        1u64 => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, v: &str) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        str_constants::PG_CRUD_OPERATOR_FIELD => Ok(__Field::f0),
                        str_constants::PG_CRUD_V_FIELD => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, v: &[u8]) -> Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match v {
                        b"operator" => Ok(__Field::f0),
                        b"v" => Ok(__Field::f1),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                {
                    serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de, PgTypeWhere> {
                marker: _serde::__private228::PhantomData<PgTypeWhere>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeWhere<T>;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    std::fmt::Formatter::write_str(
                        __f,
                        str_constants::PG_CRUD_PG_TYPE_WHERE_STRUCT_NAME,
                    )
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<Operator>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &str_constants::PG_CRUD_PG_TYPE_WHERE_EXPECTING,
                        ));
                    };
                    let Some(f1) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            1usize,
                            &str_constants::PG_CRUD_PG_TYPE_WHERE_EXPECTING,
                        ));
                    };
                    match PgTypeWhere::try_new(f0, f1.into()) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_map<__A>(self, mut __map: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut f0: Option<Operator> = None;
                    let mut f1: Option<Vec<T>> = None;
                    while let Some(__k) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __k {
                            __Field::f0 => {
                                if Option::is_some(&f0) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            str_constants::PG_CRUD_OPERATOR_FIELD,
                                        ),
                                    );
                                }
                                f0 = Some(_serde::de::MapAccess::next_value::<Operator>(
                                    &mut __map,
                                )?);
                            }
                            __Field::f1 => {
                                if Option::is_some(&f1) {
                                    return Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            str_constants::PG_CRUD_V_FIELD,
                                        ),
                                    );
                                }
                                f1 = Some(_serde::de::MapAccess::next_value::<Vec<T>>(&mut __map)?);
                            }
                            __Field::__ignore => {
                                let _: serde::de::IgnoredAny =
                                    _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                        &mut __map,
                                    )?;
                            }
                        }
                    }
                    let f0_v = match f0 {
                        Some(v) => v,
                        None => _serde::__private228::de::missing_field(
                            str_constants::PG_CRUD_OPERATOR_FIELD,
                        )?,
                    };
                    let f1_v = match f1 {
                        Some(v) => v,
                        None => {
                            _serde::__private228::de::missing_field(str_constants::PG_CRUD_V_FIELD)?
                        }
                    };
                    match PgTypeWhere::try_new(f0_v, f1_v.into()) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            serde::Deserializer::deserialize_struct(
                __deserializer,
                str_constants::PG_CRUD_PG_TYPE_WHERE_SCHEMA_NAME,
                str_constants::PG_CRUD_SERDE_PG_TYPE_WHERE_FIELDS,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<T>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<'query_lt, T: PgTypeWhereFilter<'query_lt>> PgTypeWhereFilter<'query_lt> for PgTypeWhere<T> {
    fn query_bind(
        self,
        query: SqlxPostgresQuery<'query_lt>,
    ) -> Result<SqlxPostgresQuery<'query_lt>, SqlxPostgresQueryBindError> {
        self.v
            .0
            .into_iter()
            .try_fold(query, |accumulator_query, element| {
                PgTypeWhereFilter::query_bind(element, accumulator_query)
            })
    }
    fn query_part(
        &self,
        increment: &mut dyn QueryPartIncrementMut,
        column: SqlColumnRef<'_>,
        add_operator: AddOperator,
    ) -> Result<QueryPartFragment, QueryPartError> {
        let operator_query_part = self.operator.to_query_part(add_operator);
        let mut query_part = String::with_capacity(
            operator_query_part
                .as_ref()
                .len()
                .saturating_add(self.v.0.len().saturating_mul(32))
                .saturating_add(2),
        );
        query_part.push_str(operator_query_part.as_ref());
        query_part.push('(');
        let mut add_operator_inner_handle = AddOperator::from(false);
        let mut is_first = true;
        self.v.0.iter().try_for_each(|element| {
            let v = PgTypeWhereFilter::query_part(
                element,
                increment,
                column,
                add_operator_inner_handle,
            )?;
            if is_first {
                is_first = false;
            } else {
                query_part.push(' ');
            }
            query_part.push_str(v.as_ref());
            add_operator_inner_handle = AddOperator::from(true);
            Ok::<(), QueryPartError>(())
        })?;
        query_part.push(')');
        Ok(QueryPartFragment::try_from(query_part).unwrap_or_else(QueryPartFragment::from))
    }
}
impl<T: std::fmt::Debug + PartialEq + Clone + AllEnumVariantsArrayDefaultSomeOneElement>
    DefaultSomeOneElement for PgTypeWhere<T>
{
    fn default_some_one_element() -> Self {
        Self {
            operator: DefaultSomeOneElement::default_some_one_element(),
            v: DefaultSomeOneElement::default_some_one_element(),
        }
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    utoipa::ToSchema,
    strum_macros::EnumString,
    optml::Optml,
)]
#[strum(serialize_all = "snake_case")]
pub enum Order {
    #[serde(rename(serialize = "asc", deserialize = "asc"))]
    #[default]
    Asc,
    #[serde(rename(serialize = "desc", deserialize = "desc"))]
    Desc,
}
impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "{}", naming::AscUpperCamelCase),
            Self::Desc => write!(f, "{}", naming::DescUpperCamelCase),
        }
    }
}
impl DefaultSomeOneElement for Order {
    fn default_some_one_element() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::Display)]
pub struct OrderSnakeCaseStr(String);
impl From<PgCrudStringWrapperTryFromStringError> for OrderSnakeCaseStr {
    fn from(value: PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for OrderSnakeCaseStr {
    type Error = PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, optml::Optml, newtype::Display)]
pub struct OrderUpperCamelCaseStr(String);
impl From<PgCrudStringWrapperTryFromStringError> for OrderUpperCamelCaseStr {
    fn from(value: PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for OrderUpperCamelCaseStr {
    type Error = PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > PG_CRUD_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: PG_CRUD_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
    }
}
impl Order {
    #[must_use]
    pub fn to_snake_case_str(&self) -> OrderSnakeCaseStr {
        OrderSnakeCaseStr::try_from(naming_common::DisplayToSnakeCaseStr::case(self))
            .unwrap_or_else(OrderSnakeCaseStr::from)
    }
    #[must_use]
    pub fn to_upper_camel_case_str(&self) -> OrderUpperCamelCaseStr {
        OrderUpperCamelCaseStr::try_from(naming_common::DisplayToUpperCamelCaseStr::case(self))
            .unwrap_or_else(OrderUpperCamelCaseStr::from)
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize, optml::Optml)]
pub struct OrderBy<ColumnGeneric> {
    pub column: ColumnGeneric,
    pub order: Option<Order>,
}
impl<'schema_lt, ColumnGeneric: utoipa::ToSchema<'schema_lt>> utoipa::ToSchema<'schema_lt>
    for OrderBy<ColumnGeneric>
{
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::ORDERBY,
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    str_constants::COLUMN,
                    <ColumnGeneric as utoipa::ToSchema>::schema().1,
                )
                .property(
                    str_constants::ORDER,
                    <Order as utoipa::ToSchema>::schema().1,
                )
                .required(str_constants::COLUMN)
                .build()
                .into(),
        )
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct PaginationBase {
    limit: PaginationLimit,
    offset: PaginationOffset,
}
impl PaginationBase {
    #[must_use]
    pub fn end(&self) -> PaginationEnd {
        PaginationEnd::from(self.offset.get().saturating_add(self.limit.get()))
    }
    #[must_use]
    pub fn new_unchecked<LimitTy, OffsetTy>(limit: LimitTy, offset: OffsetTy) -> Self
    where
        LimitTy: Into<PaginationLimit>,
        OffsetTy: Into<PaginationOffset>,
    {
        Self {
            limit: limit.into(),
            offset: offset.into(),
        }
    }
    #[must_use]
    pub fn start(&self) -> PaginationStart {
        PaginationStart::from(self.offset.get())
    }
}
impl<'query_lt> PgTypeWhereFilter<'query_lt> for PaginationBase {
    fn query_bind(
        self,
        mut query: SqlxPostgresQuery<'query_lt>,
    ) -> Result<SqlxPostgresQuery<'query_lt>, SqlxPostgresQueryBindError> {
        if let Err(error) = query.as_mut().try_bind(self.limit.get()) {
            return Err(SqlxPostgresQueryBindError::from(error));
        }
        if let Err(error) = query.as_mut().try_bind(self.offset.get()) {
            return Err(SqlxPostgresQueryBindError::from(error));
        }
        Ok(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn QueryPartIncrementMut,
        _: SqlColumnRef<'_>,
        _: AddOperator,
    ) -> Result<QueryPartFragment, QueryPartError> {
        let limit_increment = match increment_checked_add_one_returning_increment(increment) {
            Ok(v) => v,
            Err(error) => {
                return Err(error);
            }
        };
        let offset_increment = match increment_checked_add_one_returning_increment(increment) {
            Ok(v) => v,
            Err(error) => {
                return Err(error);
            }
        };
        let mut query_part = String::with_capacity(32);
        if std::fmt::Write::write_fmt(
            &mut query_part,
            format_args!("limit ${limit_increment} offset ${offset_increment}"),
        )
        .is_err()
        {
            return Err(QueryPartError::WriteIntoBuffer {
                location: location_macros::location!(),
            });
        }
        Ok(QueryPartFragment::try_from(query_part)?)
    }
}
impl Default for PaginationBase {
    fn default() -> Self {
        Self::new_unchecked(PaginationPolicy::standard().default_limit().get(), 0)
    }
}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema, optml::Optml)]
struct PaginationStartsWithZeroRaw {
    limit: PaginationLimit,
    offset: PaginationOffset,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
#[serde(try_from = "PaginationStartsWithZeroRaw")]
#[derive(newtype::FromInner)]
pub struct PaginationStartsWithZero(PaginationBase);

#[location::errors_with_location]
#[derive(
    Debug, serde::Serialize, serde::Deserialize, thiserror::Error, location::Location, optml::Optml,
)]
pub enum PaginationStartsWithZeroTryNewError {
    LimitIsLessThanOrEqToZero {
        #[eo_to_err_string_serde]
        limit: PaginationLimit,
    },
    OffsetIsLessThanZero {
        #[eo_to_err_string_serde]
        offset: PaginationOffset,
    },
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_err_string_serde]
        limit: PaginationLimit,
        #[eo_to_err_string_serde]
        offset: PaginationOffset,
    },
}
impl PaginationStartsWithZero {
    #[must_use]
    pub fn end(&self) -> PaginationEnd {
        self.0.end()
    }
    #[must_use]
    pub fn start(&self) -> PaginationStart {
        self.0.start()
    }
    pub fn try_new<LimitTy, OffsetTy>(
        limit: LimitTy,
        offset: OffsetTy,
    ) -> Result<Self, PaginationStartsWithZeroTryNewError>
    where
        LimitTy: Into<PaginationLimit>,
        OffsetTy: Into<PaginationOffset>,
    {
        let limit_value = limit.into();
        let offset_value = offset.into();
        if limit_value.get() <= 0 || offset_value.get() < 0 {
            if limit_value.get() <= 0 {
                Err(
                    PaginationStartsWithZeroTryNewError::LimitIsLessThanOrEqToZero {
                        limit: limit_value,
                        location: location_macros::location!(),
                    },
                )
            } else {
                Err(PaginationStartsWithZeroTryNewError::OffsetIsLessThanZero {
                    offset: offset_value,
                    location: location_macros::location!(),
                })
            }
        } else if offset_value.get().checked_add(limit_value.get()).is_some() {
            Ok(Self::from(PaginationBase::new_unchecked(
                limit_value,
                offset_value,
            )))
        } else {
            Err(
                PaginationStartsWithZeroTryNewError::OffsetPlusLimitIsIntOverflow {
                    limit: limit_value,
                    offset: offset_value,
                    location: location_macros::location!(),
                },
            )
        }
    }
}
impl TryFrom<PaginationStartsWithZeroRaw> for PaginationStartsWithZero {
    type Error = PaginationStartsWithZeroTryNewError;
    fn try_from(v: PaginationStartsWithZeroRaw) -> Result<Self, Self::Error> {
        Self::try_new(v.limit, v.offset)
    }
}
impl<'query_lt> PgTypeWhereFilter<'query_lt> for PaginationStartsWithZero {
    fn query_bind(
        self,
        query: SqlxPostgresQuery<'query_lt>,
    ) -> Result<SqlxPostgresQuery<'query_lt>, SqlxPostgresQueryBindError> {
        self.0.query_bind(query)
    }
    fn query_part(
        &self,
        increment: &mut dyn QueryPartIncrementMut,
        column: SqlColumnRef<'_>,
        add_operator: AddOperator,
    ) -> Result<QueryPartFragment, QueryPartError> {
        self.0.query_part(increment, column, add_operator)
    }
}
impl DefaultSomeOneElement for PaginationStartsWithZero {
    #[inline]
    fn default_some_one_element() -> Self {
        Self::from(PaginationBase::new_unchecked(
            PaginationPolicy::standard().default_limit().get(),
            0,
        ))
    }
}
impl DefaultSomeOneElementMaxPageSize for PaginationStartsWithZero {
    #[inline]
    fn default_some_one_element_max_page_size() -> Self {
        Self::from(PaginationBase::new_unchecked(i32::MAX, 0))
    }
}
//this needed coz serde Option<Optional<T>> #[serde(skip_serializing_if = "Option::is_none")] - if both opts: inner and parent is null then it skip - its not correct
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
)]
pub struct V<T> {
    pub v: T,
}
impl<'schema_lt, T: utoipa::ToSchema<'schema_lt>> utoipa::ToSchema<'schema_lt> for V<T> {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::V,
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    str_constants::PG_CRUD_V_FIELD,
                    <T as utoipa::ToSchema>::schema().1,
                )
                .required(str_constants::PG_CRUD_V_FIELD)
                .build()
                .into(),
        )
    }
}
//todo ExactSizeIterator now is not a solution. error[E0658]: use of unstable library feature `exact_size_is_empty`. maybe rewrite it later
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct IsStringEmptyRes(bool);
pub trait IsStringEmpty {
    fn is_string_empty(&self) -> IsStringEmptyRes;
}
#[location::errors_with_location]
#[derive(
    Debug, serde::Serialize, serde::Deserialize, thiserror::Error, location::Location, optml::Optml,
)]
pub enum NotEmptyUniqueVecTryNewError<T> {
    IsEmpty {},
    NotUnique {
        #[eo_to_err_string_serde]
        v: T,
    },
    TooLong {},
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    schemars::JsonSchema,
    optml::Optml,
    newtype::DefaultInner,
    newtype::IntoVec,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct NotEmptyUniqueVec<T>(Vec<T>);
impl<'schema_lt, T: utoipa::ToSchema<'schema_lt>> utoipa::ToSchema<'schema_lt>
    for NotEmptyUniqueVec<T>
{
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME,
            utoipa::openapi::ArrayBuilder::new()
                .items(<T as utoipa::ToSchema>::schema().1)
                .min_items(Some(1))
                .max_items(Some(NOT_EMPTY_UNIQUE_VEC_MAX_LEN))
                .build()
                .into(),
        )
    }
}
impl<T> NotEmptyUniqueVec<T> {
    #[must_use]
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T: PartialEq> NotEmptyUniqueVec<T> {
    pub fn try_new(
        values: DuplicateCandidates<T>,
    ) -> Result<Self, NotEmptyUniqueVecTryNewError<T>> {
        let raw_values = Vec::from(values);
        if raw_values.is_empty() {
            return Err(NotEmptyUniqueVecTryNewError::IsEmpty {
                location: location_macros::location!(),
            });
        }
        if raw_values.len() > NOT_EMPTY_UNIQUE_VEC_MAX_LEN {
            return Err(NotEmptyUniqueVecTryNewError::TooLong {
                location: location_macros::location!(),
            });
        }
        let mut candidates = DuplicateCandidates::from(raw_values);
        if let Some(duplicate) = take_fst_dup(&mut candidates) {
            return Err(NotEmptyUniqueVecTryNewError::NotUnique {
                v: duplicate,
                location: location_macros::location!(),
            });
        }
        Ok(Self::from(Vec::from(candidates)))
    }
}
impl<T: PartialEq> TryFrom<DuplicateCandidates<T>> for NotEmptyUniqueVec<T> {
    type Error = NotEmptyUniqueVecTryNewError<T>;
    fn try_from(value: DuplicateCandidates<T>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
impl<T: Eq + std::hash::Hash> NotEmptyUniqueVec<T> {
    pub fn try_new_by_hash(
        values: DuplicateCandidates<T>,
    ) -> Result<Self, NotEmptyUniqueVecTryNewError<T>> {
        let raw_values = Vec::from(values);
        if raw_values.is_empty() {
            return Err(NotEmptyUniqueVecTryNewError::IsEmpty {
                location: location_macros::location!(),
            });
        }
        if raw_values.len() > NOT_EMPTY_UNIQUE_VEC_MAX_LEN {
            return Err(NotEmptyUniqueVecTryNewError::TooLong {
                location: location_macros::location!(),
            });
        }
        let mut candidates = DuplicateCandidates::from(raw_values);
        if let Some(duplicate) = take_fst_dup_by_hash(&mut candidates) {
            return Err(NotEmptyUniqueVecTryNewError::NotUnique {
                v: duplicate,
                location: location_macros::location!(),
            });
        }
        Ok(Self::from(Vec::from(candidates)))
    }
}
#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
const _: () = {
    #[expect(clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>> serde::Deserialize<'de>
        for NotEmptyUniqueVec<T>
    {
        fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
        where
            __D: serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de, T>
            where
                T: serde::Deserialize<'de>,
            {
                marker: _serde::__private228::PhantomData<NotEmptyUniqueVec<T>>,
                lt: _serde::__private228::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de, T: std::fmt::Debug + PartialEq + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = NotEmptyUniqueVec<T>;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private228::fmt::Result {
                    std::fmt::Formatter::write_str(
                        __f,
                        str_constants::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME,
                    )
                }
                #[inline]
                fn visit_newtype_struct<__E>(self, __e: __E) -> Result<Self::Value, __E::Error>
                where
                    __E: serde::Deserializer<'de>,
                {
                    let f0: Vec<T> = <Vec<T> as serde::Deserialize>::deserialize(__e)?;
                    match NotEmptyUniqueVec::try_from(DuplicateCandidates::from(f0)) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let Some(f0) = _serde::de::SeqAccess::next_element::<Vec<T>>(&mut __seq)?
                    else {
                        return Err(_serde::de::Error::invalid_length(
                            0usize,
                            &str_constants::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING,
                        ));
                    };
                    match NotEmptyUniqueVec::try_from(DuplicateCandidates::from(f0)) {
                        Ok(v) => Ok(v),
                        Err(error) => Err(_serde::de::Error::custom(format!("{error:?}"))),
                    }
                }
            }
            serde::Deserializer::deserialize_newtype_struct(
                __deserializer,
                str_constants::PG_CRUD_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME,
                __Visitor {
                    marker: _serde::__private228::PhantomData::<Self>,
                    lt: _serde::__private228::PhantomData,
                },
            )
        }
    }
};
impl<T: AllEnumVariantsArrayDefaultSomeOneElement> DefaultSomeOneElement for NotEmptyUniqueVec<T> {
    fn default_some_one_element() -> Self {
        Self::from(Vec::from(
            AllEnumVariantsArrayDefaultSomeOneElement::all_variants_default_some_one_element(),
        ))
    }
}
impl<T: AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize> DefaultSomeOneElementMaxPageSize
    for NotEmptyUniqueVec<T>
{
    fn default_some_one_element_max_page_size() -> Self {
        Self::from(Vec::from(
            AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize::all_variants_default_some_one_element_max_page_size(),
        ))
    }
}
impl<T1> NotEmptyUniqueVec<T1> {
    pub fn from_t1_impl_from_t2<T2>(v: Self) -> NotEmptyUniqueVec<T2>
    where
        T2: From<T1>,
    {
        NotEmptyUniqueVec::from(v.0.into_iter().map(T2::from).collect::<Vec<T2>>())
    }
}
#[cfg(test)]
mod tests_not_empty_unique_vec {
    #[derive(Debug, PartialEq, Eq, newtype::FromInner)]
    struct NonClone(u8);
    #[test]
    fn not_empty_unique_vec_try_new_supports_non_clone_values() {
        let error =
            super::NotEmptyUniqueVec::try_new(vec![NonClone(1), NonClone(2), NonClone(1)].into())
                .expect_err(str_constants::ADF2B8C1);
        match error {
            super::NotEmptyUniqueVecTryNewError::NotUnique { v, .. } => assert_eq!(v, NonClone(1)),
            super::NotEmptyUniqueVecTryNewError::IsEmpty { .. }
            | super::NotEmptyUniqueVecTryNewError::TooLong { .. } => panic!("9f5e2a34"),
        }
    }
    #[test]
    fn not_empty_unique_vec_rejects_oversized_and_deserialized_empty_values() {
        let oversized = (0usize..=super::NOT_EMPTY_UNIQUE_VEC_MAX_LEN).collect::<Vec<_>>();
        assert!(matches!(
            super::NotEmptyUniqueVec::try_new(oversized.into()),
            Err(super::NotEmptyUniqueVecTryNewError::TooLong { .. })
        ));
        let _error =
            serde_json::from_str::<super::NotEmptyUniqueVec<u8>>("[]").expect_err("2510fe33");
    }
    #[test]
    fn not_empty_unique_vec_try_new_returns_is_empty_for_empty_vec() {
        let error = super::NotEmptyUniqueVec::<u8>::try_new(Vec::new().into())
            .expect_err(str_constants::VALUE_3B41DE7F);
        assert!(matches!(
            error,
            super::NotEmptyUniqueVecTryNewError::IsEmpty { .. }
        ));
    }
    #[test]
    fn first_duplicate_idx_returns_none_for_unique_input() {
        let values = vec![1u8, 2u8, 3u8];
        assert!(super::first_duplicate_idx(&values).is_none());
    }
    #[test]
    fn first_duplicate_idx_returns_none_for_empty_and_single_input() {
        assert!(super::first_duplicate_idx::<u8>(&[]).is_none());
        assert!(super::first_duplicate_idx(&[1u8]).is_none());
    }
    #[test]
    fn first_duplicate_idx_returns_first_repeated_value_index() {
        let values = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(
            super::first_duplicate_idx(&values),
            Some(super::DuplicateIdx::from(2))
        );
    }
    #[test]
    fn first_duplicate_idx_by_hash_returns_first_repeated_value_index() {
        let values = vec![7u8, 8u8, 8u8, 7u8];
        assert_eq!(
            super::first_duplicate_idx_by_hash(&values),
            Some(super::DuplicateIdx::from(2))
        );
    }
    #[test]
    fn first_duplicate_idx_by_hash_returns_none_for_empty_and_single_input() {
        assert!(super::first_duplicate_idx_by_hash::<u8>(&[]).is_none());
        assert!(super::first_duplicate_idx_by_hash(&[1u8]).is_none());
    }
    #[test]
    fn take_fst_dup_returns_none_for_unique_input() {
        let mut values = super::DuplicateCandidates::from(vec![1u8, 2u8, 3u8]);
        let actual = super::take_fst_dup(&mut values);
        assert!(actual.is_none());
        assert_eq!(Vec::from(values), vec![1u8, 2u8, 3u8]);
    }
    #[test]
    fn take_fst_dup_returns_first_duplicate_value() {
        let mut values = super::DuplicateCandidates::from(vec![7u8, 8u8, 8u8, 7u8]);
        let actual = super::take_fst_dup(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(Vec::from(values).len(), 3usize);
    }
    #[test]
    fn take_fst_dup_by_hash_returns_first_duplicate_value() {
        let mut values = super::DuplicateCandidates::from(vec![7u8, 8u8, 8u8, 7u8]);
        let actual = super::take_fst_dup_by_hash(&mut values);
        assert_eq!(actual, Some(8u8));
        assert_eq!(Vec::from(values).len(), 3usize);
    }
    #[test]
    fn not_empty_unique_vec_try_new_by_hash_returns_not_unique() {
        let error = super::NotEmptyUniqueVec::try_new_by_hash(vec![1u8, 2u8, 1u8].into())
            .expect_err(str_constants::VALUE_59C80912);
        assert!(matches!(
            error,
            super::NotEmptyUniqueVecTryNewError::NotUnique { v: 1u8, .. }
        ));
    }
    #[test]
    fn as_slice_matches_to_vec_view() {
        let values =
            super::NotEmptyUniqueVec::try_new(vec![1u8, 2u8, 3u8].into()).expect("3f6e8a12");
        assert_eq!(values.as_slice(), &[1u8, 2u8, 3u8]);
        assert_eq!(values.as_slice(), &[1u8, 2u8, 3u8]);
    }
}
impl<'query_lt, T> PgTypeWhereFilter<'query_lt> for NotEmptyUniqueVec<T>
where
    T: std::fmt::Debug
        + PartialEq
        + Clone
        + for<'t_lt> PgTypeWhereFilter<'t_lt>
        + AllEnumVariantsArrayDefaultSomeOneElement,
{
    fn query_bind(
        self,
        query: SqlxPostgresQuery<'query_lt>,
    ) -> Result<SqlxPostgresQuery<'query_lt>, SqlxPostgresQueryBindError> {
        self.0
            .into_iter()
            .try_fold(query, |accumulator_query, element| {
                element.query_bind(accumulator_query)
            })
    }
    fn query_part(
        &self,
        increment: &mut dyn QueryPartIncrementMut,
        column: SqlColumnRef<'_>,
        add_operator: AddOperator,
    ) -> Result<QueryPartFragment, QueryPartError> {
        let mut accumulator = String::with_capacity(self.0.len().saturating_mul(32));
        self.0.iter().enumerate().try_for_each(|(i, element)| {
            let v = element.query_part(
                increment,
                column,
                if i == 0 {
                    add_operator
                } else {
                    AddOperator::from(true)
                },
            )?;
            accumulator.push_str(v.as_ref());
            Ok::<(), QueryPartError>(())
        })?;
        Ok(QueryPartFragment::try_from(accumulator)?)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, optml::Optml)]
#[serde(from = "V<Option<()>>")]
#[derive(newtype::FromInner)]
pub struct NonPrimaryKeyPgTypeReadIds(V<Option<()>>);
impl<'schema_lt> utoipa::ToSchema<'schema_lt> for NonPrimaryKeyPgTypeReadIds {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            str_constants::NONPRIMARYKEYPGTYPEREADIDS,
            utoipa::openapi::ObjectBuilder::new()
                .property(
                    str_constants::PG_CRUD_V_FIELD,
                    utoipa::openapi::ObjectBuilder::new().nullable(true),
                )
                .required(str_constants::PG_CRUD_V_FIELD)
                .build()
                .into(),
        )
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for NonPrimaryKeyPgTypeReadIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value).map(|v0| v0.0)
    }
}
impl sqlx::Type<sqlx::Postgres> for NonPrimaryKeyPgTypeReadIds {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl Default for NonPrimaryKeyPgTypeReadIds {
    fn default() -> Self {
        Self::from(V { v: None })
    }
}
#[derive(Debug, Clone, Copy, optml::Optml)]
pub enum EqOperator {
    Eq,
    IsNull,
}
impl EqOperator {
    #[must_use]
    pub fn to_query_str(&self) -> EqOperatorQueryStr {
        match &self {
            Self::Eq => EqOperatorQueryStr::from(str_constants::PG_CRUD_EQUALITY_SQL_OPERATOR),
            Self::IsNull => EqOperatorQueryStr::from(str_constants::IS_NULL),
        }
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optml::Optml,
    newtype::AsRefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct EqOperatorQueryStr(&'static str);
pub trait PgTypeEqOperator {
    fn operator(&self) -> EqOperator;
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
)]
#[serde(try_from = "i32")]
pub struct UnsignedPartOfI32(i32);
impl From<u16> for UnsignedPartOfI32 {
    fn from(value: u16) -> Self {
        Self(i32::from(value))
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum UnsignedPartOfI32TryFromI32Error {
    LessThanZero {
        location: location_lib::location::Location,
        #[eo_to_err_string_serde]
        v: UnsignedPartOfI32Raw,
    },
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
    newtype::Display,
    newtype::FromInner,
)]
#[serde(from = "i32")]
pub struct UnsignedPartOfI32Raw(i32);
impl UnsignedPartOfI32Raw {
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }
}
impl to_err_string::ToErrString for UnsignedPartOfI32Raw {
    fn to_err_string(&self) -> to_err_string::ErrorText {
        to_err_string::ErrorText::try_from(self.to_string())
            .unwrap_or_else(to_err_string::ErrorText::from)
    }
}
impl TryFrom<i32> for UnsignedPartOfI32 {
    type Error = UnsignedPartOfI32TryFromI32Error;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v >= 0 {
            Ok(Self(v))
        } else {
            Err(Self::Error::LessThanZero {
                v: UnsignedPartOfI32Raw::from(v),
                location: location_macros::location!(),
            })
        }
    }
}
impl to_err_string::ToErrString for UnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::ErrorText {
        to_err_string::ErrorText::try_from(self.0.to_string())
            .unwrap_or_else(to_err_string::ErrorText::from)
    }
}
impl sqlx::Type<sqlx::Postgres> for UnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <i32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for UnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl UnsignedPartOfI32 {
    #[must_use]
    pub const fn get(&self) -> Self {
        *self
    }
}
impl DefaultSomeOneElement for UnsignedPartOfI32 {
    fn default_some_one_element() -> Self {
        Self::from(0u16)
    }
}
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    optml::Optml,
)]
#[serde(try_from = "i32")]
pub struct NotZeroUnsignedPartOfI32(UnsignedPartOfI32);
impl From<std::num::NonZeroU16> for NotZeroUnsignedPartOfI32 {
    fn from(value: std::num::NonZeroU16) -> Self {
        Self(UnsignedPartOfI32::from(value.get()))
    }
}
impl<'schema_lt> utoipa::ToSchema<'schema_lt> for NotZeroUnsignedPartOfI32 {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            stringify!(NotZeroUnsignedPartOfI32),
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::Integer)
                .minimum(Some(1.0f64))
                .maximum(Some(f64::from(i32::MAX)))
                .into(),
        )
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    thiserror::Error,
    location::Location,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum NotZeroUnsignedPartOfI32TryFromI32Error {
    IsZero {
        location: location_lib::location::Location,
    },
    UnsignedPartOfI32TryFromI32Error {
        #[eo_location]
        v: UnsignedPartOfI32TryFromI32Error,
        location: location_lib::location::Location,
    },
}
impl TryFrom<i32> for NotZeroUnsignedPartOfI32 {
    type Error = NotZeroUnsignedPartOfI32TryFromI32Error;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        let v0 = UnsignedPartOfI32::try_from(v).map_err(|error| {
            Self::Error::UnsignedPartOfI32TryFromI32Error {
                v: error,
                location: location_macros::location!(),
            }
        })?;
        if v0.0 == 0 {
            Err(Self::Error::IsZero {
                location: location_macros::location!(),
            })
        } else {
            Ok(Self(v0))
        }
    }
}
impl to_err_string::ToErrString for NotZeroUnsignedPartOfI32 {
    fn to_err_string(&self) -> to_err_string::ErrorText {
        self.0.to_err_string()
    }
}
impl sqlx::Type<sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <UnsignedPartOfI32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
impl sqlx::Encode<'_, sqlx::Postgres> for NotZeroUnsignedPartOfI32 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        <UnsignedPartOfI32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
    }
}
impl NotZeroUnsignedPartOfI32 {
    #[must_use]
    pub const fn get(&self) -> UnsignedPartOfI32 {
        self.0.get()
    }
}
impl DefaultSomeOneElement for NotZeroUnsignedPartOfI32 {
    fn default_some_one_element() -> Self {
        Self::from(std::num::NonZeroU16::MIN)
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    optml::Optml,
)]
pub enum SingleOrMultiple<T: std::fmt::Debug + PartialEq + Clone> {
    Multiple(NotEmptyUniqueVec<T>),
    Single(T),
}
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, optml::Optml, newtype::FromInner, newtype::IntoIterator,
)]
pub struct UuidUuidTestCases([uuid::Uuid; 1]);
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i8_test_cases_vec() -> [i8; 3] {
    [i8::MIN, 0, i8::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i16_test_cases_vec() -> [i16; 3] {
    [i16::MIN, 0, i16::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i32_test_cases_vec() -> [i32; 3] {
    [i32::MIN, 0, i32::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn i64_test_cases_vec() -> [i64; 3] {
    [i64::MIN, 0, i64::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u8_test_cases_vec() -> [u8; 3] {
    [u8::MIN, 0, u8::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u16_test_cases_vec() -> [u16; 3] {
    [u16::MIN, 0, u16::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u32_test_cases_vec() -> [u32; 3] {
    [u32::MIN, 0, u32::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn u64_test_cases_vec() -> [u64; 3] {
    [u64::MIN, 0, u64::MAX]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f32_test_cases_vec() -> [f32; 18] {
    [
        f32::EPSILON,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        -1e30,
        -1e-30,
        -16_777_214.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        16_777_214.0,
        1e-30,
        1e30,
    ]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn f64_test_cases_vec() -> [f64; 18] {
    [
        f64::EPSILON,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        -1e300,
        -1e-300,
        -9_007_199_254_740_990.0,
        -100.0,
        -10.0,
        -1.0,
        -0.0,
        0.0,
        1.0,
        10.0,
        100.0,
        9_007_199_254_740_990.0,
        1e-300,
        1e300,
    ]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub const fn bool_test_cases_vec() -> [bool; 2] {
    [true, false]
}
#[cfg(feature = "test-utils")]
#[must_use]
pub fn string_test_cases_vec() -> [String; 12] {
    [
        String::new(),
        str_constants::A_ALT.to_owned(),
        str_constants::HELLO_WORLD.to_owned(),
        str_constants::THREE_SPACES.to_owned(),
        str_constants::NEWLINE_CARRIAGE_RETURN_TAB.to_owned(),
        str_constants::VALUE_1234567890.to_owned(),
        str_constants::U_1F600.to_owned(),
        str_constants::U_3053_U_3093_U_306B_U_3061_U_306F.to_owned(),
        str_constants::U_1F30D_U_1F680_U_2728_RUST_U_1F496_U_1F980.to_owned(),
        str_constants::A_ALT.repeat(1024),
        str_constants::LINE1_NEWLINE_LINE2_NEWLINE_LINE3.to_owned(),
        str_constants::U_1F496.to_owned(),
    ]
}
#[must_use]
pub fn uuid_uuid_test_cases_vec() -> UuidUuidTestCases {
    UuidUuidTestCases::from([uuid::Uuid::from_u128(
        0x123e_4567_e89b_42d3_a456_4266_1417_4000u128,
    )])
}
