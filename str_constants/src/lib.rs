pub mod expr;
pub mod env_names {
    pub const CORS_ALLOW_ORIGIN: &str = "CORS_ALLOW_ORIGIN";
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const ENABLE_API_GIT_COMMIT_CHECK: &str = "ENABLE_API_GIT_COMMIT_CHECK";
    pub const MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES: &str = "MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES";
    pub const PG_POOL_MAX_CONNECTIONS: &str = "PG_POOL_MAX_CONNECTIONS";
    pub const SERVICE_SOCKET_ADDRESS: &str = "SERVICE_SOCKET_ADDRESS";
    pub const SRC_PLACE_TYPE: &str = "SRC_PLACE_TYPE";
    pub const TIMEZONE: &str = "TIMEZONE";
    pub const TRACING_LEVEL: &str = "TRACING_LEVEL";
}
pub mod http_header_names {
    pub const X_API_GIT_COMMIT: &str = "x-api-git-commit";
    pub const X_REQUEST_ID: &str = "x-request-id";
}
pub mod route_paths {
    pub const HEALTH: &str = "/health";
    pub const NOT_FOUND: &str = "/404";
}
pub mod sql_names {
    pub const ID: &str = "id";
}
pub mod common_routes {
    pub const GIT_INFO: &str = "/git_info";
    pub const HEALTH: &str = "/health";
    pub const HEALTH_CHECK: &str = "/health_check";
    pub const HEALTH_CHECK_SQL: &str = "SELECT 1";
    pub const HEALTH_LIVE: &str = "/health/live";
    pub const HEALTH_READY: &str = "/health/ready";
    pub const NO_ROUTE_MSG_PREFIX: &str = "No route for ";
    pub const SWAGGER_UI: &str = "/swagger-ui";
}
pub mod config {
    pub const ENV_VALUE_IS_EMPTY_MSG: &str = "is empty";
    pub const SRC_PLACE_TYPE_FIX_MSG: &str =
        "You can set environment variable SRC_PLACE_TYPE to be eq \"src\" or \"github\"";
    pub const SRC_PLACE_TYPE_PARSE_CTX: &str = "<SrcPlaceType as std::str::FromStr>::from_str(&v)";
    pub const TIMEZONE_NOT_EAST_MSG: &str = "not east";
    pub const TRACING_DEBUG: &str = "debug";
    pub const TRACING_ERROR: &str = "error";
    pub const TRACING_INFO: &str = "info";
    pub const TRACING_TRACE: &str = "trace";
    pub const TRACING_WARN: &str = "warn";
}
pub mod git_info {
    pub const PROJECT_GIT_COMMIT_ID: &str =
        git_version::git_version!(args = ["--always", "--abbrev=40"]);
    pub const PROJECT_GIT_COMMIT_LINK: &str = git_version::git_version!(
        args = ["--always", "--abbrev=40"],
        prefix = "https://github.com/kuqmua/rust_workspace_template/tree/"
    );
    pub const TREE_SEGMENT: &str = "/tree/";
}
pub mod location {
    pub const INCORRECT_DATETIME_MSG: &str = "incorrect datetime";
}
pub mod macro_diagnostics {
    pub const AS_REF_INNER_SHARED_REF_ERROR: &str =
        "#[newtype(as_ref_inner)] requires a shared reference inner type";
    pub const BOUNDED_STRING_MAX_ERROR: &str =
        "BoundedString requires #[bounded_string(max = ...)]";
    pub const CASE_TRAIT_PAIR_EXPECTED_CLOSURE_ERROR: &str = "case_trait_pair expects closure";
    pub const CASE_TRAIT_PAIR_EXPECTED_PARTS_ERROR: &str =
        "case_trait_pair expects str trait, ts trait, bound, closure expr";
    pub const CASE_TRAIT_PAIR_EXPECTED_STR_TRAIT_ERROR: &str =
        "case_trait_pair expects string trait name";
    pub const CASE_TRAIT_PAIR_EXPECTED_TS_TRAIT_ERROR: &str =
        "case_trait_pair expects token trait name";
    pub const CASE_TRAIT_PAIR_EXPECTED_BOUND_ERROR: &str = "case_trait_pair expects bound";
    pub const CASE_TRAIT_PAIR_PARSE_BODY_ERROR: &str = "case_trait_pair failed to parse body";
    pub const DUPLICATE_BOUNDED_STRING_OPTION_ERROR: &str = "duplicate bounded_string option";
    pub const EXPECTED_ANGLE_BRACKETED_ARGS_ERROR: &str = "07c6ab44: expected angle bracketed args";
    pub const EXPECTED_FIRST_PATH_SEGMENT_ERROR: &str = "595050cf: expected first path segment";
    pub const EXPECTED_HASH_MAP_C1_ERROR: &str = "c1d03b71: expected HashMap<K, T>";
    pub const EXPECTED_HASH_MAP_C8_ERROR: &str = "c828da34: expected HashMap<K, T>";
    pub const EXPECTED_HASH_MAP_E9_ERROR: &str = "e9c6a7d2: expected HashMap<K, T>";
    pub const EXPECTED_NAMED_FIELD_A2_ERROR: &str = "a21dc807: expected named field identifier";
    pub const EXPECTED_NAMED_FIELD_ERROR: &str = "438aa90e: expected named field identifier";
    pub const EXPECTED_NAMED_VARIANT_FIELDS_ERROR: &str = "79b0f231: expected named variant fields";
    pub const PRIMARY_KEY_FIELD_INDEX_ERROR: &str = "878d3f9b: primary key field index not found";
    pub const TUPLE_STRUCT_ERROR: &str = "Newtype supports only tuple structs";
}
pub mod macro_clippy {
    pub const CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 3] =
        ["check", "--all-targets", "--all-features"];
    pub const CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 22] = [
        "clippy",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings",
        "-A",
        "clippy::bool_assert_comparison",
        "-A",
        "clippy::clone_on_copy",
        "-A",
        "clippy::collapsible_if",
        "-A",
        "clippy::let_and_return",
        "-A",
        "clippy::result_large_err",
        "-A",
        "clippy::single_call_fn",
        "-A",
        "clippy::useless_borrows_in_formatting",
        "-A",
        "clippy::write_literal",
    ];
    pub const CARGO_FMT_ARGS: [&str; 1] = ["fmt"];
    pub const CARGO_TEST_LIB_ARGS: [&str; 2] = ["test", "--lib"];
}
pub mod compile_error {
    pub const ERROR_PLACEHOLDER: &str = "{error}";
    pub const CE_000: &str = "10764d2b: expected named variant fields";
    pub const CE_001: &str = "10773d36: expected named variant fields";
    pub const CE_002: &str =
        "1266ae5a: field identifier is longer than PostgreSQL column name limit";
    pub const CE_003: &str = "1a75cea1: duplicate primary key field";
    pub const CE_004: &str = "1be4a6e2: expected named variant fields";
    pub const CE_005: &str = "201dc0a4: {error}";
    pub const CE_006: &str = "22bc6672: non-primary-key field index not found";
    pub const CE_007: &str = "22c364b9: {error}";
    pub const CE_008: &str = "2acd4725: expected named variant fields";
    pub const CE_009: &str = "2ad2130d: primary key type must be a path";
    pub const CE_010: &str = "2db209a8: {error}";
    pub const CE_011: &str = "35d30bd7: frontend field order values must be unique";
    pub const CE_012: &str =
        "45dff0e2: optimistic_revision_field must be a non-primary-key signed 64-bit field";
    pub const CE_013: &str = "536203b7: bulk item limit must be greater than zero";
    pub const CE_015: &str = "6a529a99: primary key field not found";
    pub const CE_016: &str = "6d0adac1: cloned primary key type path has no segments";
    pub const CE_017: &str =
        "741aa5f9: create_exclude_fields must contain unique non-primary-key field names";
    pub const CE_018: &str = "7f31872d: expected named struct fields";
    pub const CE_019: &str = "81efa954: status code attr not found";
    pub const CE_020: &str = "86307dbc: {error}";
    pub const CE_021: &str = "8a5fbef9: frontend field configuration count does not match fields";
    pub const CE_022: &str = "8a66c852: error variant attr identifier does not match attr name";
    pub const CE_023: &str = "8af68998: location field attr not found";
    pub const CE_024: &str = "8d93bf20: expected path type";
    pub const CE_025: &str = "8dcafc1c: expected named variant fields";
    pub const CE_026: &str = "915ef2ce: expected named field identifier";
    pub const CE_027: &str =
        "91a3d9f2: read_exclude_fields must contain unique non-primary-key field names";
    pub const CE_028: &str = "9a469d36: duplicate location field attr";
    pub const CE_029: &str = "9a4d65c9: duplicate location field attr";
    pub const CE_030: &str = "ae8e173b: expected named variant field identifier";
    pub const CE_031: &str = "assert_empty_parse_err_matches expects pattern";
    pub const CE_032: &str = "assert_empty_parse_err_matches expects type";
    pub const CE_033: &str = "assert_empty_parse_err_matches expects type, pattern";
    pub const CE_034: &str = "assert_parse_err_matches expects pattern";
    pub const CE_035: &str = "assert_parse_err_matches expects type";
    pub const CE_036: &str = "assert_parse_err_matches expects type, value, pattern";
    pub const CE_037: &str = "assert_parse_err_matches expects value";
    pub const CE_038: &str = "assert_parse_ok_matches expects pattern";
    pub const CE_039: &str = "assert_parse_ok_matches expects type";
    pub const CE_040: &str = "assert_parse_ok_matches expects type, value, pattern";
    pub const CE_041: &str = "assert_parse_ok_matches expects value";
    pub const CE_042: &str = "b9f53bee: location field attr not found";
    pub const CE_043: &str = "bd4718d0: expected struct input";
    pub const CE_044: &str = "bool_enum_to_tokens expects comma after enum name";
    pub const CE_045: &str = "bool_enum_to_tokens expects enum name";
    pub const CE_046: &str = "bool_enum_to_tokens expects false => expr";
    pub const CE_047: &str = "bool_enum_to_tokens expects true => expr";
    pub const CE_048: &str = "bool_enum_to_tokens failed to parse false expr";
    pub const CE_049: &str = "bool_enum_to_tokens failed to parse true expr";
    pub const CE_050: &str = "d1003b2e: location field attr not found";
    pub const CE_051: &str =
        "d5f1b3a7: permission prefix must use lowercase ASCII letters, digits, or underscores";
    pub const CE_052: &str = "e7408836: primary key type path has no segments";
    pub const CE_053: &str = "e9b33787: expected first generic arg";
    pub const CE_054: &str = "edbbd08a: expected named field identifier";
    pub const CE_055: &str = "f7ea4b19: optimistic_revision_field must name an existing field";
    pub const CE_056: &str = "impl_cfg_getter expects fn name";
    pub const CE_057: &str = "impl_cfg_getter expects return type";
    pub const CE_058: &str = "impl_cfg_getter expects trait name";
    pub const CE_059: &str = "impl_cfg_getter expects trait, fn, ret_ty";
    pub const CE_060: &str = "impl_to_err_string_const expects type => message";
    pub const CE_061: &str = "impl_to_err_string_with expects closure";
    pub const CE_062: &str = "impl_to_err_string_with expects types => |value| body";
    pub const CE_063: &str = "impl_try_from_non_empty_string expects error name";
    pub const CE_064: &str = "impl_try_from_non_empty_string expects name";
    pub const CE_065: &str = "impl_try_from_non_empty_string expects name, error name";
    pub const CE_066: &str = "impl_try_from_parse expects error field";
    pub const CE_067: &str = "impl_try_from_parse expects error name";
    pub const CE_068: &str = "impl_try_from_parse expects error variant";
    pub const CE_069: &str = "impl_try_from_parse expects inner type";
    pub const CE_070: &str = "impl_try_from_parse expects name";
    pub const CE_071: &str =
        "impl_try_from_parse expects name, error name, inner type and error variant";
    pub const CE_072: &str = "impl_try_from_secret_url expects error name";
    pub const CE_073: &str = "impl_try_from_secret_url expects name";
    pub const CE_074: &str = "impl_try_from_secret_url expects name, error name";
    pub const CE_075: &str = "tp expects comma after type name";
    pub const CE_076: &str = "tp expects type name";
    pub const CE_077: &str = "tp_parts expects type name";
    pub const CE_078: &str = "tp_parts expects type name and at least one part";
    pub const CE_079: &str = "trait_alias expects Name = Bounds";
    pub const CE_080: &str = "trait_alias failed to parse bounds";
    pub const CE_081: &str = "ts_path_fn expects comma after function name";
    pub const CE_082: &str = "ts_path_fn expects function name";
}
pub mod naming {
    pub const GITHUB_URL: &str = "https://github.com/kuqmua/rust_workspace_template";
    pub const REGEX_VALUE: &str = "^[a-zA-Z0-9]+$";
}
pub mod panic_location {
    pub const NO_LOCATION_MSG: &str = "panic occurred but can't get location information...";
}
pub mod pg_crud {
    pub const ADJACENT_SQL_OPERATOR: &str = "-|-";
    pub const BEFORE_SQL_OPERATOR: &str = "<";
    pub const CONTAINS_SQL_OPERATOR: &str = "@>";
    pub const EMPTY_SQL_SUFFIX: &str = "";
    pub const EQUALITY_SQL_OPERATOR: &str = "=";
    pub const LEFT_OF_SQL_OPERATOR: &str = "&<";
    pub const OVERLAPS_SQL_OPERATOR: &str = "&&";
    pub const RIGHT_OF_SQL_OPERATOR: &str = "&>";
    pub const TEXT_SEARCH_SQL_OPERATOR: &str = "ILIKE";
    pub const TEXT_SEARCH_SQL_SUFFIX: &str = "ESCAPE '\\'";
    pub const WITHIN_SQL_OPERATOR: &str = "<@";
    pub const CREATE_PERMISSION_ACTION: &str = "create";
    pub const DELETE_PERMISSION_ACTION: &str = "delete";
    pub const READ_PERMISSION_ACTION: &str = "read";
    pub const UPDATE_PERMISSION_ACTION: &str = "update";
    pub const STANDARD_FORMAT_ARGUMENT: &str = "";
    pub const PG_BIGSERIAL: &str = "bigserial";
    pub const PG_BOOL: &str = "bool";
    pub const PG_BYTEA: &str = "bytea";
    pub const PG_DATE: &str = "date";
    pub const PG_DATERANGE: &str = "daterange";
    pub const PG_FLOAT4: &str = "float4";
    pub const PG_FLOAT8: &str = "float8";
    pub const PG_INET: &str = "inet";
    pub const PG_INT2: &str = "int2";
    pub const PG_INT4: &str = "int4";
    pub const PG_INT4RANGE: &str = "int4range";
    pub const PG_INT8: &str = "int8";
    pub const PG_INT8RANGE: &str = "int8range";
    pub const PG_INTERVAL: &str = "interval";
    pub const PG_MACADDR: &str = "macaddr";
    pub const PG_MONEY: &str = "money";
    pub const PG_SERIAL: &str = "serial";
    pub const PG_SMALLSERIAL: &str = "smallserial";
    pub const PG_TEXT: &str = "text";
    pub const PG_TIME: &str = "time";
    pub const PG_TIMESTAMP: &str = "timestamp";
    pub const PG_TIMESTAMPTZ: &str = "timestamptz";
    pub const PG_TSRANGE: &str = "tsrange";
    pub const PG_TSTZRANGE: &str = "tstzrange";
    pub const PG_UUID: &str = "uuid";
    pub const BETWEEN_EXPECTING: &str = "struct Between with 2 els";
    pub const BETWEEN_SCHEMA_NAME: &str = "Between";
    pub const BETWEEN_STRUCT_NAME: &str = "struct Between";
    pub const END_FIELD: &str = "end";
    pub const FIELD_IDENTIFIER: &str = "field identifier";
    pub const COMPLETE_IDEMPOTENCY_SQL: &str = "UPDATE pg_table_idempotency SET state='completed',response_status=$6,response_body=$7,completed_at=NOW() WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4 AND request_hash=$5 AND state='pending'";
    pub const GENERATE_PG_TABLE_CONFIG_PATH: &str = "generate_pg_table::generate_pg_table_config";
    pub const NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME: &str = "NotEmptyUniqueVec";
    pub const NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING: &str =
        "tuple struct NotEmptyUniqueVec with 1 element";
    pub const NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME: &str = "tuple struct NotEmptyUniqueVec";
    pub const OPERATOR_FIELD: &str = "operator";
    pub const PG_TYPE_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME: &str = "PgTypeNotEmptyUniqueVec";
    pub const PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING: &str =
        "tuple struct PgTypeNotEmptyUniqueVec with 1 element";
    pub const PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME: &str =
        "tuple struct PgTypeNotEmptyUniqueVec";
    pub const PG_TYPE_WHERE_SCHEMA_NAME: &str = "PgTypeWhere";
    pub const PG_TYPE_WHERE_STRUCT_NAME: &str = "struct PgTypeWhere";
    pub const PG_TYPE_WHERE_EXPECTING: &str = "struct PgTypeWhere with 2 els";
    pub const SERDE_BETWEEN_FIELDS: &[&str] = &["start", "end"];
    pub const SERDE_PG_TYPE_WHERE_FIELDS: &[&str] = &["operator", "v"];
    pub const REGEX_REGEX_SCHEMA_ID: &str = "tests::RegexRegex";
    pub const REGEX_REGEX_SCHEMA_NAME: &str = "RegexRegex";
    pub const START_FIELD: &str = "start";
    pub const V_FIELD: &str = "v";
}
pub mod route_validators {
    pub const BLOCK_ON_POLL_LIMIT_ER_ID: &str = "cf6e91ab";
    pub const COMMIT_NOT_EQ_MSG: &str =
        "different project commit provided, services must work only with eq project commits";
    pub const NO_COMMIT_HEADER_MSG: &str = "no_commit_header";
    pub const EXPECT_ER_ER_ID: &str = "2f755472";
    pub const EXPECT_OK_ER_ID: &str = "db9d2f63";
    pub const REPLACE_HEADER_MISSING_SRC_ER_ID: &str = "c3a0f7be";
    pub const COMMIT_HEADER_NAME: &str = "commit";
    pub const TEST_HEADER_NAME: &str = "x-test-header";
}
pub mod runtime {
    pub const CORRELATION_ID_HEADER_NAME: &str = "x-correlation-id";
    pub const FORWARDED_FOR_HEADER_NAME: &str = "x-forwarded-for";
    pub const REAL_IP_HEADER_NAME: &str = "x-real-ip";
    pub const REQUEST_ID_HEADER_NAME: &str = "x-request-id";
}
pub mod server {
    pub const TRACING_DFLT_FILTER: &str = "info";
}
pub mod server_admin {
    pub const ACCESS_COOKIE_NAME: &str = "admin_access_token";
    pub const ACTIVE_ADMIN_COUNT_SQL: &str = "SELECT COUNT(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id JOIN admin_roles role ON role.id = user_role.role_id WHERE role.name = 'admin' AND users.is_banned = FALSE";
    pub const API_PREFIX: &str = "/api/v1/admin";
    pub const INSERT_USER_SQL: &str = "INSERT INTO admin_users (login, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id";
    pub const LOCK_LAST_ADMIN_SQL: &str =
        "SELECT pg_advisory_xact_lock(hashtext('admin_last_active_administrator'))";
    pub const REVOKE_ACCESS_SESSION_SQL: &str = "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL";
    pub const REVOKE_USER_ACCESS_SESSIONS_SQL: &str = "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL";
    pub const REVOKE_USER_REFRESH_TOKENS_SQL: &str = "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL";
    pub const USER_IS_ADMIN_SQL: &str = "SELECT EXISTS (SELECT 1 FROM admin_user_roles user_role JOIN admin_roles role ON role.id = user_role.role_id WHERE user_role.user_id = $1 AND role.name = 'admin')";
    pub const RATE_LIMIT_AUDIT_READ: &str = "audit_read";
    pub const RATE_LIMIT_MUTATION: &str = "mutation";
    pub const RATE_LIMIT_REFRESH_IP: &str = "refresh_ip";
    pub const RATE_LIMIT_SIGN_IN_IP: &str = "sign_in_ip";
    pub const RATE_LIMIT_SIGN_IN_IP_LOGIN: &str = "sign_in_ip_login";
}
pub mod admin_api_paths {
    pub const AUDIT: &str = "/audit-log";
    pub const AUTH_ME: &str = "/auth/me";
    pub const AUTH_REFRESH: &str = "/auth/refresh";
    pub const AUTH_SESSION: &str = "/auth/sessions/{session_id}";
    pub const AUTH_SESSIONS: &str = "/auth/sessions";
    pub const AUTH_SIGN_IN: &str = "/auth/sign-in";
    pub const AUTH_SIGN_OUT: &str = "/auth/sign-out";
    pub const PERMISSIONS: &str = "/permissions";
    pub const ROLE: &str = "/roles/{role_id}";
    pub const ROLE_PERMISSIONS: &str = "/roles/{role_id}/permissions";
    pub const ROLES: &str = "/roles";
    pub const SETTINGS: &str = "/system-settings";
    pub const USER: &str = "/users/{user_id}";
    pub const USER_BAN: &str = "/users/{user_id}/ban";
    pub const USER_PASSWORD: &str = "/users/{user_id}/password";
    pub const USER_ROLES: &str = "/users/{user_id}/roles";
    pub const USERS: &str = "/users";
}
pub mod admin_page_paths {
    pub const ALL: [&str; 10] = [
        ROOT,
        SIGN_IN,
        USERS,
        ROLES,
        PERMISSIONS,
        AUDIT,
        SETTINGS,
        METRICS,
        VERSION,
        OPEN_API,
    ];
    pub const ASSETS: &str = "/admin/assets";
    pub const AUDIT: &str = "/admin/audit-log";
    pub const METRICS: &str = "/admin/metrics";
    pub const OPEN_API: &str = "/admin/swagger-ui";
    pub const OPEN_API_DOCUMENT: &str = "/admin/openapi.json";
    pub const PERMISSIONS: &str = "/admin/permissions";
    pub const ROLES: &str = "/admin/roles";
    pub const ROOT: &str = "/admin";
    pub const SETTINGS: &str = "/admin/system-settings";
    pub const SIGN_IN: &str = "/admin/sign-in";
    pub const USERS: &str = "/admin/users";
    pub const VERSION: &str = "/admin/version";
}
pub mod admin_permission_values {
    pub const AUDIT_LOG_READ: &str = "audit_log:read";
    pub const METRICS_READ: &str = "metrics:read";
    pub const OPEN_API_READ: &str = "openapi:read";
    pub const PERMISSIONS_READ: &str = "permissions:read";
    pub const ROLE_PERMISSIONS_CREATE: &str = "role_permissions:create";
    pub const ROLE_PERMISSIONS_DELETE: &str = "role_permissions:delete";
    pub const ROLE_PERMISSIONS_READ: &str = "role_permissions:read";
    pub const ROLE_PERMISSIONS_UPDATE: &str = "role_permissions:update";
    pub const ROLES_CREATE: &str = "roles:create";
    pub const ROLES_DELETE: &str = "roles:delete";
    pub const ROLES_READ: &str = "roles:read";
    pub const ROLES_UPDATE: &str = "roles:update";
    pub const SYSTEM_SETTINGS_READ: &str = "system_settings:read";
    pub const SYSTEM_SETTINGS_UPDATE: &str = "system_settings:update";
    pub const USER_ROLES_CREATE: &str = "user_roles:create";
    pub const USER_ROLES_DELETE: &str = "user_roles:delete";
    pub const USER_ROLES_READ: &str = "user_roles:read";
    pub const USER_ROLES_UPDATE: &str = "user_roles:update";
    pub const USERS_CREATE: &str = "users:create";
    pub const USERS_DELETE: &str = "users:delete";
    pub const USERS_READ: &str = "users:read";
    pub const USERS_UPDATE: &str = "users:update";
}
pub mod test_values {
    pub const COMMIT: &str = "abc123";
    pub const OPEN_API_TABLE_EXAMPLE_PATH_PREFIX: &str = "/paths/~1table_example~1";
    pub const UNREACHABLE_DATABASE_URL: &str = "postgres://usr:pwd@127.0.0.1:1/unreachable";
    pub const WRONG_COMMIT: &str = "deadbeef";
}
pub mod code_style {
    pub const GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME: &str =
        "generate_derive_token_stream_builder";
    pub const GENERATE_PG_TYPES_MACRO_NAME: &str = "generate_pg_types";
    pub const GENERATE_WHERE_FILTERS_MACRO_NAME: &str = "generate_where_filters";
    pub const STR_CONSTANTS_EXPR_PATH: &str = "../str_constants/src/expr.rs";
    pub const STRING_GUARD_ALLOWED_SYNTAX_FIXTURE: &str =
        "#[path = \"fixture.rs\"] mod fixture; fn f() { value.expect(\"12345678\"); }";
    pub const STRING_GUARD_DETECTION_FIXTURE: &str =
        "fn f() { consume(\"ordinary\"); outer!(inner(\"macro\")); }";
    pub const CI_WORKFLOW_PATH: &str = ".github/workflows/ci.yml";
    pub const CLIPPY_LINT_EXCEPTIONS: [&str; 22] = [
        "disallowed_fields",
        "unnecessary_trailing_comma",
        "manual_pop_if",
        "assign_ops",
        "extend_from_slice",
        "match_on_vec_items",
        "misaligned_transmute",
        "option_map_or_err_ok",
        "pub_enum_variant_names",
        "range_step_by_zero",
        "regex_macro",
        "replace_consts",
        "should_assert_eq",
        "string_to_string",
        "unsafe_vector_initialization",
        "unstable_as_mut_slice",
        "unstable_as_slice",
        "unused_collect",
        "wrong_pub_self_convention",
        "manual_noop_waker",
        "manual_option_zip",
        "useless_borrows_in_formatting",
    ];
    pub const WORKSPACE_MANIFEST_PATH: &str = "../Cargo.toml";
    pub const STRING_LITERAL_MACRO_BOUNDARIES: &[&str] = &[
        "assert",
        "assert_eq",
        "assert_ne",
        "compile_error",
        "concat",
        "debug_assert",
        "debug_assert_eq",
        "debug_assert_ne",
        "env",
        "eprint",
        "eprintln",
        "error",
        "error_span",
        "format",
        "format_args",
        "format_ident",
        GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
        GENERATE_PG_TYPES_MACRO_NAME,
        "generate_self_upper_camel_case_and_snake_case_str_and_token_stream",
        "generate_upper_camel_case_and_snake_case_str_and_token_stream",
        GENERATE_WHERE_FILTERS_MACRO_NAME,
        "include_bytes",
        "include_str",
        "impl_to_err_string_with",
        "info",
        "info_span",
        "json",
        "join",
        "migrate",
        "option_env",
        "panic",
        "parse_quote",
        "print",
        "println",
        "query",
        "query_as",
        "query_scalar",
        "quote",
        "quote_spanned",
        "select",
        "stringify",
        "todo",
        "tp",
        "trace",
        "trace_span",
        "unimplemented",
        "unreachable",
        "view",
        "warn",
        "warn_span",
        "write",
        "writeln",
    ];
    pub const GENERATED_RUST_TOKEN_STREAM_IDENTIFIER: &str = "GeneratedRustTokenStream";
    pub const GENERATED_RUST_TOKEN_STREAM_REASON: &str = "public macro-helper API name describes generated Rust tokens and is already used across generator crates";
    pub const EXPECT_METHOD_NAME: &str = "expect";
    pub const PANIC_METHOD_NAME: &str = "panic";
}
pub mod admin_table {
    pub const USER_SORTS: [(&str, &str); 4] = [
        ("login", "Login"),
        ("display_name", "Display name"),
        ("id", "ID"),
        ("status", "Status"),
    ];
    pub const ROLE_SORTS: [(&str, &str); 3] =
        [("name", "Name"), ("id", "ID"), ("system", "System")];
    pub const PERMISSION_SORTS: [(&str, &str); 2] = [("name", "Name"), ("id", "ID")];
    pub const AUDIT_SORTS: [(&str, &str); 5] = [
        ("created_at", "Time"),
        ("user_id", "User"),
        ("action", "Action"),
        ("resource", "Resource"),
        ("succeeded", "Result"),
    ];
}
pub mod workspace_test_runner {
    pub const CARGO: &str = "cargo";
    pub const CARGO_CLIPPY_ARGS: [&str; 7] = [
        "clippy",
        "--locked",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings",
    ];
    pub const CARGO_FMT_CHECK_ARGS: [&str; 2] = ["fmt", "--check"];
    pub const CARGO_TEST_DATABASE_ARGS: [&str; 4] =
        ["test", "--locked", "--features", "test-utils"];
    pub const CARGO_TEST_DOC_ARGS: [&str; 5] =
        ["test", "--locked", "--workspace", "--doc", "--all-features"];
    pub const CARGO_TEST_GEN_PG_TBL_ARGS: [&str; 6] = [
        "test",
        "--locked",
        "-p",
        "generate_pg_table_test",
        "--features",
        "test-utils",
    ];
    pub const CARGO_TEST_GEN_PG_TYPES_ARGS: [&str; 6] = [
        "test",
        "--locked",
        "-p",
        "generate_pg_types_test",
        "--features",
        "test-utils",
    ];
    pub const CARGO_TEST_GEN_WH_FLTS_ARGS: [&str; 6] = [
        "test",
        "--locked",
        "-p",
        "generate_where_filters_test",
        "--features",
        "test-utils",
    ];
    pub const CARGO_TEST_IGNORED_ARGS: [&str; 7] = [
        "test",
        "--locked",
        "--workspace",
        "--all-features",
        "--no-fail-fast",
        "--",
        "--ignored",
    ];
    pub const CARGO_TEST_STYLE_ARGS: [&str; 5] = ["test", "--locked", "-p", "tests", "--lib"];
    pub const CARGO_TEST_WORKSPACE_ARGS: [&str; 5] = [
        "test",
        "--locked",
        "--workspace",
        "--all-features",
        "--no-fail-fast",
    ];
    pub const FORMAT_QUERY_PART_FRAGMENT: &str = "QueryPartFragment :: try_from (format !";
    pub const GENERATE_PG_TABLE_WORKLOAD: &str = "alloc-workload-generate-pg-table-src";
    pub const GENERATE_PG_TYPES_WORKLOAD: &str = "alloc-workload-generate-pg-types-src";
    pub const MAJOR_PAGE_FAULTS_PREFIX: &str = "codex_major_page_faults=";
    pub const MEMUSAGE_PATH: &str = "/usr/lib/x86_64-linux-gnu/libmemusage.so";
    pub const LIBMEMUSAGE_TOOL: &str = "libmemusage";
    pub const VALGRIND_TOOL: &str = "valgrind";
    pub const VALGRIND_PATH: &str = "/usr/bin/valgrind";
    pub const HEAPTRACK_TOOL: &str = "heaptrack";
    pub const HEAPTRACK_PATH: &str = "/usr/bin/heaptrack";
    pub const LTRACE_TOOL: &str = "ltrace";
    pub const LTRACE_PATH: &str = "/usr/bin/ltrace";
    pub const PERF_TOOL: &str = "perf";
    pub const PERF_PATH: &str = "/usr/bin/perf";
    pub const TIME_TOOL: &str = "time";
    pub const TIME_PATH: &str = "/usr/bin/time";
    pub const GENERATE_PG_TABLE_MEASUREMENT: &str = "macro_generation_generate_pg_table_test";
    pub const GENERATE_PG_TYPES_MEASUREMENT: &str = "macro_generation_generate_pg_types_test";
    pub const GENERATE_WHERE_FILTERS_MEASUREMENT: &str =
        "macro_generation_generate_where_filters_test";
    pub const MINOR_PAGE_FAULTS_PREFIX: &str = "codex_minor_page_faults=";
    pub const NEXTEST_HEAVY_ARGS: [&str; 7] = [
        "nextest",
        "run",
        "--no-fail-fast",
        "--workspace",
        "--all-features",
        "-P",
        "heavy_load",
    ];
    pub const NEXTEST_IGNORED_ARGS: [&str; 9] = [
        "nextest",
        "run",
        "--no-fail-fast",
        "--workspace",
        "--all-features",
        "-P",
        STATIC_WORKSPACE_PROFILE,
        "--run-ignored",
        "only",
    ];
    pub const NEXTEST_WORKSPACE_ARGS: [&str; 7] = [
        "nextest",
        "run",
        "--no-fail-fast",
        "--workspace",
        "--all-features",
        "-P",
        STATIC_WORKSPACE_PROFILE,
    ];
    pub const PEAK_RSS_PREFIX: &str = "codex_peak_rss_kb=";
    pub const PG_CRUD_COMMON_QUERY_PART_WORKLOAD: &str = "alloc-workload-pg-crud-common-query_part";
    pub const RESULT_ROOT: &str = "test_results/workspace_test_runner";
    pub const STATIC_WORKSPACE_PROFILE: &str = "static_workspace";
    pub const STD_FMT_WRITE_CALL: &str = "std :: fmt :: Write :: write_fmt";
    pub const STRING_WITH_CAPACITY_CALL: &str = "String :: with_capacity";
    pub const WHERE_FILTERS_QUERY_PART_WORKLOAD: &str = "alloc-workload-where-filters-query_part";
}
