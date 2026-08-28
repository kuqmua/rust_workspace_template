#![allow(
    clippy::wildcard_imports,
    reason = "private owner modules are rejoined through the compatibility facade"
)]
// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::arbitrary_source_item_ordering)] // SQL helpers stay grouped by generated CRUD concern rather than alphabetically
#[path = "pg_tbl_string_wrapper_max_len.rs"]
mod pg_tbl_string_wrapper_max_len;
use pg_tbl_string_wrapper_max_len::*;
#[path = "pg_tbl_idempotency_text_max_bytes.rs"]
mod pg_tbl_idempotency_text_max_bytes;
use pg_tbl_idempotency_text_max_bytes::*;
#[path = "pg_tbl_idempotency_route_max_bytes.rs"]
mod pg_tbl_idempotency_route_max_bytes;
use pg_tbl_idempotency_route_max_bytes::*;
#[path = "combination_of_app_state_logic_traits.rs"]
mod combination_of_app_state_logic_traits;
pub use combination_of_app_state_logic_traits::*;
#[path = "pg_table_idempotency_actor.rs"]
mod pg_table_idempotency_actor;
pub use pg_table_idempotency_actor::*;
#[path = "pg_table_idempotency_key.rs"]
mod pg_table_idempotency_key;
pub use pg_table_idempotency_key::*;
#[path = "pg_table_idempotency_method.rs"]
mod pg_table_idempotency_method;
pub use pg_table_idempotency_method::*;
#[path = "pg_table_idempotency_route.rs"]
mod pg_table_idempotency_route;
pub use pg_table_idempotency_route::*;
#[path = "pg_table_idempotency_request_hash.rs"]
mod pg_table_idempotency_request_hash;
pub use pg_table_idempotency_request_hash::*;
#[path = "pg_table_idempotency_body.rs"]
mod pg_table_idempotency_body;
pub use pg_table_idempotency_body::*;
#[path = "pg_table_idempotency_body_error.rs"]
mod pg_table_idempotency_body_error;
pub use pg_table_idempotency_body_error::*;
#[path = "pg_table_idempotency_body_ref.rs"]
mod pg_table_idempotency_body_ref;
pub use pg_table_idempotency_body_ref::*;
#[path = "pg_table_idempotency_response_status.rs"]
mod pg_table_idempotency_response_status;
pub use pg_table_idempotency_response_status::*;
#[path = "pg_table_idempotency_known_response_status.rs"]
mod pg_table_idempotency_known_response_status;
pub use pg_table_idempotency_known_response_status::*;
#[path = "pg_table_idempotency_response_status_try_from_u16_error.rs"]
mod pg_table_idempotency_response_status_try_from_u16_error;
pub use pg_table_idempotency_response_status_try_from_u16_error::*;
#[path = "pg_table_idempotency_text_bytes.rs"]
mod pg_table_idempotency_text_bytes;
pub use pg_table_idempotency_text_bytes::*;
#[path = "pg_table_idempotency_cleanup_retention_seconds.rs"]
mod pg_table_idempotency_cleanup_retention_seconds;
pub use pg_table_idempotency_cleanup_retention_seconds::*;
#[path = "pg_table_idempotency_cleanup_batch_size_non_zero_i64.rs"]
mod pg_table_idempotency_cleanup_batch_size_non_zero_i64;
use pg_table_idempotency_cleanup_batch_size_non_zero_i64::*;
#[path = "pg_table_idempotency_cleanup_batch_size.rs"]
mod pg_table_idempotency_cleanup_batch_size;
pub use pg_table_idempotency_cleanup_batch_size::*;
#[path = "pg_table_idempotency_cleanup_value_try_from_i64_error.rs"]
mod pg_table_idempotency_cleanup_value_try_from_i64_error;
pub use pg_table_idempotency_cleanup_value_try_from_i64_error::*;
#[path = "pg_table_idempotency_cleanup_rows.rs"]
mod pg_table_idempotency_cleanup_rows;
pub use pg_table_idempotency_cleanup_rows::*;
#[path = "sqlx_pg_table_pg_connection_ref.rs"]
mod sqlx_pg_table_pg_connection_ref;
pub use sqlx_pg_table_pg_connection_ref::*;
#[path = "pg_table_revision.rs"]
mod pg_table_revision;
pub use pg_table_revision::*;
#[path = "pg_table_revision_parse_int_error.rs"]
mod pg_table_revision_parse_int_error;
pub use pg_table_revision_parse_int_error::*;
#[path = "pg_table_revision_try_from_string_error.rs"]
mod pg_table_revision_try_from_string_error;
pub use pg_table_revision_try_from_string_error::*;
#[path = "pg_table_idempotency_scope.rs"]
mod pg_table_idempotency_scope;
pub use pg_table_idempotency_scope::*;
#[path = "pg_table_idempotency_request.rs"]
mod pg_table_idempotency_request;
pub use pg_table_idempotency_request::*;
#[path = "pg_table_idempotency_replay.rs"]
mod pg_table_idempotency_replay;
pub use pg_table_idempotency_replay::*;
#[path = "pg_table_idempotency_begin.rs"]
mod pg_table_idempotency_begin;
pub use pg_table_idempotency_begin::*;
#[path = "pg_table_idempotency_text_error.rs"]
mod pg_table_idempotency_text_error;
pub use pg_table_idempotency_text_error::*;
#[path = "sqlx_pg_table_idempotency_error.rs"]
mod sqlx_pg_table_idempotency_error;
pub use sqlx_pg_table_idempotency_error::*;
#[path = "new_pg_table_idempotency_key.rs"]
mod new_pg_table_idempotency_key;
pub use new_pg_table_idempotency_key::*;
#[path = "ensure_pg_table_idempotency_schema.rs"]
mod ensure_pg_table_idempotency_schema;
pub use ensure_pg_table_idempotency_schema::*;
#[path = "complete_pg_table_idempotency.rs"]
mod complete_pg_table_idempotency;
pub use complete_pg_table_idempotency::*;
#[path = "complete_pg_table_idempotency_in_connection.rs"]
mod complete_pg_table_idempotency_in_connection;
pub use complete_pg_table_idempotency_in_connection::*;
#[path = "release_pg_table_idempotency.rs"]
mod release_pg_table_idempotency;
pub use release_pg_table_idempotency::*;
#[path = "cleanup_pg_table_idempotency.rs"]
mod cleanup_pg_table_idempotency;
pub use cleanup_pg_table_idempotency::*;
#[path = "insert_values_fmt.rs"]
mod insert_values_fmt;
use insert_values_fmt::*;
#[path = "select_where_fmt.rs"]
mod select_where_fmt;
use select_where_fmt::*;
#[path = "update_selector_fmt.rs"]
mod update_selector_fmt;
use update_selector_fmt::*;
#[path = "pg_table_name_ref.rs"]
mod pg_table_name_ref;
pub use pg_table_name_ref::*;
#[path = "pg_table_sql_fragment_ref.rs"]
mod pg_table_sql_fragment_ref;
pub use pg_table_sql_fragment_ref::*;
#[path = "pg_table_query_string.rs"]
mod pg_table_query_string;
pub use pg_table_query_string::*;
#[path = "pg_table_string_wrapper_try_from_string_error.rs"]
mod pg_table_string_wrapper_try_from_string_error;
pub use pg_table_string_wrapper_try_from_string_error::*;
#[path = "pg_table_query_part_fragment.rs"]
mod pg_table_query_part_fragment;
pub use pg_table_query_part_fragment::*;
#[path = "generate_insert_query_string.rs"]
mod generate_insert_query_string;
use generate_insert_query_string::*;
#[path = "generate_select_query_string.rs"]
mod generate_select_query_string;
use generate_select_query_string::*;
#[path = "generate_update_query_string.rs"]
mod generate_update_query_string;
use generate_update_query_string::*;
#[path = "generate_delete_query_string.rs"]
mod generate_delete_query_string;
use generate_delete_query_string::*;
#[path = "generate_cm_query_string.rs"]
mod generate_cm_query_string;
pub use generate_cm_query_string::*;
#[path = "generate_co_query_string.rs"]
mod generate_co_query_string;
pub use generate_co_query_string::*;
#[path = "generate_rm_query_string.rs"]
mod generate_rm_query_string;
pub use generate_rm_query_string::*;
#[path = "generate_ro_query_string.rs"]
mod generate_ro_query_string;
pub use generate_ro_query_string::*;
#[path = "generate_column_eq_v_comma_uo_query_part.rs"]
mod generate_column_eq_v_comma_uo_query_part;
pub use generate_column_eq_v_comma_uo_query_part::*;
#[path = "generate_when_column_id_then_v_um_query_part.rs"]
mod generate_when_column_id_then_v_um_query_part;
pub use generate_when_column_id_then_v_um_query_part::*;
#[path = "generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part.rs"]
mod generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part;
pub use generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part::*;
#[path = "generate_um_query_string.rs"]
mod generate_um_query_string;
pub use generate_um_query_string::*;
#[path = "generate_uo_query_string.rs"]
mod generate_uo_query_string;
pub use generate_uo_query_string::*;
#[path = "add_uo_optimistic_revision_predicate.rs"]
mod add_uo_optimistic_revision_predicate;
pub use add_uo_optimistic_revision_predicate::*;
#[path = "generate_dm_query_string.rs"]
mod generate_dm_query_string;
pub use generate_dm_query_string::*;
#[path = "generate_dlo_query_string.rs"]
mod generate_dlo_query_string;
pub use generate_dlo_query_string::*;
#[path = "functions.rs"]
mod functions;
pub use functions::*;

#[cfg(test)]
#[path = "domain_types_tests_idempotency.rs"]
mod idempotency_tests;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
