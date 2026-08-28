#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "validators stay adjacent to their domain wrappers and ranges retain minimum-then-maximum order"
)]

mod bounded_text_policy_error;
pub mod domain_types;
mod fixed_length_ascii_hex_text;
mod fixed_length_ascii_hex_text_error;
mod non_empty_trimmed_text;
mod password_length;
mod password_length_range;
mod password_length_range_error;
mod password_policy_violation;
mod password_text_ref;
mod required_nul_free_bounded_text;
mod url_safe_token_part_maximum_bytes;
mod url_safe_token_part_ref;
mod url_safe_token_part_text;
mod url_safe_token_part_text_error;
mod validate_password_policy;
mod validate_url_safe_token_part;
