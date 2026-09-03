#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "private root-owned modules preserve SQL-helper grouping and the compatibility facade vocabulary inherited from the former owner module"
)]

pub mod add_uo_optimistic_revision_predicate;
pub mod begin_pg_table_idempotency;
pub mod calculate_pg_table_idempotency_request_hash;
pub mod cleanup_pg_table_idempotency;
pub mod combination_of_app_state_logic_traits;
pub mod complete_pg_table_idempotency;
pub mod complete_pg_table_idempotency_in_connection;
pub mod ensure_pg_table_idempotency_schema;
pub mod generate_cm_query_string;
pub mod generate_co_query_string;
pub mod generate_column_eq_v_comma_uo_query_part;
pub mod generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part;
pub mod generate_delete_query_string;
pub mod generate_dlo_query_string;
pub mod generate_dm_query_string;
pub mod generate_insert_query_string;
pub mod generate_rm_query_string;
pub mod generate_ro_query_string;
pub mod generate_select_query_string;
pub mod generate_um_query_string;
pub mod generate_uo_query_string;
pub mod generate_update_query_string;
pub mod generate_when_column_id_then_v_um_query_part;
pub mod insert_values_fmt;
pub mod new_pg_table_idempotency_key;
pub mod pg_table_idempotency_actor;
pub mod pg_table_idempotency_begin;
pub mod pg_table_idempotency_body;
pub mod pg_table_idempotency_body_error;
pub mod pg_table_idempotency_body_ref;
pub mod pg_table_idempotency_cleanup_batch_size;
pub mod pg_table_idempotency_cleanup_retention_seconds;
pub mod pg_table_idempotency_cleanup_rows;
pub mod pg_table_idempotency_cleanup_value_try_from_i64_error;
pub mod pg_table_idempotency_key;
pub mod pg_table_idempotency_known_response_status;
pub mod pg_table_idempotency_method;
pub mod pg_table_idempotency_replay;
pub mod pg_table_idempotency_request;
pub mod pg_table_idempotency_request_hash;
pub mod pg_table_idempotency_response_status;
pub mod pg_table_idempotency_response_status_try_from_u16_error;
pub mod pg_table_idempotency_route;
pub mod pg_table_idempotency_scope;
pub mod pg_table_idempotency_text_bytes;
pub mod pg_table_idempotency_text_error;
pub mod pg_table_name_ref;
pub mod pg_table_query_part_fragment;
pub mod pg_table_query_string;
pub mod pg_table_revision;
pub mod pg_table_revision_parse_int_error;
pub mod pg_table_revision_try_from_string_error;
pub mod pg_table_sql_fragment_ref;
pub mod pg_table_string_wrapper_try_from_string_error;
pub mod pg_tbl_idempotency_route_max_bytes;
pub mod pg_tbl_idempotency_text_max_bytes;
pub mod pg_tbl_string_wrapper_max_len;
pub mod release_pg_table_idempotency;
pub mod select_where_fmt;
pub mod sqlx_pg_table_idempotency_error;
pub mod sqlx_pg_table_pg_connection_ref;
#[cfg(test)]
pub mod test_tests;
#[cfg(test)]
pub mod test_tests_domain_types_idempotency;
pub mod update_selector_fmt;
mod validate_pg_table_idempotency_text;
