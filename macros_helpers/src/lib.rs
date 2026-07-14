// AST inspection and attribute parsing.
pub mod attr_ident_str;
#[path = "get_macro_attr.rs"]
pub mod attr_reader;
pub mod derive_ts_builder;
#[path = "syn_field.rs"]
pub mod field_data;
// Typed token construction.
pub mod gen_field_loc_new_ts;
pub mod gen_if_write_is_err_ts;
pub mod gen_impl_dflt_ts;
pub mod gen_impl_display_ts;
pub mod gen_impl_from_ts;
pub mod gen_impl_to_err_string_ts;
pub mod gen_impl_try_from_ts;
pub mod gen_new_or_try_new;
pub mod gen_pub_type_al_ts;
pub mod gen_simple_syn_punct;
pub mod generated_rust_ts;
// Test-only contract fixtures.
#[cfg(feature = "test-utils")]
pub mod json_contract;
// Location and source-model support.
#[path = "loc.rs"]
pub mod loc_data;
pub mod loc_syn_field;
pub mod panic_if_err;
pub mod pgn_start_end_init_ts;
pub mod rs_file_path;
pub mod status_code;
pub mod tool_command;
// Deterministic generated-source writing and formatting.
#[path = "write_string_into_file.rs"]
pub mod string_writer;
#[cfg(feature = "test-utils")]
pub mod test_database;
#[cfg(test)]
mod test_hlp;
#[path = "write_ts_into_file.rs"]
pub mod ts_writer;
// Derive assembly.
pub mod wrap_derive;
