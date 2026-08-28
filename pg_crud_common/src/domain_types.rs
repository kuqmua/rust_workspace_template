#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[path = "add_operator.rs"]
mod add_operator;
#[path = "all_enum_variants.rs"]
mod all_enum_variants;
#[path = "all_enum_variants_array_default_some_one_element.rs"]
mod all_enum_variants_array_default_some_one_element;
#[path = "all_enum_variants_array_default_some_one_element_max_page_size.rs"]
mod all_enum_variants_array_default_some_one_element_max_page_size;
#[path = "batch_validation.rs"]
mod batch_validation;
#[path = "bind_index.rs"]
mod bind_index;
#[path = "bool_test_cases_vec.rs"]
mod bool_test_cases_vec;
#[path = "bounded_btree_map.rs"]
pub mod bounded_btree_map;
#[path = "bounded_unique_vec.rs"]
pub mod bounded_unique_vec;
#[path = "bounded_vec.rs"]
pub mod bounded_vec;
#[path = "build_date_sql_filter.rs"]
mod build_date_sql_filter;
#[path = "build_pg_scoped_foreign_key_clause.rs"]
mod build_pg_scoped_foreign_key_clause;
#[path = "build_sql_like_pattern.rs"]
mod build_sql_like_pattern;
#[path = "build_stable_read_query_plan.rs"]
mod build_stable_read_query_plan;
#[path = "bulk_mutation_outcome.rs"]
mod bulk_mutation_outcome;
#[path = "chrono_utc_date_time_ref.rs"]
mod chrono_utc_date_time_ref;
#[path = "chrono_utc_date_times.rs"]
mod chrono_utc_date_times;
#[path = "classify_pg_code.rs"]
mod classify_pg_code;
#[path = "classify_pg_error.rs"]
mod classify_pg_error;
#[path = "classify_slice_ordering.rs"]
mod classify_slice_ordering;
#[path = "contains_duplicate_identifier.rs"]
mod contains_duplicate_identifier;
#[path = "cursor_codec.rs"]
mod cursor_codec;
#[path = "cursor_codec_build_error.rs"]
mod cursor_codec_build_error;
#[path = "cursor_decode_error.rs"]
mod cursor_decode_error;
#[path = "cursor_encode_error.rs"]
mod cursor_encode_error;
#[path = "cursor_maximum_length.rs"]
mod cursor_maximum_length;
#[path = "cursor_maximum_length_non_zero_usize.rs"]
mod cursor_maximum_length_non_zero_usize;
#[path = "cursor_pagination_usage.rs"]
mod cursor_pagination_usage;
#[path = "cursor_payload.rs"]
mod cursor_payload;
#[path = "cursor_payload_error.rs"]
mod cursor_payload_error;
#[path = "cursor_signing_key.rs"]
mod cursor_signing_key;
#[path = "cursor_signing_key_error.rs"]
mod cursor_signing_key_error;
#[path = "cursor_signing_key_maximum_length.rs"]
mod cursor_signing_key_maximum_length;
#[path = "data_invariant_violation.rs"]
mod data_invariant_violation;
#[path = "date_filter_bounds.rs"]
mod date_filter_bounds;
#[path = "date_sql_bind_start_non_zero_u32.rs"]
mod date_sql_bind_start_non_zero_u32;
#[path = "date_sql_filter.rs"]
mod date_sql_filter;
#[path = "date_sql_filter_error.rs"]
mod date_sql_filter_error;
#[path = "db_schema_conformance.rs"]
mod db_schema_conformance;
#[path = "deduplicate_preserving_order_by_key.rs"]
mod deduplicate_preserving_order_by_key;
#[path = "default_some_one_element.rs"]
mod default_some_one_element;
#[path = "default_some_one_element_max_page_size.rs"]
mod default_some_one_element_max_page_size;
#[path = "duplicate_candidates.rs"]
mod duplicate_candidates;
#[path = "duplicate_idx.rs"]
mod duplicate_idx;
#[path = "eq_operator.rs"]
mod eq_operator;
#[path = "eq_operator_query_str.rs"]
mod eq_operator_query_str;
#[path = "errors.rs"]
mod errors;
#[path = "f32_test_cases_vec.rs"]
mod f32_test_cases_vec;
#[path = "f64_test_cases_vec.rs"]
mod f64_test_cases_vec;
#[path = "filter_bind_plan.rs"]
mod filter_bind_plan;
#[path = "finite_f64.rs"]
mod finite_f64;
#[path = "finite_f64_error.rs"]
mod finite_f64_error;
#[path = "first_duplicate_index.rs"]
mod first_duplicate_index;
#[path = "first_duplicate_index_by_hash.rs"]
mod first_duplicate_index_by_hash;
#[path = "i16_test_cases_vec.rs"]
mod i16_test_cases_vec;
#[path = "i32_test_cases_vec.rs"]
mod i32_test_cases_vec;
#[path = "i64_test_cases_vec.rs"]
mod i64_test_cases_vec;
#[path = "i8_test_cases_vec.rs"]
mod i8_test_cases_vec;
#[path = "is_primary_key.rs"]
mod is_primary_key;
#[path = "is_string_empty.rs"]
mod is_string_empty;
#[path = "is_string_empty_res.rs"]
mod is_string_empty_res;
#[path = "list_total.rs"]
mod list_total;
#[path = "lock_pg_relation_resources.rs"]
mod lock_pg_relation_resources;
#[path = "maximum_resource_count.rs"]
mod maximum_resource_count;
#[path = "maximum_scoped_foreign_key_columns.rs"]
mod maximum_scoped_foreign_key_columns;
#[path = "minimum_scoped_foreign_key_columns.rs"]
mod minimum_scoped_foreign_key_columns;
#[path = "non_primary_key_pg_type_read_ids.rs"]
mod non_primary_key_pg_type_read_ids;
#[path = "not_empty_unique_vec.rs"]
mod not_empty_unique_vec;
#[path = "not_empty_unique_vec_max_len.rs"]
mod not_empty_unique_vec_max_len;
#[path = "not_empty_unique_vec_try_new_error.rs"]
mod not_empty_unique_vec_try_new_error;
#[path = "not_zero_unsigned_part_of_i32.rs"]
mod not_zero_unsigned_part_of_i32;
#[path = "not_zero_unsigned_part_of_i32_non_zero_i32.rs"]
mod not_zero_unsigned_part_of_i32_non_zero_i32;
#[path = "not_zero_unsigned_part_of_i32_try_from_i32_error.rs"]
mod not_zero_unsigned_part_of_i32_try_from_i32_error;
#[path = "nullable_json_obj_pg_type_where_filter.rs"]
mod nullable_json_obj_pg_type_where_filter;
#[path = "offset_pagination_presence.rs"]
mod offset_pagination_presence;
#[path = "operation_budget.rs"]
mod operation_budget;
#[path = "operation_budget_exceeded.rs"]
mod operation_budget_exceeded;
#[path = "operation_count.rs"]
mod operation_count;
#[path = "operator.rs"]
mod operator;
#[path = "order.rs"]
mod order;
#[path = "order_by.rs"]
mod order_by;
#[path = "order_preserving_values.rs"]
mod order_preserving_values;
#[path = "order_snake_case_str.rs"]
mod order_snake_case_str;
#[path = "order_text_string.rs"]
mod order_text_string;
#[path = "order_upper_camel_case_str.rs"]
mod order_upper_camel_case_str;
#[path = "pagination.rs"]
mod pagination;
#[path = "pagination_base.rs"]
mod pagination_base;
#[path = "pagination_starts_with_zero.rs"]
mod pagination_starts_with_zero;
#[path = "pagination_starts_with_zero_raw.rs"]
mod pagination_starts_with_zero_raw;
#[path = "pagination_starts_with_zero_try_new_error.rs"]
mod pagination_starts_with_zero_try_new_error;
#[path = "pagination_total.rs"]
mod pagination_total;
#[path = "patch_field.rs"]
mod patch_field;
#[path = "pg_counter_reconciliation.rs"]
mod pg_counter_reconciliation;
#[path = "pg_counter_value.rs"]
mod pg_counter_value;
#[path = "pg_crud_string_wrapper_max_len.rs"]
mod pg_crud_string_wrapper_max_len;
#[path = "pg_duplicate_identifier_presence.rs"]
mod pg_duplicate_identifier_presence;
#[path = "pg_error_kind.rs"]
mod pg_error_kind;
#[path = "pg_filter_bind_value.rs"]
mod pg_filter_bind_value;
#[path = "pg_filter_bool.rs"]
mod pg_filter_bool;
#[path = "pg_filter_i64.rs"]
mod pg_filter_i64;
#[path = "pg_filter_text.rs"]
mod pg_filter_text;
#[path = "pg_filter_text_error.rs"]
mod pg_filter_text_error;
#[path = "pg_operational_limit.rs"]
mod pg_operational_limit;
#[path = "pg_operational_limit_error.rs"]
mod pg_operational_limit_error;
#[path = "pg_operational_limit_non_zero_u64.rs"]
mod pg_operational_limit_non_zero_u64;
#[path = "pg_operational_limit_update_authority.rs"]
mod pg_operational_limit_update_authority;
#[path = "pg_relation_capacity_error.rs"]
mod pg_relation_capacity_error;
#[path = "pg_relation_capacity_maximum.rs"]
mod pg_relation_capacity_maximum;
#[path = "pg_relation_capacity_maximum_non_zero_u64.rs"]
mod pg_relation_capacity_maximum_non_zero_u64;
#[path = "pg_relation_lock_error.rs"]
mod pg_relation_lock_error;
#[path = "pg_relation_lock_namespace.rs"]
mod pg_relation_lock_namespace;
#[path = "pg_relation_resource_id.rs"]
mod pg_relation_resource_id;
#[path = "pg_relation_resource_ids.rs"]
mod pg_relation_resource_ids;
#[path = "pg_relation_row_count.rs"]
mod pg_relation_row_count;
#[path = "pg_scoped_foreign_key.rs"]
mod pg_scoped_foreign_key;
#[path = "pg_scoped_foreign_key_clause_text.rs"]
mod pg_scoped_foreign_key_clause_text;
#[path = "pg_scoped_foreign_key_error.rs"]
mod pg_scoped_foreign_key_error;
#[path = "pg_scoped_foreign_key_on_delete.rs"]
mod pg_scoped_foreign_key_on_delete;
#[path = "pg_sql_identifiers.rs"]
mod pg_sql_identifiers;
#[path = "pg_type.rs"]
mod pg_type;
#[path = "pg_type_eq_operator.rs"]
mod pg_type_eq_operator;
#[path = "pg_type_greater_than_test.rs"]
mod pg_type_greater_than_test;
#[path = "pg_type_greater_than_variant.rs"]
mod pg_type_greater_than_variant;
#[path = "pg_type_len_greater_than_test.rs"]
mod pg_type_len_greater_than_test;
#[path = "pg_type_not_primary_key.rs"]
mod pg_type_not_primary_key;
#[path = "pg_type_primary_key.rs"]
mod pg_type_primary_key;
#[path = "pg_type_test_cases.rs"]
mod pg_type_test_cases;
#[path = "pg_type_where.rs"]
mod pg_type_where;
#[path = "pg_type_where_filter.rs"]
mod pg_type_where_filter;
#[path = "positive_finite_f64.rs"]
mod positive_finite_f64;
#[path = "positive_finite_f64_error.rs"]
mod positive_finite_f64_error;
#[path = "push_identifier_list.rs"]
mod push_identifier_list;
#[path = "query_fragment.rs"]
mod query_fragment;
#[path = "query_sort_order.rs"]
mod query_sort_order;
#[path = "read_query_plan.rs"]
mod read_query_plan;
#[path = "read_query_plan_error.rs"]
mod read_query_plan_error;
#[path = "reconcile_pg_counter.rs"]
mod reconcile_pg_counter;
#[path = "resolve_pg_operational_limit_update.rs"]
mod resolve_pg_operational_limit_update;
#[path = "signed_cursor.rs"]
mod signed_cursor;
#[path = "signed_cursor_error.rs"]
mod signed_cursor_error;
#[path = "signed_cursor_presence.rs"]
mod signed_cursor_presence;
#[path = "single_or_multiple.rs"]
mod single_or_multiple;
#[path = "slice_ordering.rs"]
mod slice_ordering;
#[path = "sql_identifier.rs"]
mod sql_identifier;
#[path = "sql_identifier_error.rs"]
mod sql_identifier_error;
#[path = "sql_identifier_list_text.rs"]
mod sql_identifier_list_text;
#[path = "sql_identifiers.rs"]
mod sql_identifiers;
#[path = "sql_like_input_ref.rs"]
mod sql_like_input_ref;
#[path = "sql_like_match_mode.rs"]
mod sql_like_match_mode;
#[path = "sql_like_pattern.rs"]
mod sql_like_pattern;
#[path = "sql_like_pattern_error.rs"]
mod sql_like_pattern_error;
#[path = "sql_qualified_identifier.rs"]
mod sql_qualified_identifier;
#[path = "sql_query_text.rs"]
mod sql_query_text;
#[path = "sql_select_builder.rs"]
mod sql_select_builder;
#[path = "sql_sort_order_text.rs"]
mod sql_sort_order_text;
#[path = "sqlx_pg_error_ref.rs"]
mod sqlx_pg_error_ref;
#[path = "sqlx_pg_relation_lock_connection_ref.rs"]
mod sqlx_pg_relation_lock_connection_ref;
#[path = "sqlx_pg_relation_lock_error.rs"]
mod sqlx_pg_relation_lock_error;
#[path = "sqlx_postgres_query.rs"]
mod sqlx_postgres_query;
#[path = "string_test_cases_vec.rs"]
mod string_test_cases_vec;
#[path = "take_fst_dup.rs"]
mod take_fst_dup;
#[path = "take_fst_dup_by.rs"]
mod take_fst_dup_by;
#[path = "take_fst_dup_by_hash.rs"]
mod take_fst_dup_by_hash;
#[path = "transaction_failure.rs"]
mod transaction_failure;
#[path = "try_new_unique_vec.rs"]
mod try_new_unique_vec;
#[path = "u16_test_cases_vec.rs"]
mod u16_test_cases_vec;
#[path = "u32_test_cases_vec.rs"]
mod u32_test_cases_vec;
#[path = "u64_test_cases_vec.rs"]
mod u64_test_cases_vec;
#[path = "u8_test_cases_vec.rs"]
mod u8_test_cases_vec;
#[path = "unit_interval_f64.rs"]
mod unit_interval_f64;
#[path = "unit_interval_f64_error.rs"]
mod unit_interval_f64_error;
#[path = "unsigned_part_of_i32.rs"]
mod unsigned_part_of_i32;
#[path = "unsigned_part_of_i32_raw.rs"]
mod unsigned_part_of_i32_raw;
#[path = "unsigned_part_of_i32_try_from_i32_error.rs"]
mod unsigned_part_of_i32_try_from_i32_error;
#[path = "uuid_uuid_test_cases.rs"]
mod uuid_uuid_test_cases;
#[path = "uuid_uuid_test_cases_vec.rs"]
mod uuid_uuid_test_cases_vec;
#[path = "v.rs"]
mod v;
#[path = "validate_bulk_atomicity.rs"]
mod validate_bulk_atomicity;
#[path = "validate_migration_idempotency.rs"]
mod validate_migration_idempotency;
#[path = "validate_operation_budget.rs"]
mod validate_operation_budget;
#[path = "validate_pagination_invariants.rs"]
mod validate_pagination_invariants;
#[path = "validate_pg_relation_capacity.rs"]
mod validate_pg_relation_capacity;
pub use add_operator::AddOperator;
pub use batch_validation::{
    BatchDuplicatePolicy, BatchInvalidItemCount, BatchInvalidItems, BatchProcessedItemCount,
    BatchRecordsBTreeMap, BatchStoppedEarly, BatchValidationReport, validate_batch_by_key,
};
pub use bind_index::{
    QueryPartIncrement, QueryPartIncrementMut, increment_checked_add_one_returning_increment,
};
#[cfg(feature = "test-utils")]
pub use bool_test_cases_vec::bool_test_cases_vec;
pub use build_date_sql_filter::build_date_sql_filter;
pub use build_pg_scoped_foreign_key_clause::build_pg_scoped_foreign_key_clause;
pub use build_sql_like_pattern::build_sql_like_pattern;
pub use build_stable_read_query_plan::build_stable_read_query_plan;
pub use bulk_mutation_outcome::BulkMutationOutcome;
pub use chrono_utc_date_time_ref::ChronoUtcDateTimeRef;
pub use chrono_utc_date_times::ChronoUtcDateTimes;
pub(crate) use classify_pg_code::classify_pg_code;
pub use classify_pg_error::classify_pg_error;
pub use classify_slice_ordering::classify_slice_ordering;
pub use cursor_codec::CursorCodec;
pub use cursor_codec_build_error::CursorCodecBuildError;
pub use cursor_decode_error::CursorDecodeError;
pub use cursor_encode_error::CursorEncodeError;
pub use cursor_maximum_length::CursorMaximumLength;
pub use cursor_pagination_usage::CursorPaginationUsage;
pub use cursor_payload::CursorPayload;
pub use cursor_payload_error::CursorPayloadError;
pub use cursor_signing_key::CursorSigningKey;
pub use cursor_signing_key_error::CursorSigningKeyError;
pub use data_invariant_violation::DataInvariantViolation;
pub use date_filter_bounds::DateFilterBounds;
pub use date_sql_bind_start_non_zero_u32::DateSqlBindStartNonZeroU32;
pub use date_sql_filter::DateSqlFilter;
pub use date_sql_filter_error::DateSqlFilterError;
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
pub use deduplicate_preserving_order_by_key::deduplicate_preserving_order_by_key;
pub use default_some_one_element::DefaultSomeOneElement;
pub use default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize;
pub use duplicate_candidates::DuplicateCandidates;
pub use duplicate_idx::DuplicateIdx;
pub use eq_operator::EqOperator;
pub use eq_operator_query_str::EqOperatorQueryStr;
pub use errors::{
    PgCrudStringWrapperTryFromStringError, QueryPartError, QueryPartErrorWithSerde,
    SqlxPostgresQueryBindError, make_query_bind_error,
};
#[cfg(feature = "test-utils")]
pub use f32_test_cases_vec::f32_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use f64_test_cases_vec::f64_test_cases_vec;
pub use filter_bind_plan::FilterBindPlan;
pub use finite_f64::FiniteF64;
pub use finite_f64_error::FiniteF64Error;
pub use first_duplicate_index::first_duplicate_index;
pub use first_duplicate_index_by_hash::first_duplicate_index_by_hash;
#[cfg(feature = "test-utils")]
pub use i8_test_cases_vec::i8_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use i16_test_cases_vec::i16_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use i32_test_cases_vec::i32_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use i64_test_cases_vec::i64_test_cases_vec;
pub use is_primary_key::IsPrimaryKey;
pub use is_string_empty::IsStringEmpty;
pub use is_string_empty_res::IsStringEmptyRes;
pub use list_total::{
    ListItems, ListOffset, ListPage, ListRows, ListRowsPresence, ListTotal, ListTotalError,
    ListTotalSource, WindowTotalPresence, resolve_list_total_source, run_list_with_total,
};
pub use lock_pg_relation_resources::lock_pg_relation_resources;
pub use non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds;
pub use not_empty_unique_vec::NotEmptyUniqueVec;
pub(crate) use not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN;
pub use not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError;
pub use not_zero_unsigned_part_of_i32::NotZeroUnsignedPartOfI32;
pub use not_zero_unsigned_part_of_i32_try_from_i32_error::NotZeroUnsignedPartOfI32TryFromI32Error;
pub use nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter;
pub use offset_pagination_presence::OffsetPaginationPresence;
pub use operation_budget::OperationBudget;
pub use operation_budget_exceeded::OperationBudgetExceeded;
pub use operation_count::OperationCount;
pub use operator::Operator;
pub use order::Order;
pub use order_by::OrderBy;
pub use order_preserving_values::OrderPreservingValues;
pub use order_snake_case_str::OrderSnakeCaseStr;
pub(crate) use order_text_string::OrderTextString;
pub use order_upper_camel_case_str::OrderUpperCamelCaseStr;
pub use pagination::{
    PaginationEnd, PaginationLimit, PaginationOffset, PaginationPolicy, PaginationStart,
};
pub use pagination_base::PaginationBase;
pub use pagination_starts_with_zero::PaginationStartsWithZero;
pub use pagination_starts_with_zero_try_new_error::PaginationStartsWithZeroTryNewError;
pub use pagination_total::PaginationTotal;
pub use patch_field::PatchField;
pub use pg_counter_reconciliation::PgCounterReconciliation;
pub use pg_counter_value::PgCounterValue;
pub(crate) use pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
pub use pg_error_kind::PgErrorKind;
pub use pg_filter_bind_value::PgFilterBindValue;
pub use pg_filter_bool::PgFilterBool;
pub use pg_filter_i64::PgFilterI64;
pub use pg_filter_text::PgFilterText;
pub use pg_filter_text_error::PgFilterTextError;
pub use pg_operational_limit::PgOperationalLimit;
pub use pg_operational_limit_error::PgOperationalLimitError;
pub use pg_operational_limit_update_authority::PgOperationalLimitUpdateAuthority;
pub use pg_relation_capacity_error::PgRelationCapacityError;
pub use pg_relation_capacity_maximum::PgRelationCapacityMaximum;
pub use pg_relation_lock_error::PgRelationLockError;
pub use pg_relation_lock_namespace::PgRelationLockNamespace;
pub use pg_relation_resource_id::PgRelationResourceId;
pub use pg_relation_resource_ids::PgRelationResourceIds;
pub use pg_relation_row_count::PgRelationRowCount;
pub use pg_scoped_foreign_key::PgScopedForeignKey;
pub use pg_scoped_foreign_key_error::PgScopedForeignKeyError;
pub use pg_scoped_foreign_key_on_delete::PgScopedForeignKeyOnDelete;
pub use pg_sql_identifiers::PgSqlIdentifiers;
pub use pg_type::PgType;
pub use pg_type_eq_operator::PgTypeEqOperator;
pub use pg_type_greater_than_test::PgTypeGreaterThanTest;
pub use pg_type_greater_than_variant::PgTypeGreaterThanVariant;
pub use pg_type_len_greater_than_test::PgTypeLenGreaterThanTest;
pub use pg_type_not_primary_key::PgTypeNotPrimaryKey;
pub use pg_type_primary_key::PgTypePrimaryKey;
#[cfg(feature = "test-utils")]
pub use pg_type_test_cases::PgTypeTestCases;
pub use pg_type_where::PgTypeWhere;
pub use pg_type_where_filter::PgTypeWhereFilter;
pub use positive_finite_f64::PositiveFiniteF64;
pub use positive_finite_f64_error::PositiveFiniteF64Error;
pub use query_fragment::{QueryPartFragment, ReadQueryBindIndexNonZeroU32, SqlColumnRef};
pub use query_sort_order::QuerySortOrder;
pub use read_query_plan::ReadQueryPlan;
pub use read_query_plan_error::ReadQueryPlanError;
pub use reconcile_pg_counter::reconcile_pg_counter;
pub use resolve_pg_operational_limit_update::resolve_pg_operational_limit_update;
pub use signed_cursor::SignedCursor;
pub use signed_cursor_error::SignedCursorError;
pub use signed_cursor_presence::SignedCursorPresence;
pub use single_or_multiple::SingleOrMultiple;
pub use slice_ordering::SliceOrdering;
pub use sql_identifier::SqlIdentifier;
pub use sql_identifier_error::SqlIdentifierError;
pub(crate) use sql_identifier_list_text::SqlIdentifierListText;
pub use sql_identifiers::SqlIdentifiers;
pub use sql_like_input_ref::SqlLikeInputRef;
pub use sql_like_match_mode::SqlLikeMatchMode;
pub use sql_like_pattern::SqlLikePattern;
pub use sql_like_pattern_error::SqlLikePatternError;
pub use sql_qualified_identifier::SqlQualifiedIdentifier;
pub(crate) use sql_query_text::SqlQueryText;
pub use sql_select_builder::SqlSelectBuilder;
pub(crate) use sql_sort_order_text::SqlSortOrderText;
pub use sqlx_pg_error_ref::SqlxPgErrorRef;
pub use sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef;
pub use sqlx_pg_relation_lock_error::SqlxPgRelationLockError;
pub use sqlx_postgres_query::SqlxPostgresQuery;
#[cfg(feature = "test-utils")]
pub use string_test_cases_vec::string_test_cases_vec;
pub use take_fst_dup::take_fst_dup;
pub(crate) use take_fst_dup_by::take_fst_dup_by;
pub use take_fst_dup_by_hash::take_fst_dup_by_hash;
pub use transaction_failure::TransactionFailure;
#[cfg(feature = "test-utils")]
pub use u8_test_cases_vec::u8_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use u16_test_cases_vec::u16_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use u32_test_cases_vec::u32_test_cases_vec;
#[cfg(feature = "test-utils")]
pub use u64_test_cases_vec::u64_test_cases_vec;
pub use unit_interval_f64::UnitIntervalF64;
pub use unit_interval_f64_error::UnitIntervalF64Error;
pub use unsigned_part_of_i32::UnsignedPartOfI32;
pub use unsigned_part_of_i32_raw::UnsignedPartOfI32Raw;
pub use unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32Error;
pub(crate) use unsigned_part_of_i32_try_from_i32_error::UnsignedPartOfI32TryFromI32ErrorWithSerde;
pub use uuid_uuid_test_cases::UuidUuidTestCases;
pub use uuid_uuid_test_cases_vec::uuid_uuid_test_cases_vec;
pub use v::V;
pub use validate_bulk_atomicity::validate_bulk_atomicity;
pub use validate_migration_idempotency::validate_migration_idempotency;
pub use validate_operation_budget::validate_operation_budget;
pub use validate_pagination_invariants::validate_pagination_invariants;
pub use validate_pg_relation_capacity::validate_pg_relation_capacity;
#[cfg(test)]
#[path = "domain_types_tests_operator_to_query_part.rs"]
mod tests;
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
pub use all_enum_variants::AllEnumVariants;
pub use all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement;
pub use all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize;
