#![allow(
    clippy::shadow_reuse,
    clippy::wildcard_imports,
    reason = "root-owned validator stages retain test-helper conversion naming and share the compatibility-facade vocabulary previously supplied by nested owner modules"
)]

#[cfg(test)]
pub mod assert_err_status_code;
#[cfg(test)]
pub mod assert_err_status_code_only;
#[cfg(test)]
pub mod assert_err_status_code_variant_ref;
#[cfg(test)]
pub mod assert_ok_eq;
#[cfg(test)]
pub mod assert_panics;
pub mod axum_body;
pub mod axum_body_size_error;
pub mod axum_commit_to_str_conversion_error;
#[cfg(test)]
pub mod axum_header_value_ref;
pub mod axum_headers_ref;
pub mod axum_http_status_code;
pub mod axum_http_status_code_provider;
#[cfg(test)]
pub mod axum_test_header_value;
#[cfg(test)]
pub mod axum_test_headers;
#[cfg(test)]
pub mod axum_test_headers_mut_ref;
pub mod body_size_error;
pub mod body_size_limit_bytes;
pub mod bytes_body_bytes;
pub mod check_body_size;
pub mod check_commit;
pub mod commit_error;
pub mod commit_header_name;
pub mod commit_not_eq_message;
pub mod commit_to_use;
pub mod enable_api_git_commit_check;
#[cfg(test)]
pub mod expect_err_variant_ref_with_status;
#[cfg(test)]
pub mod expect_error;
#[cfg(test)]
pub mod expect_error_mapped;
#[cfg(test)]
pub mod expect_error_variant_ref;
#[cfg(test)]
pub mod expect_ok;
#[cfg(test)]
pub mod expect_variant;
#[cfg(test)]
pub mod expect_variant_ref;
#[cfg(test)]
pub mod header_str_ref;
#[cfg(test)]
pub mod header_value_tests;
pub mod http_body_size_hint;
#[cfg(test)]
pub mod increment_block_on_poll_count;
#[cfg(test)]
pub mod insert_header_no_prev;
#[cfg(test)]
pub mod is_block_on_poll_limit_reached;
#[cfg(test)]
pub mod make_headers_with_entry;
#[cfg(test)]
pub mod map_err;
#[cfg(test)]
pub mod map_err_after_status_check;
#[cfg(test)]
pub mod map_or_panic_unexpected_variant;
#[cfg(test)]
pub mod max_block_on_polls;
pub mod no_commit_header_message;
#[cfg(test)]
pub mod non_utf8_header_value;
#[cfg(test)]
pub mod panic_unexpected_result;
#[cfg(test)]
pub mod panic_unexpected_variant;
#[cfg(test)]
pub mod poll_test_future;
#[cfg(test)]
pub mod read_commit_header_str;
#[cfg(test)]
pub mod replace_header_name;
#[cfg(test)]
pub mod required_header_str;
#[cfg(test)]
pub mod required_header_str_parsed;
#[cfg(test)]
pub mod required_header_value;
#[cfg(test)]
pub mod test_exp_id;
#[cfg(test)]
pub(crate) mod test_helper;
#[cfg(test)]
pub mod test_panic_text;
#[cfg(test)]
pub mod test_poll_count;
#[cfg(test)]
pub mod test_poll_limit_reached;
#[cfg(test)]
pub mod validate_commit_header;
#[cfg(test)]
pub mod validate_commit_header_value;
