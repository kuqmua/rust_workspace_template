#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "private root-owned modules preserve SQL-helper grouping and the compatibility facade vocabulary inherited from the former owner module"
)]

mod add_uo_optimistic_revision_predicate;
mod begin_pg_table_idempotency;
mod calculate_pg_table_idempotency_request_hash;
mod cleanup_pg_table_idempotency;
mod combination_of_app_state_logic_traits;
mod complete_pg_table_idempotency;
mod complete_pg_table_idempotency_in_connection;
#[cfg(test)]
mod domain_types_tests_idempotency;
mod ensure_pg_table_idempotency_schema;
mod generate_cm_query_string;
mod generate_co_query_string;
mod generate_column_eq_v_comma_uo_query_part;
mod generate_column_eqs_case_accumulator_else_column_end_comma_um_query_part;
mod generate_delete_query_string;
mod generate_dlo_query_string;
mod generate_dm_query_string;
mod generate_insert_query_string;
mod generate_rm_query_string;
mod generate_ro_query_string;
mod generate_select_query_string;
mod generate_um_query_string;
mod generate_uo_query_string;
mod generate_update_query_string;
mod generate_when_column_id_then_v_um_query_part;
mod insert_values_fmt;
mod new_pg_table_idempotency_key;
mod pg_table_idempotency_actor;
mod pg_table_idempotency_begin;
mod pg_table_idempotency_body;
mod pg_table_idempotency_body_error;
mod pg_table_idempotency_body_ref;
mod pg_table_idempotency_cleanup_batch_size;
mod pg_table_idempotency_cleanup_batch_size_non_zero_i64;
mod pg_table_idempotency_cleanup_retention_seconds;
mod pg_table_idempotency_cleanup_rows;
mod pg_table_idempotency_cleanup_value_try_from_i64_error;
mod pg_table_idempotency_key;
mod pg_table_idempotency_known_response_status;
mod pg_table_idempotency_method;
mod pg_table_idempotency_replay;
mod pg_table_idempotency_request;
mod pg_table_idempotency_request_hash;
mod pg_table_idempotency_response_status;
mod pg_table_idempotency_response_status_try_from_u16_error;
mod pg_table_idempotency_route;
mod pg_table_idempotency_scope;
mod pg_table_idempotency_text_bytes;
mod pg_table_idempotency_text_error;
mod pg_table_name_ref;
mod pg_table_query_part_fragment;
mod pg_table_query_string;
mod pg_table_revision;
mod pg_table_revision_parse_int_error;
mod pg_table_revision_try_from_string_error;
mod pg_table_sql_fragment_ref;
mod pg_table_string_wrapper_try_from_string_error;
mod pg_tbl_idempotency_route_max_bytes;
mod pg_tbl_idempotency_text_max_bytes;
mod pg_tbl_string_wrapper_max_len;
mod release_pg_table_idempotency;
mod select_where_fmt;
mod sqlx_pg_table_idempotency_error;
mod sqlx_pg_table_pg_connection_ref;
#[cfg(test)]
mod tests;
mod update_selector_fmt;

pub mod domain_types;
pub(crate) use domain_types::*;
