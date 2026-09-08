#![allow(
    unused_crate_dependencies,
    reason = "proc_macro_frontend_contract is a dependency of the trybuild fixture crates"
)]

#[cfg(test)]
pub mod analyzer_bool;
#[cfg(test)]
pub mod analyzer_count;
#[cfg(test)]
pub mod cargo_metadata;
#[cfg(test)]
pub mod cargo_metadata_ref;
#[cfg(test)]
pub mod cargo_package_id_ref_hash_set;
#[cfg(test)]
pub mod cargo_toml_file_index;
#[cfg(test)]
pub mod code_style;
#[cfg(test)]
pub mod diagnostic_messages;
#[cfg(test)]
pub mod diagnostic_messages_mut_ref;
#[cfg(test)]
pub mod domain_analysis;
#[cfg(test)]
pub mod function_body_hash;
#[cfg(test)]
pub mod function_body_locations_b_tree_map;
#[cfg(test)]
pub mod function_body_locations_b_tree_map_mut_ref;
#[cfg(test)]
pub mod owned_path_buf;
#[cfg(test)]
pub mod path_ref;
#[cfg(test)]
pub mod regex_regex_ref;
#[cfg(test)]
pub mod rs_source_files_ref;
#[cfg(test)]
pub mod runtime_analysis;
#[cfg(test)]
pub mod source_analysis;
#[cfg(test)]
pub mod source_text;
#[cfg(test)]
pub mod source_text_b_tree_set;
#[cfg(test)]
pub mod source_text_b_tree_set_ref;
#[cfg(test)]
pub mod source_text_hash_set;
#[cfg(test)]
pub mod source_text_list;
#[cfg(test)]
pub mod source_text_list_ref;
#[cfg(test)]
pub mod source_text_ref;
#[cfg(test)]
pub mod source_text_ref_hash_set;
#[cfg(test)]
pub mod source_text_try_from_string_error;
#[cfg(test)]
pub mod static_str;
#[cfg(test)]
pub mod static_str_slice_ref;
#[cfg(test)]
pub mod syn_attribute_list_ref;
#[cfg(test)]
pub mod syn_attribute_ref;
#[cfg(test)]
pub mod syn_block_ref;
#[cfg(test)]
pub mod syn_expr_call_ref;
#[cfg(test)]
pub mod syn_fields_ref;
#[cfg(test)]
pub mod syn_file;
#[cfg(test)]
pub mod syn_file_ref;
#[cfg(test)]
pub mod syn_generics_ref;
#[cfg(test)]
pub mod syn_identifier_ref;
#[cfg(test)]
pub mod syn_item_fn_ref;
#[cfg(test)]
pub mod syn_item_impl_ref;
#[cfg(test)]
pub mod syn_item_ref;
#[cfg(test)]
pub mod syn_item_struct_ref;
#[cfg(test)]
pub mod syn_path_arguments_ref;
#[cfg(test)]
pub mod syn_path_ref;
#[cfg(test)]
pub mod syn_path_segment_ref;
#[cfg(test)]
pub mod syn_signature_ref;
#[cfg(test)]
pub mod syn_type_path_ref;
#[cfg(test)]
pub mod syn_type_ref;
#[cfg(test)]
pub mod syn_use_tree_ref;
#[cfg(test)]
pub mod test_code_style_advanced_policy;
#[cfg(test)]
pub mod test_code_style_cargo_policy;
#[cfg(test)]
pub mod test_code_style_ci_policy;
#[cfg(test)]
pub mod test_code_style_contract_source_policy;
#[cfg(test)]
pub mod test_code_style_deployment_policy;
#[cfg(test)]
pub mod test_code_style_domain_type_policy;
#[cfg(test)]
pub mod test_code_style_lint_sync;
#[cfg(test)]
pub mod test_code_style_module_policy;
#[cfg(test)]
pub mod test_code_style_reuse_policy;
#[cfg(test)]
pub mod test_code_style_route_contract_policy;
#[cfg(test)]
pub mod test_code_style_runtime_policy;
#[cfg(test)]
pub mod test_code_style_secret_policy;
#[cfg(test)]
pub mod test_code_style_snapshot;
#[cfg(test)]
pub mod test_code_style_source_policy;
#[cfg(test)]
pub mod toml_table;
#[cfg(test)]
pub mod toml_table_ref;
#[cfg(test)]
pub mod toml_value_ref;
#[cfg(test)]
pub mod walkdir_walk_dir;
