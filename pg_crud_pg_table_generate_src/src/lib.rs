#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "root-owned generator stages preserve operation descriptor grouping from their former owner modules"
)]

mod build_generate_pg_table;
mod compile_error_message;
mod compile_error_token_stream;
pub mod domain_types;
mod emit_generate_pg_table;
mod generate_pg_table;
mod generate_pg_table_field_count;
mod generate_pg_table_model;
mod generate_pg_table_pipeline_error;
mod idempotency_capable;
mod operation_dsc;
mod optimistic_concurrency_capable;
mod parse_generate_pg_table;
pub use build_generate_pg_table::build_generate_pg_table;
pub use generate_pg_table_pipeline_error::GeneratePgTablePipelineError;
pub use parse_generate_pg_table::parse_generate_pg_table;
pub use syn_built_generate_pg_table_input::SynBuiltGeneratePgTableInput;
pub use syn_generate_pg_table_pipeline_error::SynGeneratePgTablePipelineError;
pub use syn_parsed_generate_pg_table_input::SynParsedGeneratePgTableInput;
pub use syn_validated_generate_pg_table_input::SynValidatedGeneratePgTableInput;
pub use validate_generate_pg_table::validate_generate_pg_table;
mod route_http_method;
mod route_success_status;
pub use emit_generate_pg_table::emit_generate_pg_table;
pub use generate_pg_table::generate_pg_table;
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
