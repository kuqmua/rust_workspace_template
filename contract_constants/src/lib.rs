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
    pub const GENERATED_RUST_TOKEN_STREAM_IDENTIFIER: &str = "GeneratedRustTokenStream";
    pub const GENERATED_RUST_TOKEN_STREAM_REASON: &str = "public macro-helper API name describes generated Rust tokens and is already used across generator crates";
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
