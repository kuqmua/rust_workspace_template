#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::single_call_fn,
    reason = "root-owned generator stages preserve operation descriptor grouping and independently named projection boundaries from their former owner modules"
)]

mod build_generate_pg_table;
mod compile_error_message;
mod compile_error_token_stream;
mod contract_tests;
pub mod domain_types;
mod emit_generate_pg_table;
mod frontend_http_method;
mod frontend_operation_kind;
mod frontend_permission_action;
mod frontend_success_status;
mod generate_pg_table;
mod generate_pg_table_field_count;
mod generate_pg_table_model;
mod generate_pg_table_pipeline_error;
mod http_method;
mod idempotency_capable;
mod openapi;
mod openapi_http_method;
mod openapi_success_status;
mod operation_dsc;
mod optimistic_concurrency_capable;
mod parse_generate_pg_table;
mod pipeline;
mod route_http_method;
mod route_operation_kind;
mod route_permission_action;
mod route_success_status;
mod source;
mod sql;
mod struct_shape;
mod success_status;
mod syn_built_generate_pg_table_input;
mod syn_generate_pg_table_model_error;
mod syn_generate_pg_table_model_input;
mod syn_generate_pg_table_pipeline_error;
mod syn_parsed_generate_pg_table_input;
mod syn_validated_generate_pg_table_input;
mod table;
mod table_test_names;
#[cfg(test)]
mod tests;
mod validate_generate_pg_table;
