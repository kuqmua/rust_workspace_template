// AST inspection and attribute parsing.
pub mod attr_identifier_str;
#[path = "domain_types/get_macro_attr.rs"]
pub mod attr_reader;
pub mod derive_token_stream_builder;
#[path = "domain_types/syn_field.rs"]
pub mod field_data;
// Typed token construction.
pub mod generate_field_location_new_token_stream;
pub mod generate_if_write_is_err_token_stream;
pub mod generate_impl_default_token_stream;
pub mod generate_impl_display_token_stream;
pub mod generate_impl_from_token_stream;
pub mod generate_impl_to_err_string_token_stream;
pub mod generate_impl_try_from_token_stream;
pub mod generate_new_or_try_new;
pub mod generate_pub_type_alias_token_stream;
pub mod generate_simple_syn_punct;
pub mod proc_macro2_tokens;
// Test-only contract fixtures.
#[cfg(feature = "test-utils")]
pub mod json_contract;
// Location and source-model support.
#[path = "domain_types/location.rs"]
pub mod location_data;
pub mod location_syn_field;
pub mod pagination_start_end_initialization_token_stream;
pub mod rs_file_path;
pub mod status_code;
pub mod tool_command;
// Deterministic generated-source writing and formatting.
#[path = "domain_types/write_string_into_file.rs"]
pub mod string_writer;
#[cfg(feature = "test-utils")]
pub mod test_database;
#[cfg(test)]
mod test_hlp;
#[path = "domain_types/write_token_stream_into_file.rs"]
pub mod ts_writer;
// Derive assembly.
pub mod wrap_derive;
