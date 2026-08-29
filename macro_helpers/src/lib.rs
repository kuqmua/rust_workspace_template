#![allow(
    unused_imports,
    clippy::arbitrary_source_item_ordering,
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    reason = "root-owned modules retain the vocabulary and compatibility namespaces previously inherited from nested owner modules"
)]

#[cfg(test)]
mod assert_file_content;
#[cfg(test)]
pub(crate) use assert_file_content::*;
#[cfg(test)]
mod assert_file_path_ref;
#[cfg(test)]
pub(crate) use assert_file_path_ref::*;
mod attr_identifier_name;
pub(crate) use attr_identifier_name::*;
mod attr_identifier_str;
pub use attr_identifier_name::AttrIdentifierName;
pub use attr_identifier_str::AttrIdentifierStr;
pub(crate) use attr_identifier_str::*;
#[cfg(test)]
mod cleanup_test_file;
#[cfg(test)]
pub(crate) use cleanup_test_file::*;
mod compile_error_message;
pub(crate) use compile_error_message::*;
mod compile_error_token_stream;
pub(crate) use compile_error_token_stream::*;
#[cfg(feature = "test-utils")]
mod contract_error;
#[cfg(feature = "test-utils")]
pub(crate) use contract_error::*;
mod derive_token_stream_builder;
pub(crate) use derive_token_stream_builder::*;
#[cfg(feature = "test-utils")]
mod ensure_json_contract_round_trip;
#[cfg(feature = "test-utils")]
pub(crate) use ensure_json_contract_round_trip::*;
#[cfg(test)]
mod expected_file_content;
#[cfg(test)]
pub(crate) use expected_file_content::*;
#[cfg(test)]
mod expected_file_content_ref;
#[cfg(test)]
pub(crate) use expected_file_content_ref::*;
mod field_location_column;
pub(crate) use field_location_column::*;
mod field_location_coordinate_try_from_u32_error;
pub(crate) use field_location_coordinate_try_from_u32_error::*;
mod field_location_file;
pub(crate) use field_location_file::*;
mod field_location_line;
pub(crate) use field_location_line::*;
mod find_macro_attribute;
pub(crate) use find_macro_attribute::*;
mod format_with_cargofmt;
pub(crate) use format_with_cargofmt::*;
mod generate_const_new_token_stream_impl;
pub(crate) use generate_const_new_token_stream_impl::*;
mod generate_const_try_new_token_stream_impl;
pub(crate) use generate_const_try_new_token_stream_impl::*;
mod generate_field_location_new_token_stream;
pub(crate) use generate_field_location_new_token_stream::*;
mod generate_if_write_is_error_token_stream;
pub(crate) use generate_if_write_is_error_token_stream::*;
mod generate_impl_const_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_const_new_for_identifier_token_stream_impl::*;
mod generate_impl_default_token_stream;
pub(crate) use generate_impl_default_token_stream::*;
mod generate_impl_display_token_stream;
pub(crate) use generate_impl_display_token_stream::*;
mod generate_impl_from_token_stream;
pub(crate) use generate_impl_from_token_stream::*;
mod generate_impl_modified_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_modified_new_for_identifier_token_stream_impl::*;
mod generate_impl_modified_try_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_modified_try_new_for_identifier_token_stream_impl::*;
mod generate_impl_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_const_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_pub_const_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_const_try_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_pub_const_try_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_pub_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_try_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_pub_try_new_for_identifier_token_stream_impl::*;
mod generate_impl_to_err_string_token_stream;
pub(crate) use generate_impl_to_err_string_token_stream::*;
mod generate_impl_try_from_token_stream;
pub(crate) use generate_impl_try_from_token_stream::*;
mod generate_impl_try_new_for_identifier_token_stream_impl;
pub(crate) use generate_impl_try_new_for_identifier_token_stream_impl::*;
mod generate_modified_new_token_stream_impl;
pub(crate) use generate_modified_new_token_stream_impl::*;
mod generate_modified_try_new_token_stream_impl;
pub use generate_const_new_token_stream_impl::generate_const_new_token_stream_impl;
pub use generate_const_try_new_token_stream_impl::generate_const_try_new_token_stream_impl;
pub use generate_impl_const_new_for_identifier_token_stream_impl::generate_impl_const_new_for_identifier_token_stream_impl;
pub use generate_impl_new_for_identifier_token_stream_impl::generate_impl_new_for_identifier_token_stream_impl;
pub use generate_impl_pub_const_new_for_identifier_token_stream_impl::generate_impl_pub_const_new_for_identifier_token_stream_impl;
pub use generate_impl_pub_const_try_new_for_identifier_token_stream_impl::generate_impl_pub_const_try_new_for_identifier_token_stream_impl;
pub use generate_impl_pub_new_for_identifier_token_stream_impl::generate_impl_pub_new_for_identifier_token_stream_impl;
pub use generate_impl_pub_try_new_for_identifier_token_stream_impl::generate_impl_pub_try_new_for_identifier_token_stream_impl;
pub use generate_impl_try_new_for_identifier_token_stream_impl::generate_impl_try_new_for_identifier_token_stream_impl;
pub(crate) use generate_modified_try_new_token_stream_impl::*;
pub use generate_new_token_stream_impl::generate_new_token_stream_impl;
pub use generate_pub_const_new_token_stream_impl::generate_pub_const_new_token_stream_impl;
pub use generate_pub_const_try_new_token_stream_impl::generate_pub_const_try_new_token_stream_impl;
pub use generate_pub_new_token_stream_impl::generate_pub_new_token_stream_impl;
pub use generate_pub_try_new_token_stream_impl::generate_pub_try_new_token_stream_impl;
pub use generate_try_new_token_stream_impl::generate_try_new_token_stream_impl;
#[cfg(test)]
mod generate_new_or_try_new_tests;
#[cfg(test)]
pub(crate) use generate_new_or_try_new_tests::*;
mod generate_new_token_stream_impl;
pub(crate) use generate_new_token_stream_impl::*;
mod generate_pub_const_new_token_stream_impl;
pub(crate) use generate_pub_const_new_token_stream_impl::*;
mod generate_pub_const_try_new_token_stream_impl;
pub(crate) use generate_pub_const_try_new_token_stream_impl::*;
mod generate_pub_new_token_stream_impl;
pub(crate) use generate_pub_new_token_stream_impl::*;
mod generate_pub_try_new_token_stream_impl;
pub(crate) use generate_pub_try_new_token_stream_impl::*;
mod generate_pub_type_alias_token_stream;
pub(crate) use generate_pub_type_alias_token_stream::*;
mod generate_serde_version_of_named_syn_variant;
pub(crate) use generate_serde_version_of_named_syn_variant::*;
mod generate_simple_syn_punct;
pub(crate) use generate_simple_syn_punct::*;
mod generate_try_new_token_stream_impl;
pub(crate) use generate_try_new_token_stream_impl::*;
mod generated_file_maximum_bytes;
pub(crate) use generated_file_maximum_bytes::*;
mod get_macro_attribute;
pub(crate) use get_macro_attribute::*;
mod impl_identifier_token_stream_impl;
pub(crate) use impl_identifier_token_stream_impl::*;
#[cfg(feature = "test-utils")]
mod json_contract;
#[cfg(feature = "test-utils")]
pub(crate) use json_contract::*;
#[cfg(feature = "test-utils")]
mod json_fixture_ref;
pub use generate_serde_version_of_named_syn_variant::generate_serde_version_of_named_syn_variant;
#[cfg(feature = "test-utils")]
pub(crate) use json_fixture_ref::*;
pub use location_field_attr::LocationFieldAttr;
pub use syn_variant_ref::SynVariantRef;
mod location_field_attr;
pub(crate) use location_field_attr::*;
mod location_syn_field;
pub(crate) use location_syn_field::*;
mod macro_attr_error;
pub(crate) use macro_attr_error::*;
mod only_one;
pub(crate) use only_one::*;
mod only_one_status_code_error;
pub(crate) use only_one_status_code_error::*;
mod os_string_value;
pub(crate) use os_string_value::*;
mod pagination_start_end_initialization_token_stream;
pub(crate) use pagination_start_end_initialization_token_stream::*;
mod path_ref;
pub(crate) use path_ref::*;
mod proc_macro2_derive_tokens_ref;
pub(crate) use proc_macro2_derive_tokens_ref::*;
mod proc_macro2_generated_rust_token_stream;
pub(crate) use proc_macro2_generated_rust_token_stream::*;
mod proc_macro2_if_write_is_err_token_stream;
pub(crate) use proc_macro2_if_write_is_err_token_stream::*;
mod proc_macro2_macro_attr_meta_list_token_stream_ref;
pub(crate) use proc_macro2_macro_attr_meta_list_token_stream_ref::*;
mod proc_macro2_token_stream_ref;
pub(crate) use proc_macro2_token_stream_ref::*;
mod process_command;
pub(crate) use process_command::*;
mod process_exit_status;
pub(crate) use process_exit_status::*;
mod process_output;
pub(crate) use process_output::*;
mod rs_file_path;
pub(crate) use rs_file_path::*;
mod rs_file_path_buf;
pub(crate) use rs_file_path_buf::*;
#[cfg(feature = "test-utils")]
mod sanitized_database_target;
#[cfg(feature = "test-utils")]
pub(crate) use sanitized_database_target::*;
#[cfg(feature = "test-utils")]
mod serde_json_error;
#[cfg(feature = "test-utils")]
pub(crate) use serde_json_error::*;
mod should_write_string;
pub(crate) use should_write_string::*;
mod should_write_string_into_file;
pub(crate) use should_write_string_into_file::*;
mod should_write_token_stream_into_file;
pub(crate) use should_write_token_stream_into_file::*;
mod status_code;
pub(crate) use status_code::*;
#[cfg(test)]
mod std_assert_file_path;
#[cfg(test)]
pub(crate) use std_assert_file_path::*;
mod string_file_content_ref;
pub(crate) use string_file_content_ref::*;
mod string_syn_punct;
pub(crate) use string_syn_punct::*;
mod syn_field;
pub(crate) use syn_field::*;
mod syn_field_identifier;
pub(crate) use syn_field_identifier::*;
mod syn_field_type;
pub(crate) use syn_field_type::*;
mod syn_field_vis;
pub(crate) use syn_field_vis::*;
mod syn_location_field;
pub(crate) use syn_location_field::*;
mod syn_macro_attr_ref;
pub(crate) use syn_macro_attr_ref::*;
mod syn_path_segment;
pub(crate) use syn_path_segment::*;
mod syn_path_segments;
pub(crate) use syn_path_segments::*;
mod syn_variant_ref;
pub(crate) use syn_variant_ref::*;
#[cfg(feature = "test-utils")]
mod test_database;
#[cfg(test)]
pub(crate) use assert_file_content::assert_file_content;
#[cfg(test)]
pub(crate) use assert_file_path_ref::AssertFilePathRef;
#[cfg(test)]
pub(crate) use cleanup_test_file::cleanup_test_file;
#[cfg(test)]
pub(crate) use expected_file_content::ExpectedFileContent;
#[cfg(test)]
pub(crate) use expected_file_content_ref::ExpectedFileContentRef;
#[cfg(test)]
pub(crate) use std_assert_file_path::StdAssertFilePath;
#[cfg(feature = "test-utils")]
pub(crate) use test_database::*;
#[cfg(test)]
pub(crate) use test_path::test_path;
#[cfg(test)]
pub(crate) use test_path_stem::TestPathStem;
#[cfg(test)]
mod test_path;
#[cfg(test)]
pub(crate) use test_path::*;
#[cfg(test)]
mod test_path_stem;
#[cfg(test)]
pub(crate) use test_path_stem::*;
mod tool_arg_ref;
pub(crate) use tool_arg_ref::*;
mod tool_args_ref;
pub(crate) use tool_args_ref::*;
mod tool_command;
pub(crate) use tool_command::*;
mod tool_env_key_ref;
pub(crate) use tool_env_key_ref::*;
mod tool_env_value_ref;
pub(crate) use tool_env_value_ref::*;
mod tool_program_ref;
pub(crate) use tool_program_ref::*;
mod try_get_macro_attr_meta_list_token_stream;
pub(crate) use try_get_macro_attr_meta_list_token_stream::*;
mod try_get_macro_attribute;
pub(crate) use try_get_macro_attribute::*;
mod try_maybe_write_token_stream_into_file;
pub(crate) use try_maybe_write_token_stream_into_file::*;
mod try_write_string_into_file;
pub(crate) use try_write_string_into_file::*;
mod try_write_string_into_file_with_outcome;
pub(crate) use try_write_string_into_file_with_outcome::*;
mod try_write_string_into_path;
pub(crate) use try_write_string_into_path::*;
mod try_write_string_into_path_with_outcome;
pub(crate) use try_write_string_into_path_with_outcome::*;
#[cfg(feature = "test-utils")]
mod url_error;
#[cfg(feature = "test-utils")]
pub(crate) use url_error::*;
#[cfg(feature = "test-utils")]
mod url_ref;
#[cfg(feature = "test-utils")]
pub(crate) use url_ref::*;
mod validate_existing_file_text;
pub(crate) use validate_existing_file_text::*;
#[cfg(feature = "test-utils")]
mod validate_test_database_url;
#[cfg(feature = "test-utils")]
pub(crate) use validate_test_database_url::*;
mod with_attr_token_stream_impl;
pub(crate) use with_attr_token_stream_impl::*;
mod wrap_derive;
pub(crate) use wrap_derive::*;
mod write_path_outcome;
pub(crate) use write_path_outcome::*;
mod write_string_if_needed;
pub(crate) use write_string_if_needed::*;
mod write_string_into_file;
pub(crate) use write_string_into_file::*;
mod write_token_stream_into_file;
pub(crate) use write_token_stream_into_file::*;
mod written_file_path_buf;
pub(crate) use written_file_path_buf::*;
mod written_file_path_ref;
pub(crate) use written_file_path_ref::*;
pub mod domain_types;
