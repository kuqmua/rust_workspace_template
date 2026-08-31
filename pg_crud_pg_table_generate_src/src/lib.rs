#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "root-owned generator stages preserve operation descriptor grouping from their former owner modules"
)]

pub mod build_generate_pg_table;
pub mod domain_types;
pub mod emit_generate_pg_table;
pub mod generate_pg_table;
pub mod generate_pg_table_field_count;
pub mod generate_pg_table_model;
pub mod generate_pg_table_pipeline_error;
pub mod idempotency_capable;
pub mod operation_dsc;
pub mod optimistic_concurrency_capable;
pub mod parse_generate_pg_table;
pub mod pg_table_compile_error_message;
pub mod pg_table_compile_error_tokens;
pub mod route_http_method;
pub mod route_success_status;
pub mod sql;
pub mod struct_shape;
pub mod success_status;
pub mod syn_built_generate_pg_table_input;
pub mod syn_generate_pg_table_model_error;
pub mod syn_generate_pg_table_model_input;
pub mod syn_generate_pg_table_pipeline_error;
pub mod syn_parsed_generate_pg_table_input;
pub mod syn_validated_generate_pg_table_input;
pub mod table;
pub mod table_test_names;
#[cfg(test)]
pub mod test_tests;
pub mod validate_generate_pg_table;

const _: fn(&str) -> Result<(), bounded_types::bounded_string_error::BoundedStringError> =
    bounded_types::bounded_string::BoundedString::<0, 0>::validate_str;
