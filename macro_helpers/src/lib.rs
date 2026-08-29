#![allow(
    unused_imports,
    clippy::arbitrary_source_item_ordering,
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    reason = "root-owned modules retain the vocabulary and compatibility namespaces previously inherited from nested owner modules"
)]

#[cfg(test)]
pub mod assert_file_content;
#[cfg(test)]
pub mod assert_file_path_ref;
pub mod attr_identifier_name;
pub mod attr_identifier_str;
#[cfg(test)]
pub mod cleanup_test_file;
pub mod compile_error_message;
#[cfg(feature = "test-utils")]
pub mod contract_error;
pub mod derive_token_stream_builder;
pub mod domain_types;
#[cfg(feature = "test-utils")]
pub mod ensure_json_contract_round_trip;
#[cfg(test)]
pub mod expected_file_content;
#[cfg(test)]
pub mod expected_file_content_ref;
pub mod field_location_column;
pub mod field_location_coordinate_try_from_u32_error;
pub mod field_location_file;
pub mod field_location_line;
pub mod find_macro_attribute;
pub mod format_with_cargofmt;
pub mod generate_const_new_token_stream_impl;
pub mod generate_const_try_new_token_stream_impl;
pub mod generate_field_location_new_token_stream;
pub mod generate_if_write_is_error_token_stream;
pub mod generate_impl_const_new_for_identifier_token_stream_impl;
pub mod generate_impl_default_token_stream;
pub mod generate_impl_display_token_stream;
pub mod generate_impl_from_token_stream;
pub mod generate_impl_modified_new_for_identifier_token_stream_impl;
pub mod generate_impl_modified_try_new_for_identifier_token_stream_impl;
pub mod generate_impl_new_for_identifier_token_stream_impl;
pub mod generate_impl_pub_const_new_for_identifier_token_stream_impl;
pub mod generate_impl_pub_const_try_new_for_identifier_token_stream_impl;
pub mod generate_impl_pub_new_for_identifier_token_stream_impl;
pub mod generate_impl_pub_try_new_for_identifier_token_stream_impl;
pub mod generate_impl_to_err_string_token_stream;
pub mod generate_impl_try_from_token_stream;
pub mod generate_impl_try_new_for_identifier_token_stream_impl;
pub mod generate_modified_new_token_stream_impl;
pub mod generate_modified_try_new_token_stream_impl;
#[cfg(test)]
pub mod generate_new_or_try_new_tests;
pub mod generate_new_token_stream_impl;
pub mod generate_pub_const_new_token_stream_impl;
pub mod generate_pub_const_try_new_token_stream_impl;
pub mod generate_pub_new_token_stream_impl;
pub mod generate_pub_try_new_token_stream_impl;
pub mod generate_pub_type_alias_token_stream;
pub mod generate_serde_version_of_named_syn_variant;
pub mod generate_simple_syn_punct;
pub mod generate_try_new_token_stream_impl;
pub mod generated_file_maximum_bytes;
#[cfg(test)]
pub mod get_macro_attribute_tests;
pub mod impl_identifier_token_stream_impl;
#[cfg(feature = "test-utils")]
#[cfg(test)]
pub mod json_contract_tests;
#[cfg(feature = "test-utils")]
pub mod json_fixture_ref;
pub mod location_field_attr;
pub mod location_syn_field;
pub mod macro_attr_error;
pub mod macro_compile_error_tokens;
pub mod only_one;
pub mod only_one_status_code_error;
pub mod os_string_value;
pub mod pagination_start_end_initialization_token_stream;
pub mod path_ref;
pub mod proc_macro2_derive_tokens_ref;
pub mod proc_macro2_generated_rust_token_stream;
pub mod proc_macro2_if_write_is_err_token_stream;
pub mod proc_macro2_macro_attr_meta_list_token_stream_ref;
pub mod proc_macro2_token_stream_ref;
pub mod process_command;
pub mod process_exit_status;
pub mod process_output;
pub mod rs_file_path_buf;
#[cfg(test)]
pub mod rs_file_path_tests;
#[cfg(feature = "test-utils")]
pub mod sanitized_database_target;
#[cfg(feature = "test-utils")]
pub mod serde_json_error;
pub mod should_write_string;
#[cfg(test)]
pub mod should_write_string_into_file_tests;
pub mod should_write_token_stream_into_file;
pub mod status_code;
#[cfg(test)]
pub mod std_assert_file_path;
pub mod string_file_content_ref;
pub mod string_syn_punct;
pub mod syn_field;
pub mod syn_field_identifier;
pub mod syn_field_type;
pub mod syn_field_vis;
pub mod syn_location_field;
pub mod syn_macro_attr_ref;
pub mod syn_path_segment;
pub mod syn_path_segments;
pub mod syn_variant_ref;
#[cfg(feature = "test-utils")]
pub mod test_database;
#[cfg(test)]
pub mod test_path;
#[cfg(test)]
pub mod test_path_stem;
pub mod tool_arg_ref;
pub mod tool_args_ref;
pub mod tool_command;
pub mod tool_env_key_ref;
pub mod tool_env_value_ref;
pub mod tool_program_ref;
pub mod try_get_macro_attr_meta_list_token_stream;
pub mod try_get_macro_attribute;
pub mod try_maybe_write_token_stream_into_file;
pub mod try_write_string_into_file;
pub mod try_write_string_into_file_with_outcome;
#[cfg(test)]
pub mod try_write_string_into_path_tests;
#[cfg(test)]
pub mod try_write_string_into_path_with_outcome_tests;
#[cfg(feature = "test-utils")]
pub mod url_error;
#[cfg(feature = "test-utils")]
pub mod url_ref;
pub mod validate_existing_file_text;
#[cfg(feature = "test-utils")]
pub mod validate_test_database_url;
pub mod with_attr_token_stream_impl;
pub mod wrap_derive;
pub mod write_path_outcome;
#[cfg(test)]
pub mod write_string_if_needed_tests;
#[cfg(test)]
pub mod write_string_into_file_tests;
#[cfg(test)]
pub mod write_token_stream_into_file_tests;
pub mod written_file_path_buf;
pub mod written_file_path_ref;
