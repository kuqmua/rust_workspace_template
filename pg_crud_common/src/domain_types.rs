pub mod bounded_btree_map {
    pub use crate::*;
}
pub mod bounded_unique_vec {
    pub use crate::bounded_unique_vec::*;
}
pub mod bounded_vec {
    pub use crate::bounded_vec::*;
}
pub use crate::add_operator::AddOperator;
pub use crate::batch_validation::{
    BatchDuplicatePolicy, BatchInvalidItemCount, BatchInvalidItems, BatchProcessedItemCount,
    BatchRecordsBTreeMap, BatchStoppedEarly, BatchValidationReport, validate_batch_by_key,
};
#[cfg(feature = "test-utils")]
pub use crate::bool_test_cases_vec::bool_test_cases_vec;
pub use crate::build_date_sql_filter::build_date_sql_filter;
pub use crate::build_pg_scoped_foreign_key_clause::build_pg_scoped_foreign_key_clause;
pub use crate::build_sql_like_pattern::build_sql_like_pattern;
pub use crate::build_stable_read_query_plan::build_stable_read_query_plan;
pub use crate::bulk_mutation_outcome::BulkMutationOutcome;
pub use crate::chrono_utc_date_time_ref::ChronoUtcDateTimeRef;
pub use crate::chrono_utc_date_times::ChronoUtcDateTimes;
#[cfg(test)]
pub(crate) use crate::classify_pg_code::classify_pg_code;
pub use crate::classify_pg_error::classify_pg_error;
pub use crate::classify_slice_ordering::classify_slice_ordering;
pub use crate::cursor_codec::CursorCodec;
pub use crate::cursor_codec_build_error::CursorCodecBuildError;
pub use crate::cursor_decode_error::CursorDecodeError;
pub use crate::cursor_encode_error::CursorEncodeError;
pub use crate::cursor_maximum_length::CursorMaximumLength;
pub use crate::cursor_pagination_usage::CursorPaginationUsage;
pub use crate::cursor_payload::CursorPayload;
pub use crate::cursor_payload_error::CursorPayloadError;
pub use crate::cursor_signing_key::CursorSigningKey;
pub use crate::cursor_signing_key_error::CursorSigningKeyError;
pub use crate::data_invariant_violation::DataInvariantViolation;
pub use crate::date_filter_bounds::DateFilterBounds;
pub use crate::date_sql_bind_start_non_zero_u32::DateSqlBindStartNonZeroU32;
pub use crate::date_sql_filter::DateSqlFilter;
pub use crate::date_sql_filter_error::DateSqlFilterError;
pub use crate::deduplicate_preserving_order_by_key::deduplicate_preserving_order_by_key;
pub use crate::default_some_one_element::DefaultSomeOneElement;
pub use crate::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize;
pub use crate::duplicate_candidates::DuplicateCandidates;
pub use crate::duplicate_idx::DuplicateIdx;
pub use crate::eq_operator::EqOperator;
pub use crate::eq_operator_query_str::EqOperatorQueryStr;
#[cfg(feature = "test-utils")]
pub use crate::f32_test_cases_vec::f32_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use crate::f64_test_cases_vec::f64_test_cases_vec;
pub use crate::filter_bind_plan::FilterBindPlan;
pub use crate::finite_f64::FiniteF64;
pub use crate::finite_f64_error::FiniteF64Error;
pub use crate::first_duplicate_index::first_duplicate_index;
pub use crate::first_duplicate_index_by_hash::first_duplicate_index_by_hash;
#[cfg(feature = "test-utils")]
pub use crate::i8_test_cases_vec::i8_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use crate::i16_test_cases_vec::i16_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use crate::i32_test_cases_vec::i32_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use crate::i64_test_cases_vec::i64_test_cases_vec;
pub use crate::is_primary_key::IsPrimaryKey;
pub use crate::is_string_empty::IsStringEmpty;
pub use crate::is_string_empty_res::IsStringEmptyRes;
pub use crate::list_total::{
    ListItems, ListOffset, ListPage, ListRows, ListRowsPresence, ListTotal, ListTotalError,
    ListTotalSource, WindowTotalPresence, resolve_list_total_source, run_list_with_total,
};
pub use crate::lock_pg_relation_resources::lock_pg_relation_resources;
pub use crate::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds;
pub use crate::not_empty_unique_vec::NotEmptyUniqueVec;
pub(crate) use crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN;
pub use crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError;
pub use crate::not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32;
pub use crate::not_zero_unsigned_part_of_i32_try_from_i32_error::NotZeroUnsignedPartOfI32TryFromI32Error;
pub use crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter;
pub use crate::offset_pagination_presence::OffsetPaginationPresence;
pub use crate::operation_budget::OperationBudget;
pub use crate::operation_budget_exceeded::OperationBudgetExceeded;
pub use crate::operation_count::OperationCount;
pub use crate::operator::Operator;
pub use crate::order::Order;
pub use crate::order_by::OrderBy;
pub use crate::order_preserving_values::OrderPreservingValues;
pub use crate::order_snake_case_str::OrderSnakeCaseStr;
pub(crate) use crate::order_text_string::OrderTextString;
pub use crate::order_upper_camel_case_str::OrderUpperCamelCaseStr;
pub use crate::pagination_base::PaginationBase;
pub use crate::pagination_starts_with_zero::PaginationStartsWithZero;
pub use crate::pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError;
pub use crate::pagination_total::PaginationTotal;
pub use crate::patch_field::PatchField;
pub use crate::pg_counter_reconciliation::PgCounterReconciliation;
pub use crate::pg_counter_value::PgCounterValue;
pub(crate) use crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
pub use crate::pg_error_kind::PgErrorKind;
pub use crate::pg_filter_bind_value::PgFilterBindValue;
pub use crate::pg_filter_bool::PgFilterBool;
pub use crate::pg_filter_i64::PgFilterI64;
pub use crate::pg_filter_text::PgFilterText;
pub use crate::pg_filter_text_error::PgFilterTextError;
pub use crate::pg_operational_limit::PgOperationalLimit;
pub use crate::pg_operational_limit_error::PgOperationalLimitError;
pub use crate::pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority;
pub use crate::pg_relation_capacity_error::PgRelationCapacityError;
pub use crate::pg_relation_capacity_maximum::PgRelationCapacityMaximum;
pub use crate::pg_relation_lock_error::PgRelationLockError;
pub use crate::pg_relation_lock_namespace::PgRelationLockNamespace;
pub use crate::pg_relation_resource_id::PgRelationResourceId;
pub use crate::pg_relation_resource_ids::PgRelationResourceIds;
pub use crate::pg_relation_row_count::PgRelationRowCount;
pub use crate::pg_scoped_foreign_key::PgScopedForeignKey;
pub use crate::pg_scoped_foreign_key_error::PgScopedForeignKeyError;
pub use crate::pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete;
pub use crate::pg_sql_identifiers::PgSqlIdentifiers;
pub use crate::pg_type::PgType;
pub use crate::pg_type_eq_operator::PgTypeEqOperator;
pub use crate::pg_type_greater_than_test::PgTypeGreaterThanTest;
pub use crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant;
pub use crate::pg_type_len_greater_than_test::PgTypeLenGreaterThanTest;
pub use crate::pg_type_not_primary_key::PgTypeNotPrimaryKey;
pub use crate::pg_type_primary_key::PgTypePrimaryKey;
#[cfg(feature = "test-utils")]
pub use crate::pg_type_test_cases::PgTypeTestCases;
pub use crate::pg_type_where::PgTypeWhere;
pub use crate::pg_type_where_filter::PgTypeWhereFilter;
pub use crate::positive_finite_f64::PositiveFiniteF64;
pub use crate::positive_finite_f64_error::PositiveFiniteF64Error;
pub use crate::query_sort_order::QuerySortOrder;
pub use crate::read_query_plan::ReadQueryPlan;
pub use crate::read_query_plan_error::ReadQueryPlanError;
pub use crate::reconcile_pg_counter::reconcile_pg_counter;
pub use crate::resolve_pg_operational_limit_update::resolve_pg_operational_limit_update;
pub use crate::signed_cursor::SignedCursor;
pub use crate::signed_cursor_error::SignedCursorError;
pub use crate::signed_cursor_presence::SignedCursorPresence;
pub use crate::single_or_multiple::SingleOrMultiple;
pub use crate::slice_ordering::SliceOrdering;
pub use crate::sql_identifier::SqlIdentifier;
pub use crate::sql_identifier_error::SqlIdentifierError;
pub(crate) use crate::sql_identifier_list_text::SqlIdentifierListText;
pub use crate::sql_identifiers::SqlIdentifiers;
pub use crate::sql_like_input_ref::SqlLikeInputRef;
pub use crate::sql_like_match_mode::SqlLikeMatchMode;
pub use crate::sql_like_pattern::SqlLikePattern;
pub use crate::sql_like_pattern_error::SqlLikePatternError;
pub use crate::sql_qualified_identifier::SqlQualifiedIdentifier;
pub(crate) use crate::sql_query_text::SqlQueryText;
pub use crate::sql_select_builder::SqlSelectBuilder;
pub(crate) use crate::sql_sort_order_text::SqlSortOrderText;
pub use crate::sqlx_pg_error_ref::SqlxPgErrorRef;
pub use crate::sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef;
pub use crate::sqlx_pg_relation_lock_error::SqlxPgRelationLockError;
pub use crate::sqlx_postgres_query::SqlxPostgresQuery;
#[cfg(feature = "test-utils")]
pub use crate::string_test_cases_vec::string_test_cases_vec;
pub use crate::take_fst_dup::take_fst_dup;
pub(crate) use crate::take_fst_dup_by::take_fst_dup_by;
pub use crate::take_fst_dup_by_hash::take_fst_dup_by_hash;
pub use crate::transaction_failure::TransactionFailure;
#[cfg(feature = "test-utils")]
pub use crate::u8_test_cases_vec::u8_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use crate::u16_test_cases_vec::u16_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use crate::u32_test_cases_vec::u32_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use crate::u64_test_cases_vec::u64_test_cases_vec;
pub use crate::unit_interval_f64::UnitIntervalF64;
pub use crate::unit_interval_f64_error::UnitIntervalF64Error;
pub use crate::unsigned_part_of_i32::UnsignedPartOfI32;
pub use crate::unsigned_part_of_i32_raw::UnsignedPartOfI32Raw;
pub use crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error;
pub(crate) use crate::unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32ErrorWithSerde;
pub use crate::uuid_uuid_test_cases::UuidUuidTestCases;
pub use crate::uuid_uuid_test_cases_vec::uuid_uuid_test_cases_vec;
pub use crate::v::V;
pub use crate::validate_bulk_atomicity::validate_bulk_atomicity;
pub use crate::validate_migration_idempotency::validate_migration_idempotency;
pub use crate::validate_operation_budget::validate_operation_budget;
pub use crate::validate_pagination_invariants::validate_pagination_invariants;
pub use crate::validate_pg_relation_capacity::validate_pg_relation_capacity;
pub use crate::{
    DbCatalogSnapshot, DbColumnContractSnapshot, DbColumnContractSnapshots,
    DbColumnHasServerDefault, DbColumnNullable, DbColumnSnapshot, DbColumnSnapshots, DbColumnSpec,
    DbColumnSpecs, DbDefaultSpec, DbDefaultSpecs, DbExtendedTableSchema, DbKeyContractSnapshot,
    DbKeyContractSnapshots, DbKeySpec, DbKeySpecs, DbObjectKind, DbObjectSnapshot,
    DbObjectSnapshots, DbObjectSpec, DbObjectSpecs, DbSchemaConformanceError, DbSchemaNameRef,
    DbSchemaText, DbSchemaTextTryFromStringError, DbSchemaTexts, DbStaticSchemaText,
    DbStaticSchemaTexts, DbTableNameRef, DbTableSchema, DbTableSnapshot, PgColumnSchema,
    SqlxDbSchemaInspectionError, SqlxPgPoolRef, inspect_postgres_catalog, inspect_postgres_table,
    validate_generated_postgres_table, validate_postgres_catalog,
    validate_postgres_table_extensions, validate_postgres_table_schema,
};
pub use crate::{
    PaginationEnd, PaginationLimit, PaginationOffset, PaginationPolicy, PaginationStart,
};
pub use crate::{
    PgCrudStringWrapperTryFromStringError, QueryPartError, QueryPartErrorWithSerde,
    SqlxPostgresQueryBindError, make_query_bind_error,
};
pub use crate::{QueryPartFragment, ReadQueryBindIndexNonZeroU32, SqlColumnRef};
pub use crate::{
    QueryPartIncrement, QueryPartIncrementMut, increment_checked_add_one_returning_increment,
};
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
pub use crate::all_enum_variants::AllEnumVariants;
pub use crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement;
pub use crate::all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize;
