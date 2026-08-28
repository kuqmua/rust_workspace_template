#![allow(
    unused_imports,
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    reason = "root-owned modules retain the vocabulary and compatibility namespaces previously inherited from nested owner modules"
)]

#[cfg(test)]
mod assert_file_content;
#[cfg(test)]
pub(crate) use crate::assert_file_content::*;
#[cfg(test)]
mod assert_file_path_ref;
#[cfg(test)]
pub(crate) use crate::assert_file_path_ref::*;
mod attr_identifier_name;
pub(crate) use crate::attr_identifier_name::*;
mod attr_identifier_str;
pub(crate) use crate::attr_identifier_str::*;
mod attribute_identifier_string;
pub(crate) use crate::attribute_identifier_string::*;
#[cfg(test)]
mod cleanup_test_file;
#[cfg(test)]
pub(crate) use crate::cleanup_test_file::*;
mod compile_error_message;
pub(crate) use crate::compile_error_message::*;
mod compile_error_token_stream;
pub(crate) use crate::compile_error_token_stream::*;
#[cfg(feature = "test-utils")]
mod contract_error;
#[cfg(feature = "test-utils")]
pub(crate) use crate::contract_error::*;
mod derive_token_stream_builder;
pub(crate) use crate::derive_token_stream_builder::*;
#[cfg(feature = "test-utils")]
mod ensure_json_contract_round_trip;
#[cfg(feature = "test-utils")]
pub(crate) use crate::ensure_json_contract_round_trip::*;
#[cfg(test)]
mod expected_file_content;
#[cfg(test)]
pub(crate) use crate::expected_file_content::*;
#[cfg(test)]
mod expected_file_content_ref;
#[cfg(test)]
pub(crate) use crate::expected_file_content_ref::*;
mod field_location_column;
pub(crate) use crate::field_location_column::*;
mod field_location_column_non_zero_u32;
pub(crate) use crate::field_location_column_non_zero_u32::*;
mod field_location_coordinate_try_from_u32_error;
pub(crate) use crate::field_location_coordinate_try_from_u32_error::*;
mod field_location_file;
pub(crate) use crate::field_location_file::*;
mod field_location_line;
pub(crate) use crate::field_location_line::*;
mod field_location_line_non_zero_u32;
pub(crate) use crate::field_location_line_non_zero_u32::*;
mod find_macro_attribute;
pub(crate) use crate::find_macro_attribute::*;
mod format_with_cargofmt;
pub(crate) use crate::format_with_cargofmt::*;
mod generate_const_new_token_stream_impl;
pub(crate) use crate::generate_const_new_token_stream_impl::*;
mod generate_const_try_new_token_stream_impl;
pub(crate) use crate::generate_const_try_new_token_stream_impl::*;
mod generate_field_location_new_token_stream;
pub(crate) use crate::generate_field_location_new_token_stream::*;
mod generate_if_write_is_error_token_stream;
pub(crate) use crate::generate_if_write_is_error_token_stream::*;
mod generate_impl_const_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_const_new_for_identifier_token_stream_impl::*;
mod generate_impl_default_token_stream;
pub(crate) use crate::generate_impl_default_token_stream::*;
mod generate_impl_display_token_stream;
pub(crate) use crate::generate_impl_display_token_stream::*;
mod generate_impl_from_token_stream;
pub(crate) use crate::generate_impl_from_token_stream::*;
mod generate_impl_modified_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_modified_new_for_identifier_token_stream_impl::*;
mod generate_impl_modified_try_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_modified_try_new_for_identifier_token_stream_impl::*;
mod generate_impl_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_const_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_pub_const_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_const_try_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_pub_const_try_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_pub_new_for_identifier_token_stream_impl::*;
mod generate_impl_pub_try_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_pub_try_new_for_identifier_token_stream_impl::*;
mod generate_impl_to_err_string_token_stream;
pub(crate) use crate::generate_impl_to_err_string_token_stream::*;
mod generate_impl_try_from_token_stream;
pub(crate) use crate::generate_impl_try_from_token_stream::*;
mod generate_impl_try_new_for_identifier_token_stream_impl;
pub(crate) use crate::generate_impl_try_new_for_identifier_token_stream_impl::*;
mod generate_modified_new_token_stream_impl;
pub(crate) use crate::generate_modified_new_token_stream_impl::*;
mod generate_modified_try_new_token_stream_impl;
pub(crate) use crate::generate_modified_try_new_token_stream_impl::*;
mod generate_new_or_try_new;
pub(crate) use crate::generate_new_or_try_new::*;
#[cfg(test)]
mod generate_new_or_try_new_tests;
#[cfg(test)]
pub(crate) use crate::generate_new_or_try_new_tests::*;
mod generate_new_token_stream_impl;
pub(crate) use crate::generate_new_token_stream_impl::*;
mod generate_pub_const_new_token_stream_impl;
pub(crate) use crate::generate_pub_const_new_token_stream_impl::*;
mod generate_pub_const_try_new_token_stream_impl;
pub(crate) use crate::generate_pub_const_try_new_token_stream_impl::*;
mod generate_pub_new_token_stream_impl;
pub(crate) use crate::generate_pub_new_token_stream_impl::*;
mod generate_pub_try_new_token_stream_impl;
pub(crate) use crate::generate_pub_try_new_token_stream_impl::*;
mod generate_pub_type_alias_token_stream;
pub(crate) use crate::generate_pub_type_alias_token_stream::*;
mod generate_serde_version_of_named_syn_variant;
pub(crate) use crate::generate_serde_version_of_named_syn_variant::*;
mod generate_simple_syn_punct;
pub(crate) use crate::generate_simple_syn_punct::*;
mod generate_try_new_token_stream_impl;
pub(crate) use crate::generate_try_new_token_stream_impl::*;
mod generated_file_maximum_bytes;
pub(crate) use crate::generated_file_maximum_bytes::*;
mod get_macro_attribute;
pub(crate) use crate::get_macro_attribute::*;
mod impl_identifier_token_stream_impl;
pub(crate) use crate::impl_identifier_token_stream_impl::*;
#[cfg(feature = "test-utils")]
mod json_contract;
#[cfg(feature = "test-utils")]
pub(crate) use crate::json_contract::*;
#[cfg(feature = "test-utils")]
mod json_fixture_ref;
#[cfg(feature = "test-utils")]
pub(crate) use crate::json_fixture_ref::*;
mod location;
pub(crate) use crate::location::*;
mod location_field_attr;
pub(crate) use crate::location_field_attr::*;
mod location_syn_field;
pub(crate) use crate::location_syn_field::*;
mod macro_attr_error;
pub(crate) use crate::macro_attr_error::*;
mod only_one;
pub(crate) use crate::only_one::*;
mod only_one_status_code_error;
pub(crate) use crate::only_one_status_code_error::*;
mod os_string_value;
pub(crate) use crate::os_string_value::*;
mod pagination_start_end_initialization_token_stream;
pub(crate) use crate::pagination_start_end_initialization_token_stream::*;
mod path_ref;
pub(crate) use crate::path_ref::*;
mod proc_macro2_derive_tokens_ref;
pub(crate) use crate::proc_macro2_derive_tokens_ref::*;
mod proc_macro2_generated_rust_token_stream;
pub(crate) use crate::proc_macro2_generated_rust_token_stream::*;
mod proc_macro2_if_write_is_err_token_stream;
pub(crate) use crate::proc_macro2_if_write_is_err_token_stream::*;
mod proc_macro2_macro_attr_meta_list_token_stream_ref;
pub(crate) use crate::proc_macro2_macro_attr_meta_list_token_stream_ref::*;
mod proc_macro2_token_stream_ref;
pub(crate) use crate::proc_macro2_token_stream_ref::*;
mod process_command;
pub(crate) use crate::process_command::*;
mod process_exit_status;
pub(crate) use crate::process_exit_status::*;
mod process_output;
pub(crate) use crate::process_output::*;
mod rs_file_path;
pub(crate) use crate::rs_file_path::*;
mod rs_file_path_buf;
pub(crate) use crate::rs_file_path_buf::*;
#[cfg(feature = "test-utils")]
mod sanitized_database_target;
#[cfg(feature = "test-utils")]
pub(crate) use crate::sanitized_database_target::*;
#[cfg(feature = "test-utils")]
mod serde_json_error;
#[cfg(feature = "test-utils")]
pub(crate) use crate::serde_json_error::*;
mod should_write_string;
pub(crate) use crate::should_write_string::*;
mod should_write_string_into_file;
pub(crate) use crate::should_write_string_into_file::*;
mod should_write_token_stream_into_file;
pub(crate) use crate::should_write_token_stream_into_file::*;
mod status_code;
pub(crate) use crate::status_code::*;
#[cfg(test)]
mod std_assert_file_path;
#[cfg(test)]
pub(crate) use crate::std_assert_file_path::*;
mod string_file_content_ref;
pub(crate) use crate::string_file_content_ref::*;
mod string_syn_punct;
pub(crate) use crate::string_syn_punct::*;
mod syn_field;
pub(crate) use crate::syn_field::*;
mod syn_field_identifier;
pub(crate) use crate::syn_field_identifier::*;
mod syn_field_type;
pub(crate) use crate::syn_field_type::*;
mod syn_field_vis;
pub(crate) use crate::syn_field_vis::*;
mod syn_location_field;
pub(crate) use crate::syn_location_field::*;
mod syn_macro_attr_ref;
pub(crate) use crate::syn_macro_attr_ref::*;
mod syn_path_segment;
pub(crate) use crate::syn_path_segment::*;
mod syn_path_segments;
pub(crate) use crate::syn_path_segments::*;
mod syn_variant_ref;
pub(crate) use crate::syn_variant_ref::*;
#[cfg(feature = "test-utils")]
mod test_database;
#[cfg(feature = "test-utils")]
pub(crate) use crate::test_database::*;
#[cfg(test)]
mod test_helper;
#[cfg(test)]
pub(crate) use crate::test_helper::*;
#[cfg(test)]
mod test_path;
#[cfg(test)]
pub(crate) use crate::test_path::*;
#[cfg(test)]
mod test_path_stem;
#[cfg(test)]
pub(crate) use crate::test_path_stem::*;
mod tool_arg_ref;
pub(crate) use crate::tool_arg_ref::*;
mod tool_args_ref;
pub(crate) use crate::tool_args_ref::*;
mod tool_command;
pub(crate) use crate::tool_command::*;
mod tool_env_key_ref;
pub(crate) use crate::tool_env_key_ref::*;
mod tool_env_value_ref;
pub(crate) use crate::tool_env_value_ref::*;
mod tool_program_ref;
pub(crate) use crate::tool_program_ref::*;
mod try_get_macro_attr_meta_list_token_stream;
pub(crate) use crate::try_get_macro_attr_meta_list_token_stream::*;
mod try_get_macro_attribute;
pub(crate) use crate::try_get_macro_attribute::*;
mod try_maybe_write_token_stream_into_file;
pub(crate) use crate::try_maybe_write_token_stream_into_file::*;
mod try_write_string_into_file;
pub(crate) use crate::try_write_string_into_file::*;
mod try_write_string_into_file_with_outcome;
pub(crate) use crate::try_write_string_into_file_with_outcome::*;
mod try_write_string_into_path;
pub(crate) use crate::try_write_string_into_path::*;
mod try_write_string_into_path_with_outcome;
pub(crate) use crate::try_write_string_into_path_with_outcome::*;
#[cfg(feature = "test-utils")]
mod url_error;
#[cfg(feature = "test-utils")]
pub(crate) use crate::url_error::*;
#[cfg(feature = "test-utils")]
mod url_ref;
#[cfg(feature = "test-utils")]
pub(crate) use crate::url_ref::*;
mod validate_existing_file_text;
pub(crate) use crate::validate_existing_file_text::*;
#[cfg(feature = "test-utils")]
mod validate_test_database_url;
#[cfg(feature = "test-utils")]
pub(crate) use crate::validate_test_database_url::*;
mod with_attr_token_stream_impl;
pub(crate) use crate::with_attr_token_stream_impl::*;
mod wrap_derive;
pub(crate) use crate::wrap_derive::*;
mod write_path_outcome;
pub(crate) use crate::write_path_outcome::*;
mod write_string_if_needed;
pub(crate) use crate::write_string_if_needed::*;
mod write_string_into_file;
pub(crate) use crate::write_string_into_file::*;
mod write_token_stream_into_file;
pub(crate) use crate::write_token_stream_into_file::*;
mod written_file_path_buf;
pub(crate) use crate::written_file_path_buf::*;
mod written_file_path_ref;
pub(crate) use crate::written_file_path_ref::*;

pub mod domain_types;
