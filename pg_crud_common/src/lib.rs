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
mod pg_values;
mod query_collections;
mod query_fragment;
mod query_pagination;
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
pub use pg_values::{
    EqOperator, EqOperatorQueryStr, NotZeroUnsignedPartOfI32,
    NotZeroUnsignedPartOfI32TryFromI32Error, PgTypeEqOperator, SingleOrMultiple, UnsignedPartOfI32,
    UnsignedPartOfI32Raw, UnsignedPartOfI32TryFromI32Error, UuidUuidTestCases,
    uuid_uuid_test_cases_vec,
};
#[cfg(feature = "test-utils")]
pub use pg_values::{
    bool_test_cases_vec, f32_test_cases_vec, f64_test_cases_vec, i8_test_cases_vec,
    i16_test_cases_vec, i32_test_cases_vec, i64_test_cases_vec, string_test_cases_vec,
    u8_test_cases_vec, u16_test_cases_vec, u32_test_cases_vec, u64_test_cases_vec,
};
pub use query_collections::{
    IsStringEmpty, IsStringEmptyRes, NonPrimaryKeyPgTypeReadIds, NotEmptyUniqueVec,
    NotEmptyUniqueVecTryNewError, V,
};
pub use query_fragment::{QueryPartFragment, SqlColumnRef, StdReadQueryBindIndex};
pub use query_pagination::{
    Order, OrderBy, OrderSnakeCaseStr, OrderUpperCamelCaseStr, PaginationBase,
    PaginationStartsWithZero, PaginationStartsWithZeroTryNewError,
};
pub use read_query_plan::{
    QuerySortOrder, ReadQueryPlan, ReadQueryPlanError, build_stable_read_query_plan,
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
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
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(Debug, Clone, Copy, PartialEq, optimal_memory_layout::OptimalMemoryLayout)]
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
    UtoipaToSchemaAndSchemarsJsonSchemaAlias = utoipa::ToSchema + schemars::JsonSchema
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
#[derive(Debug, Clone, PartialEq, optimal_memory_layout::OptimalMemoryLayout)]
pub struct PgTypeGreaterThanTest<T: PgType> {
    pub greater_than: <T as PgType>::TableType,
    pub create: <T as PgType>::Create,
    pub variant: PgTypeGreaterThanVariant,
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct PgTypeLenGreaterThanTest<T: PgType> {
    pub create: <T as PgType>::Create,
    pub variant: PgTypeGreaterThanVariant,
    pub len_greater_than: UnsignedPartOfI32,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
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
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct AddOperator(bool);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    schemars::JsonSchema,
    optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct PgTypeWhere<T> {
    v: NotEmptyUniqueVec<T>,
    operator: Operator,
}
impl<T: utoipa::PartialSchema> utoipa::__dev::ComposeSchema for PgTypeWhere<T> {
    fn compose(
        _new_generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
    ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .property(
                str_constants::PG_CRUD_V_FIELD,
                <NotEmptyUniqueVec<T> as utoipa::PartialSchema>::schema(),
            )
            .property(
                str_constants::PG_CRUD_OPERATOR_FIELD,
                <Operator as utoipa::PartialSchema>::schema(),
            )
            .required(str_constants::PG_CRUD_V_FIELD)
            .required(str_constants::PG_CRUD_OPERATOR_FIELD)
            .build()
            .into()
    }
}
impl<T: utoipa::ToSchema> utoipa::ToSchema for PgTypeWhere<T> {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(str_constants::PG_CRUD_PG_TYPE_WHERE_SCHEMA_NAME)
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
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[expect(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                f0,
                f1,
                __ignore,
            }
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __FieldVisitor;
            impl _serde::de::Visitor<'_> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
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
            #[derive(optimal_memory_layout::OptimalMemoryLayout)]
            #[doc(hidden)]
            struct __Visitor<'de, PgTypeWhere> {
                marker: _serde::__private229::PhantomData<PgTypeWhere>,
                lt: _serde::__private229::PhantomData<&'de ()>,
            }
            impl<'de, T: std::fmt::Debug + PartialEq + Clone + serde::Deserialize<'de>>
                _serde::de::Visitor<'de> for __Visitor<'de, T>
            {
                type Value = PgTypeWhere<T>;
                fn expecting(
                    &self,
                    __f: &mut std::fmt::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
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
                        None => _serde::__private229::de::missing_field(
                            str_constants::PG_CRUD_OPERATOR_FIELD,
                        )?,
                    };
                    let f1_v = match f1 {
                        Some(v) => v,
                        None => {
                            _serde::__private229::de::missing_field(str_constants::PG_CRUD_V_FIELD)?
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
                    marker: _serde::__private229::PhantomData::<T>,
                    lt: _serde::__private229::PhantomData,
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
            .into_vec()
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
                .saturating_add(self.v.as_slice().len().saturating_mul(32))
                .saturating_add(2),
        );
        query_part.push_str(operator_query_part.as_ref());
        query_part.push('(');
        let mut add_operator_inner_handle = AddOperator::from(false);
        let mut is_first = true;
        self.v.as_slice().iter().try_for_each(|element| {
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
