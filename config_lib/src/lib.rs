#![allow(
    unused_imports,
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "root-owned modules retain the vocabulary and declaration grouping previously inherited from the configuration owner module"
)]

pub mod admin;
pub mod admin_access_token_ttl_seconds;
pub mod admin_cookie_secure;
pub mod admin_jwt;
pub mod admin_jwt_secret;
pub mod admin_jwt_secret_max_count;
pub mod admin_jwt_secret_min_len;
pub mod admin_login_failure_limit;
pub mod admin_password_hash_concurrency;
pub mod admin_refresh_token_ttl_seconds;
pub mod admin_session_limit;
pub mod admin_sign_in_rate_limit;
pub mod admin_swagger_enabled;
pub mod admin_token_audience;
pub mod admin_token_issuer;
pub mod chrono_fixed_offset_error;
pub mod chrono_timezone;
pub mod config_example_validity;
pub mod config_field_descriptor;
pub mod config_field_example_ref;
pub mod config_field_requirement;
pub mod config_field_sensitivity;
pub mod config_lib_string_wrapper_max_len;
pub mod config_lib_string_wrapper_try_from_string_error;
pub mod config_rust_type_name;
pub mod content_security_policy;
pub mod content_security_policy_error;
pub mod env_parse_error;
pub mod env_var_error;
pub mod env_var_name;
pub mod env_var_name_ref;
pub mod env_var_result_var_error;
#[cfg(test)]
pub mod env_var_value_ref;
pub mod http;
pub mod http_gzip_enabled;
pub mod i32_parse_int_error;
pub mod maximum_size_of_http_body_in_bytes;
pub mod maximum_size_of_http_body_in_bytes_try_from_usize_error;
pub mod parse_admin_positive_u64;
pub mod parse_admin_token_text;
pub mod parse_bool_error;
pub mod parse_ctx_ref;
pub mod parse_east_fixed_offset;
pub mod parse_env_var_name_ref;
pub mod parse_from_env_var_from_str;
pub mod parse_from_env_var_with;
pub mod parse_from_str_with_ctx;
pub mod parse_from_str_with_error;
pub mod parse_int_error;
pub mod parse_pg_pool_non_zero_seconds;
pub mod parse_required_env_var;
pub mod pg_pool;
pub mod pg_pool_acquire_timeout_seconds;
pub mod pg_pool_config_parse_error;
pub mod pg_pool_idle_timeout_seconds;
pub mod pg_pool_max_connections;
pub mod pg_pool_max_connections_try_from_u32_error;
pub mod pg_pool_max_lifetime_seconds;
pub mod pg_pool_min_connections;
pub mod production_mode;
pub mod request_timeout_seconds;
pub mod secrecy_secret_box_string;
pub mod src_place_type;
pub mod std_config_secret_string;
pub mod std_env_var_ok;
pub mod std_env_var_ok_ref;
pub mod svc_mode;
#[cfg(test)]
pub mod tests;
pub mod timezone_seconds;
pub mod tracing_format;
pub mod tracing_level;
pub mod tracing_level_name;
pub mod try_from_std_env_var_ok;
pub mod try_from_std_env_var_ok_admin_jwt_secret_error;
pub mod try_from_std_env_var_ok_admin_password_hash_concurrency_error;
pub mod try_from_std_env_var_ok_admin_positive_u64_error;
pub mod try_from_std_env_var_ok_admin_token_text_error;
pub mod try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error;
pub mod try_from_std_env_var_ok_pg_pool_max_connections_error;
pub mod try_from_std_env_var_ok_svc_mode_error;
pub mod try_from_std_env_var_ok_timezone_error;
pub mod try_map_non_empty_env_value;
pub mod types;
pub mod u32_parse_int_error;
pub mod usize_parse_int_error;

pub mod domain_types;
