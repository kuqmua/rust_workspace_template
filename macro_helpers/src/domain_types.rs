// AST inspection and attribute parsing.
#[path = "get_macro_attribute.rs"]
pub mod attr_reader;
#[path = "attribute_identifier_string.rs"]
pub mod attribute_identifier_string;
#[path = "derive_token_stream_builder.rs"]
pub mod derive_token_stream_builder;
#[path = "syn_field.rs"]
pub mod syn_field;
// Typed token construction.
#[path = "generate_field_location_new_token_stream.rs"]
pub mod generate_field_location_new_token_stream;
#[path = "generate_if_write_is_error_token_stream.rs"]
pub mod generate_if_write_is_error_token_stream;
#[path = "generate_impl_default_token_stream.rs"]
pub mod generate_impl_default_token_stream;
#[path = "generate_impl_display_token_stream.rs"]
pub mod generate_impl_display_token_stream;
#[path = "generate_impl_from_token_stream.rs"]
pub mod generate_impl_from_token_stream;
#[path = "generate_impl_to_err_string_token_stream.rs"]
pub mod generate_impl_to_err_string_token_stream;
#[path = "generate_impl_try_from_token_stream.rs"]
pub mod generate_impl_try_from_token_stream;
#[path = "generate_new_or_try_new.rs"]
pub mod generate_new_or_try_new;
#[path = "generate_pub_type_alias_token_stream.rs"]
pub mod generate_pub_type_alias_token_stream;
#[path = "generate_simple_syn_punct.rs"]
pub mod generate_simple_syn_punct;
#[path = "proc_macro2_generated_rust_token_stream.rs"]
pub mod proc_macro2_generated_rust_token_stream;
// Test-only contract fixtures.
#[cfg(feature = "test-utils")]
#[path = "json_contract.rs"]
pub mod json_contract;
// Location and source-model support.
#[path = "location.rs"]
pub mod location_data;
#[path = "location_syn_field.rs"]
pub mod location_syn_field;
#[path = "pagination_start_end_initialization_token_stream.rs"]
pub mod pagination_start_end_initialization_token_stream;
#[path = "rs_file_path.rs"]
pub mod rs_file_path;
#[path = "status_code.rs"]
pub mod status_code;
#[path = "tool_command.rs"]
pub mod tool_command;
// Deterministic generated-source writing and formatting.
#[path = "write_string_into_file.rs"]
pub mod string_writer;
#[cfg(feature = "test-utils")]
#[path = "test_database.rs"]
pub mod test_database;
#[cfg(test)]
#[path = "test_helper.rs"]
mod test_helper;
#[path = "write_token_stream_into_file.rs"]
pub mod ts_writer;
// Derive assembly.
#[path = "wrap_derive.rs"]
pub mod wrap_derive;
