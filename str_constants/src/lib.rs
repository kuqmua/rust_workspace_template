pub(crate) const SHARED_VALUES_CHECK: &str = "check";
pub(crate) const SHARED_VALUES_ALL_TARGETS: &str = "--all-targets";
pub(crate) const SHARED_VALUES_ALL_FEATURES: &str = "--all-features";
pub(crate) const SHARED_VALUES_EMPTY: &str = "--";
pub(crate) const SHARED_VALUES_D: &str = "-D";
pub(crate) const SHARED_VALUES_WARNINGS: &str = "warnings";
pub(crate) const SHARED_VALUES_A: &str = "-A";
pub(crate) const SHARED_VALUES_CLIPPY_BOOL_ASSERT_COMPARISON: &str =
    "clippy::bool_assert_comparison";
pub(crate) const SHARED_VALUES_CLIPPY_CLONE_ON_COPY: &str = "clippy::clone_on_copy";
pub(crate) const SHARED_VALUES_CLIPPY_COLLAPSIBLE_IF: &str = "clippy::collapsible_if";
pub(crate) const SHARED_VALUES_CLIPPY_LET_AND_RETURN: &str = "clippy::let_and_return";
pub(crate) const SHARED_VALUES_CLIPPY_RESULT_LARGE_ERR: &str = "clippy::result_large_err";
pub(crate) const SHARED_VALUES_CLIPPY_SINGLE_CALL_FN: &str = "clippy::single_call_fn";
pub(crate) const SHARED_VALUES_CLIPPY_USELESS_BORROWS_IN_FORMATTING: &str =
    "clippy::useless_borrows_in_formatting";
pub(crate) const SHARED_VALUES_CLIPPY_WRITE_LITERAL: &str = "clippy::write_literal";
pub(crate) const SHARED_VALUES_FMT: &str = "fmt";
pub(crate) const SHARED_VALUES_LIB: &str = "--lib";
pub(crate) const SHARED_VALUES_DISALLOWED_FIELDS: &str = "disallowed_fields";
pub(crate) const SHARED_VALUES_UNNECESSARY_TRAILING_COMMA: &str = "unnecessary_trailing_comma";
pub(crate) const SHARED_VALUES_MANUAL_POP_IF: &str = "manual_pop_if";
pub(crate) const SHARED_VALUES_ASSIGN_OPS: &str = "assign_ops";
pub(crate) const SHARED_VALUES_EXTEND_FROM_SLICE: &str = "extend_from_slice";
pub(crate) const SHARED_VALUES_MATCH_ON_VEC_ITEMS: &str = "match_on_vec_items";
pub(crate) const SHARED_VALUES_MISALIGNED_TRANSMUTE: &str = "misaligned_transmute";
pub(crate) const SHARED_VALUES_OPTION_MAP_OR_ERR_OK: &str = "option_map_or_err_ok";
pub(crate) const SHARED_VALUES_PUB_ENUM_VARIANT_NAMES: &str = "pub_enum_variant_names";
pub(crate) const SHARED_VALUES_RANGE_STEP_BY_ZERO: &str = "range_step_by_zero";
pub(crate) const SHARED_VALUES_REGEX_MACRO: &str = "regex_macro";
pub(crate) const SHARED_VALUES_REPLACE_CONSTS: &str = "replace_consts";
pub(crate) const SHARED_VALUES_SHOULD_ASSERT_EQ: &str = "should_assert_eq";
pub(crate) const SHARED_VALUES_STRING_TO_STRING: &str = "string_to_string";
pub(crate) const SHARED_VALUES_UNSAFE_VECTOR_INITIALIZATION: &str = "unsafe_vector_initialization";
pub(crate) const SHARED_VALUES_UNSTABLE_AS_MUT_SLICE: &str = "unstable_as_mut_slice";
pub(crate) const SHARED_VALUES_UNSTABLE_AS_SLICE: &str = "unstable_as_slice";
pub(crate) const SHARED_VALUES_UNUSED_COLLECT: &str = "unused_collect";
pub(crate) const SHARED_VALUES_WRONG_PUB_SELF_CONVENTION: &str = "wrong_pub_self_convention";
pub(crate) const SHARED_VALUES_MANUAL_NOOP_WAKER: &str = "manual_noop_waker";
pub(crate) const SHARED_VALUES_MANUAL_OPTION_ZIP: &str = "manual_option_zip";
pub(crate) const SHARED_VALUES_USELESS_BORROWS_IN_FORMATTING: &str =
    "useless_borrows_in_formatting";
pub(crate) const SHARED_VALUES_ASSERT: &str = "assert";
pub(crate) const SHARED_VALUES_ASSERT_EQ: &str = "assert_eq";
pub(crate) const SHARED_VALUES_ASSERT_NE: &str = "assert_ne";
pub(crate) const SHARED_VALUES_COMPILE_ERROR: &str = "compile_error";
pub(crate) const SHARED_VALUES_CONCAT: &str = "concat";
pub(crate) const SHARED_VALUES_DEBUG_ASSERT: &str = "debug_assert";
pub(crate) const SHARED_VALUES_DEBUG_ASSERT_EQ: &str = "debug_assert_eq";
pub(crate) const SHARED_VALUES_DEBUG_ASSERT_NE: &str = "debug_assert_ne";
pub(crate) const SHARED_VALUES_DEFINE_STR_CONSTANTS: &str = "define_str_constants";
pub(crate) const SHARED_VALUES_ENV: &str = "env";
pub(crate) const SHARED_VALUES_EPRINT: &str = "eprint";
pub(crate) const SHARED_VALUES_EPRINTLN: &str = "eprintln";
pub(crate) const SHARED_VALUES_ERROR_SPAN: &str = "error_span";
pub(crate) const SHARED_VALUES_FORMAT: &str = "format";
pub(crate) const SHARED_VALUES_FORMAT_ARGS: &str = "format_args";
pub(crate) const SHARED_VALUES_FORMAT_IDENT: &str = "format_ident";
pub(crate) const SHARED_VALUES_GENERATE_SELF_UPPER_CAMEL_CASE_AND_SNAKE_CASE_STR_AND_TOKEN_STREAM: &str =
    "generate_self_upper_camel_case_and_snake_case_str_and_token_stream";
pub(crate) const SHARED_VALUES_GENERATE_UPPER_CAMEL_CASE_AND_SNAKE_CASE_STR_AND_TOKEN_STREAM: &str =
    "generate_upper_camel_case_and_snake_case_str_and_token_stream";
pub(crate) const SHARED_VALUES_IMPL_TO_ERR_STRING_WITH: &str = "impl_to_err_string_with";
pub(crate) const SHARED_VALUES_INFO_SPAN: &str = "info_span";
pub(crate) const SHARED_VALUES_JOIN: &str = "join";
pub(crate) const SHARED_VALUES_MIGRATE: &str = "migrate";
pub(crate) const SHARED_VALUES_OPTION_ENV: &str = "option_env";
pub(crate) const SHARED_VALUES_PARSE_QUOTE: &str = "parse_quote";
pub(crate) const SHARED_VALUES_PRINT: &str = "print";
pub(crate) const SHARED_VALUES_PRINTLN: &str = "println";
pub(crate) const SHARED_VALUES_QUERY: &str = "query";
pub(crate) const SHARED_VALUES_QUERY_AS: &str = "query_as";
pub(crate) const SHARED_VALUES_QUERY_SCALAR: &str = "query_scalar";
pub(crate) const SHARED_VALUES_QUOTE: &str = "quote";
pub(crate) const SHARED_VALUES_QUOTE_SPANNED: &str = "quote_spanned";
pub(crate) const SHARED_VALUES_STRINGIFY: &str = "stringify";
pub(crate) const SHARED_VALUES_TP: &str = "tp";
pub(crate) const SHARED_VALUES_TRACE_SPAN: &str = "trace_span";
pub(crate) const SHARED_VALUES_UNREACHABLE: &str = "unreachable";
pub(crate) const SHARED_VALUES_VIEW: &str = "view";
pub(crate) const SHARED_VALUES_WARN_SPAN: &str = "warn_span";
pub(crate) const SHARED_VALUES_WRITELN: &str = "writeln";
pub(crate) const SHARED_VALUES_LOGIN_2: &str = "Login";
pub(crate) const SHARED_VALUES_DISPLAY_NAME_2: &str = "Display name";
pub(crate) const SHARED_VALUES_STATUS_2: &str = "Status";
pub(crate) const SHARED_VALUES_NAME_2: &str = "Name";
pub(crate) const SHARED_VALUES_SYSTEM_2: &str = "System";
pub(crate) const SHARED_VALUES_TIME: &str = "Time";
pub(crate) const SHARED_VALUES_USER: &str = "User";
pub(crate) const SHARED_VALUES_ACTION_2: &str = "Action";
pub(crate) const SHARED_VALUES_RESOURCE_2: &str = "Resource";
pub(crate) const SHARED_VALUES_LOCKED: &str = "--locked";
pub(crate) const SHARED_VALUES_CHECK_2: &str = "--check";
pub(crate) const SHARED_VALUES_FEATURES: &str = "--features";
pub(crate) const SHARED_VALUES_WORKSPACE: &str = "--workspace";
pub(crate) const SHARED_VALUES_DOC: &str = "--doc";
pub(crate) const SHARED_VALUES_GENERATE_PG_TABLE_TEST: &str = "generate_pg_table_test";
pub(crate) const SHARED_VALUES_GENERATE_PG_TYPES_TEST: &str = "generate_pg_types_test";
pub(crate) const SHARED_VALUES_GENERATE_WHERE_FILTERS_TEST: &str = "generate_where_filters_test";
pub(crate) const SHARED_VALUES_NO_FAIL_FAST: &str = "--no-fail-fast";
pub(crate) const SHARED_VALUES_IGNORED: &str = "--ignored";
pub(crate) const SHARED_VALUES_RUN: &str = "run";
pub(crate) const SHARED_VALUES_P_2: &str = "-P";
pub(crate) const SHARED_VALUES_HEAVY_LOAD: &str = "heavy_load";
pub(crate) const SHARED_VALUES_RUN_IGNORED: &str = "--run-ignored";
pub(crate) const SHARED_VALUES_ONLY: &str = "only";
pub const ENV_NAMES_CORS_ALLOW_ORIGIN: &str = "CORS_ALLOW_ORIGIN";
pub const ENV_NAMES_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_NAMES_ENABLE_API_GIT_COMMIT_CHECK: &str = "ENABLE_API_GIT_COMMIT_CHECK";
pub const ENV_NAMES_MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES: &str = "MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES";
pub const ENV_NAMES_PG_POOL_MAX_CONNECTIONS: &str = "PG_POOL_MAX_CONNECTIONS";
pub const ENV_NAMES_SERVICE_SOCKET_ADDRESS: &str = "SERVICE_SOCKET_ADDRESS";
pub const ENV_NAMES_SRC_PLACE_TYPE: &str = "SRC_PLACE_TYPE";
pub const ENV_NAMES_TIMEZONE: &str = "TIMEZONE";
pub const ENV_NAMES_TRACING_LEVEL: &str = "TRACING_LEVEL";
pub const HTTP_HEADER_NAMES_X_API_GIT_COMMIT: &str = "x-api-git-commit";
pub const HTTP_HEADER_NAMES_X_REQUEST_ID: &str = "x-request-id";
pub const ROUTE_PATHS_NOT_FOUND: &str = "/404";
pub const SQL_NAMES_ID: &str = "id";
pub const COMMON_ROUTES_GIT_INFO: &str = "/git_info";
str_constants_macros::define_str_constants! {
    fragments {
        HEALTH = "/health";
    }
    constants {
        COMMON_ROUTES_HEALTH = [HEALTH];
        COMMON_ROUTES_HEALTH_CHECK = [HEALTH, "_check"];
        COMMON_ROUTES_HEALTH_LIVE = [HEALTH, "/live"];
        COMMON_ROUTES_HEALTH_READY = [HEALTH, "/ready"];
    }
}
pub const COMMON_ROUTES_HEALTH_CHECK_SQL: &str = "SELECT 1";
pub const COMMON_ROUTES_NO_ROUTE_MSG_PREFIX: &str = "No route for ";
pub const COMMON_ROUTES_SWAGGER_UI: &str = "/swagger-ui";
pub const CONFIG_ENV_VALUE_IS_EMPTY_MSG: &str = "is empty";
pub const CONFIG_SRC_PLACE_TYPE_FIX_MSG: &str =
    "You can set environment variable SRC_PLACE_TYPE to be eq \"src\" or \"github\"";
pub const CONFIG_SRC_PLACE_TYPE_PARSE_CTX: &str =
    "<SrcPlaceType as std::str::FromStr>::from_str(&v)";
pub const CONFIG_TIMEZONE_NOT_EAST_MSG: &str = "not east";
pub const CONFIG_TRACING_DEBUG: &str = "debug";
pub const CONFIG_TRACING_ERROR: &str = "error";
pub const CONFIG_TRACING_INFO: &str = "info";
pub const CONFIG_TRACING_TRACE: &str = "trace";
pub const CONFIG_TRACING_WARN: &str = "warn";
pub const GIT_INFO_TREE_SEGMENT: &str = "/tree/";

pub const GIT_INFO_PROJECT_GIT_COMMIT_ID: &str =
    git_version::git_version!(args = ["--always", "--abbrev=40"]);
pub const GIT_INFO_PROJECT_GIT_COMMIT_LINK: &str = git_version::git_version!(
    args = ["--always", "--abbrev=40"],
    prefix = "https://github.com/kuqmua/rust_workspace_template/tree/"
);
pub const LOCATION_INCORRECT_DATETIME_MSG: &str = "incorrect datetime";
pub const MACRO_DIAGNOSTICS_AS_REF_INNER_SHARED_REF_ERROR: &str =
    "#[newtype(as_ref_inner)] requires a shared reference inner type";
pub const MACRO_DIAGNOSTICS_BOUNDED_STRING_MAX_ERROR: &str =
    "BoundedString requires #[bounded_string(max = ...)]";
pub const MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_CLOSURE_ERROR: &str =
    "case_trait_pair expects closure";
pub const MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_PARTS_ERROR: &str =
    "case_trait_pair expects str trait, ts trait, bound, closure expr";
pub const MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_STR_TRAIT_ERROR: &str =
    "case_trait_pair expects string trait name";
pub const MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_TS_TRAIT_ERROR: &str =
    "case_trait_pair expects token trait name";
pub const MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_EXPECTED_BOUND_ERROR: &str =
    "case_trait_pair expects bound";
pub const MACRO_DIAGNOSTICS_CASE_TRAIT_PAIR_PARSE_BODY_ERROR: &str =
    "case_trait_pair failed to parse body";
pub const MACRO_DIAGNOSTICS_DUPLICATE_BOUNDED_STRING_OPTION_ERROR: &str =
    "duplicate bounded_string option";
pub const MACRO_DIAGNOSTICS_EXPECTED_ANGLE_BRACKETED_ARGS_ERROR: &str =
    "07c6ab44: expected angle bracketed args";
pub const MACRO_DIAGNOSTICS_EXPECTED_FIRST_PATH_SEGMENT_ERROR: &str =
    "595050cf: expected first path segment";
pub const MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C1_ERROR: &str = "c1d03b71: expected HashMap<K, T>";
pub const MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_C8_ERROR: &str = "c828da34: expected HashMap<K, T>";
pub const MACRO_DIAGNOSTICS_EXPECTED_HASH_MAP_E9_ERROR: &str = "e9c6a7d2: expected HashMap<K, T>";
pub const MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_A2_ERROR: &str =
    "a21dc807: expected named field identifier";
pub const MACRO_DIAGNOSTICS_EXPECTED_NAMED_FIELD_ERROR: &str =
    "438aa90e: expected named field identifier";
pub const MACRO_DIAGNOSTICS_EXPECTED_NAMED_VARIANT_FIELDS_ERROR: &str =
    "79b0f231: expected named variant fields";
pub const MACRO_DIAGNOSTICS_PRIMARY_KEY_FIELD_INDEX_ERROR: &str =
    "878d3f9b: primary key field index not found";
pub const MACRO_DIAGNOSTICS_TUPLE_STRUCT_ERROR: &str = "Newtype supports only tuple structs";
pub const MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 3] = [
    SHARED_VALUES_CHECK,
    SHARED_VALUES_ALL_TARGETS,
    SHARED_VALUES_ALL_FEATURES,
];
pub const MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 22] = [
    CLIPPY,
    SHARED_VALUES_ALL_TARGETS,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_EMPTY,
    SHARED_VALUES_D,
    SHARED_VALUES_WARNINGS,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_BOOL_ASSERT_COMPARISON,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_CLONE_ON_COPY,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_COLLAPSIBLE_IF,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_LET_AND_RETURN,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_RESULT_LARGE_ERR,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_SINGLE_CALL_FN,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_USELESS_BORROWS_IN_FORMATTING,
    SHARED_VALUES_A,
    SHARED_VALUES_CLIPPY_WRITE_LITERAL,
];
pub const MACRO_CLIPPY_CARGO_FMT_ARGS: [&str; 1] = [SHARED_VALUES_FMT];
pub const MACRO_CLIPPY_CARGO_TEST_LIB_ARGS: [&str; 2] = [TEST_ALT_3, SHARED_VALUES_LIB];
pub const COMPILE_ERROR_ERROR_PLACEHOLDER: &str = "{error}";
pub const COMPILE_ERROR_CE_000: &str = "10764d2b: expected named variant fields";
pub const COMPILE_ERROR_CE_001: &str = "10773d36: expected named variant fields";
pub const COMPILE_ERROR_CE_002: &str =
    "1266ae5a: field identifier is longer than PostgreSQL column name limit";
pub const COMPILE_ERROR_CE_003: &str = "1a75cea1: duplicate primary key field";
pub const COMPILE_ERROR_CE_004: &str = "1be4a6e2: expected named variant fields";
pub const COMPILE_ERROR_CE_005: &str = "201dc0a4: {error}";
pub const COMPILE_ERROR_CE_006: &str = "22bc6672: non-primary-key field index not found";
pub const COMPILE_ERROR_CE_007: &str = "22c364b9: {error}";
pub const COMPILE_ERROR_CE_008: &str = "2acd4725: expected named variant fields";
pub const COMPILE_ERROR_CE_009: &str = "2ad2130d: primary key type must be a path";
pub const COMPILE_ERROR_CE_010: &str = "2db209a8: {error}";
pub const COMPILE_ERROR_CE_011: &str = "35d30bd7: frontend field order values must be unique";
pub const COMPILE_ERROR_CE_012: &str =
    "45dff0e2: optimistic_revision_field must be a non-primary-key signed 64-bit field";
pub const COMPILE_ERROR_CE_013: &str = "536203b7: bulk item limit must be greater than zero";
pub const COMPILE_ERROR_CE_015: &str = "6a529a99: primary key field not found";
pub const COMPILE_ERROR_CE_016: &str = "6d0adac1: cloned primary key type path has no segments";
pub const COMPILE_ERROR_CE_017: &str =
    "741aa5f9: create_exclude_fields must contain unique non-primary-key field names";
pub const COMPILE_ERROR_CE_018: &str = "7f31872d: expected named struct fields";
pub const COMPILE_ERROR_CE_019: &str = "81efa954: status code attr not found";
pub const COMPILE_ERROR_CE_020: &str = "86307dbc: {error}";
pub const COMPILE_ERROR_CE_021: &str =
    "8a5fbef9: frontend field configuration count does not match fields";
pub const COMPILE_ERROR_CE_022: &str =
    "8a66c852: error variant attr identifier does not match attr name";
pub const COMPILE_ERROR_CE_023: &str = "8af68998: location field attr not found";
pub const COMPILE_ERROR_CE_024: &str = "8d93bf20: expected path type";
pub const COMPILE_ERROR_CE_025: &str = "8dcafc1c: expected named variant fields";
pub const COMPILE_ERROR_CE_026: &str = "915ef2ce: expected named field identifier";
pub const COMPILE_ERROR_CE_027: &str =
    "91a3d9f2: read_exclude_fields must contain unique non-primary-key field names";
pub const COMPILE_ERROR_CE_028: &str = "9a469d36: duplicate location field attr";
pub const COMPILE_ERROR_CE_029: &str = "9a4d65c9: duplicate location field attr";
pub const COMPILE_ERROR_CE_030: &str = "ae8e173b: expected named variant field identifier";
pub const COMPILE_ERROR_CE_031: &str = "assert_empty_parse_err_matches expects pattern";
pub const COMPILE_ERROR_CE_032: &str = "assert_empty_parse_err_matches expects type";
pub const COMPILE_ERROR_CE_033: &str = "assert_empty_parse_err_matches expects type, pattern";
pub const COMPILE_ERROR_CE_034: &str = "assert_parse_err_matches expects pattern";
pub const COMPILE_ERROR_CE_035: &str = "assert_parse_err_matches expects type";
pub const COMPILE_ERROR_CE_036: &str = "assert_parse_err_matches expects type, value, pattern";
pub const COMPILE_ERROR_CE_037: &str = "assert_parse_err_matches expects value";
pub const COMPILE_ERROR_CE_038: &str = "assert_parse_ok_matches expects pattern";
pub const COMPILE_ERROR_CE_039: &str = "assert_parse_ok_matches expects type";
pub const COMPILE_ERROR_CE_040: &str = "assert_parse_ok_matches expects type, value, pattern";
pub const COMPILE_ERROR_CE_041: &str = "assert_parse_ok_matches expects value";
pub const COMPILE_ERROR_CE_042: &str = "b9f53bee: location field attr not found";
pub const COMPILE_ERROR_CE_043: &str = "bd4718d0: expected struct input";
pub const COMPILE_ERROR_CE_044: &str = "bool_enum_to_tokens expects comma after enum name";
pub const COMPILE_ERROR_CE_045: &str = "bool_enum_to_tokens expects enum name";
pub const COMPILE_ERROR_CE_046: &str = "bool_enum_to_tokens expects false => expr";
pub const COMPILE_ERROR_CE_047: &str = "bool_enum_to_tokens expects true => expr";
pub const COMPILE_ERROR_CE_048: &str = "bool_enum_to_tokens failed to parse false expr";
pub const COMPILE_ERROR_CE_049: &str = "bool_enum_to_tokens failed to parse true expr";
pub const COMPILE_ERROR_CE_050: &str = "d1003b2e: location field attr not found";
pub const COMPILE_ERROR_CE_051: &str =
    "d5f1b3a7: permission prefix must use lowercase ASCII letters, digits, or underscores";
pub const COMPILE_ERROR_CE_052: &str = "e7408836: primary key type path has no segments";
pub const COMPILE_ERROR_CE_053: &str = "e9b33787: expected first generic arg";
pub const COMPILE_ERROR_CE_054: &str = "edbbd08a: expected named field identifier";
pub const COMPILE_ERROR_CE_055: &str =
    "f7ea4b19: optimistic_revision_field must name an existing field";
pub const COMPILE_ERROR_CE_056: &str = "impl_cfg_getter expects fn name";
pub const COMPILE_ERROR_CE_057: &str = "impl_cfg_getter expects return type";
pub const COMPILE_ERROR_CE_058: &str = "impl_cfg_getter expects trait name";
pub const COMPILE_ERROR_CE_059: &str = "impl_cfg_getter expects trait, fn, ret_ty";
pub const COMPILE_ERROR_CE_060: &str = "impl_to_err_string_const expects type => message";
pub const COMPILE_ERROR_CE_061: &str = "impl_to_err_string_with expects closure";
pub const COMPILE_ERROR_CE_062: &str = "impl_to_err_string_with expects types => |value| body";
pub const COMPILE_ERROR_CE_063: &str = "impl_try_from_non_empty_string expects error name";
pub const COMPILE_ERROR_CE_064: &str = "impl_try_from_non_empty_string expects name";
pub const COMPILE_ERROR_CE_065: &str = "impl_try_from_non_empty_string expects name, error name";
pub const COMPILE_ERROR_CE_066: &str = "impl_try_from_parse expects error field";
pub const COMPILE_ERROR_CE_067: &str = "impl_try_from_parse expects error name";
pub const COMPILE_ERROR_CE_068: &str = "impl_try_from_parse expects error variant";
pub const COMPILE_ERROR_CE_069: &str = "impl_try_from_parse expects inner type";
pub const COMPILE_ERROR_CE_070: &str = "impl_try_from_parse expects name";
pub const COMPILE_ERROR_CE_071: &str =
    "impl_try_from_parse expects name, error name, inner type and error variant";
pub const COMPILE_ERROR_CE_072: &str = "impl_try_from_secret_url expects error name";
pub const COMPILE_ERROR_CE_073: &str = "impl_try_from_secret_url expects name";
pub const COMPILE_ERROR_CE_074: &str = "impl_try_from_secret_url expects name, error name";
pub const COMPILE_ERROR_CE_075: &str = "tp expects comma after type name";
pub const COMPILE_ERROR_CE_076: &str = "tp expects type name";
pub const COMPILE_ERROR_CE_077: &str = "tp_parts expects type name";
pub const COMPILE_ERROR_CE_078: &str = "tp_parts expects type name and at least one part";
pub const COMPILE_ERROR_CE_079: &str = "trait_alias expects Name = Bounds";
pub const COMPILE_ERROR_CE_080: &str = "trait_alias failed to parse bounds";
pub const COMPILE_ERROR_CE_081: &str = "ts_path_fn expects comma after function name";
pub const COMPILE_ERROR_CE_082: &str = "ts_path_fn expects function name";
pub const NAMING_GITHUB_URL: &str = "https://github.com/kuqmua/rust_workspace_template";
pub const NAMING_REGEX_VALUE: &str = "^[a-zA-Z0-9]+$";
pub const PANIC_LOCATION_NO_LOCATION_MSG: &str =
    "panic occurred but can\'t get location information...";
pub const PG_CRUD_ADJACENT_SQL_OPERATOR: &str = "-|-";
pub const PG_CRUD_BEFORE_SQL_OPERATOR: &str = "<";
pub const PG_CRUD_CONTAINS_SQL_OPERATOR: &str = "@>";
pub const PG_CRUD_EMPTY_SQL_SUFFIX: &str = "";
pub const PG_CRUD_EQUALITY_SQL_OPERATOR: &str = "=";
pub const PG_CRUD_LEFT_OF_SQL_OPERATOR: &str = "&<";
pub const PG_CRUD_OVERLAPS_SQL_OPERATOR: &str = "&&";
pub const PG_CRUD_RIGHT_OF_SQL_OPERATOR: &str = "&>";
pub const PG_CRUD_TEXT_SEARCH_SQL_OPERATOR: &str = "ILIKE";
pub const PG_CRUD_TEXT_SEARCH_SQL_SUFFIX: &str = "ESCAPE \'\\\'";
pub const PG_CRUD_WITHIN_SQL_OPERATOR: &str = "<@";
pub const PG_CRUD_CREATE_PERMISSION_ACTION: &str = "create";
pub const PG_CRUD_DELETE_PERMISSION_ACTION: &str = "delete";
pub const PG_CRUD_READ_PERMISSION_ACTION: &str = "read";
pub const PG_CRUD_UPDATE_PERMISSION_ACTION: &str = "update";
pub const PG_CRUD_PG_BIGSERIAL: &str = "bigserial";
pub const PG_CRUD_PG_BOOL: &str = "bool";
pub const PG_CRUD_PG_BYTEA: &str = "bytea";
pub const PG_CRUD_PG_DATE: &str = "date";
pub const PG_CRUD_PG_DATERANGE: &str = "daterange";
pub const PG_CRUD_PG_FLOAT4: &str = "float4";
pub const PG_CRUD_PG_FLOAT8: &str = "float8";
pub const PG_CRUD_PG_INET: &str = "inet";
pub const PG_CRUD_PG_INT2: &str = "int2";
pub const PG_CRUD_PG_INT4: &str = "int4";
pub const PG_CRUD_PG_INT4RANGE: &str = "int4range";
pub const PG_CRUD_PG_INT8: &str = "int8";
pub const PG_CRUD_PG_INT8RANGE: &str = "int8range";
pub const PG_CRUD_PG_INTERVAL: &str = "interval";
pub const PG_CRUD_PG_MACADDR: &str = "macaddr";
pub const PG_CRUD_PG_MONEY: &str = "money";
pub const PG_CRUD_PG_SERIAL: &str = "serial";
pub const PG_CRUD_PG_SMALLSERIAL: &str = "smallserial";
pub const PG_CRUD_PG_TEXT: &str = "text";
pub const PG_CRUD_PG_TIME: &str = "time";
pub const PG_CRUD_PG_TIMESTAMP: &str = "timestamp";
pub const PG_CRUD_PG_TIMESTAMPTZ: &str = "timestamptz";
pub const PG_CRUD_PG_TSRANGE: &str = "tsrange";
pub const PG_CRUD_PG_TSTZRANGE: &str = "tstzrange";
pub const PG_CRUD_PG_UUID: &str = "uuid";
pub const PG_CRUD_BETWEEN_EXPECTING: &str = "struct Between with 2 els";
pub const PG_CRUD_BETWEEN_SCHEMA_NAME: &str = "Between";
pub const PG_CRUD_BETWEEN_STRUCT_NAME: &str = "struct Between";
pub const PG_CRUD_END_FIELD: &str = "end";
pub const PG_CRUD_FIELD_IDENTIFIER: &str = "field identifier";
pub const PG_CRUD_COMPLETE_IDEMPOTENCY_SQL: &str = "UPDATE pg_table_idempotency SET state=\'completed\',response_status=$6,response_body=$7,completed_at=NOW() WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4 AND request_hash=$5 AND state=\'pending\'";
pub const PG_CRUD_GENERATE_PG_TABLE_CONFIG_PATH: &str =
    "generate_pg_table::generate_pg_table_config";
pub const PG_CRUD_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME: &str = "NotEmptyUniqueVec";
pub const PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING: &str =
    "tuple struct NotEmptyUniqueVec with 1 element";
pub const PG_CRUD_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME: &str = "tuple struct NotEmptyUniqueVec";
pub const PG_CRUD_OPERATOR_FIELD: &str = "operator";
pub const PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_SCHEMA_NAME: &str = "PgTypeNotEmptyUniqueVec";
pub const PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_EXPECTING: &str =
    "tuple struct PgTypeNotEmptyUniqueVec with 1 element";
pub const PG_CRUD_PG_TYPE_NOT_EMPTY_UNIQUE_VEC_TUPLE_NAME: &str =
    "tuple struct PgTypeNotEmptyUniqueVec";
pub const PG_CRUD_PG_TYPE_WHERE_SCHEMA_NAME: &str = "PgTypeWhere";
pub const PG_CRUD_PG_TYPE_WHERE_STRUCT_NAME: &str = "struct PgTypeWhere";
pub const PG_CRUD_PG_TYPE_WHERE_EXPECTING: &str = "struct PgTypeWhere with 2 els";
pub const PG_CRUD_REGEX_REGEX_SCHEMA_ID: &str = "tests::RegexRegex";
pub const PG_CRUD_REGEX_REGEX_SCHEMA_NAME: &str = "RegexRegex";
pub const PG_CRUD_START_FIELD: &str = "start";
pub const PG_CRUD_V_FIELD: &str = "v";

pub const PG_CRUD_SERDE_BETWEEN_FIELDS: &[&str] = &[PG_CRUD_START_FIELD, PG_CRUD_END_FIELD];
pub const PG_CRUD_SERDE_PG_TYPE_WHERE_FIELDS: &[&str] = &[PG_CRUD_OPERATOR_FIELD, PG_CRUD_V_FIELD];
pub const ROUTE_VALIDATORS_BLOCK_ON_POLL_LIMIT_ER_ID: &str = "cf6e91ab";
pub const ROUTE_VALIDATORS_COMMIT_NOT_EQ_MSG: &str =
    "different project commit provided, services must work only with eq project commits";
pub const ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG: &str = "no_commit_header";
pub const ROUTE_VALIDATORS_EXPECT_ER_ER_ID: &str = "2f755472";
pub const ROUTE_VALIDATORS_EXPECT_OK_ER_ID: &str = "db9d2f63";
pub const ROUTE_VALIDATORS_REPLACE_HEADER_MISSING_SRC_ER_ID: &str = "c3a0f7be";
pub const ROUTE_VALIDATORS_COMMIT_HEADER_NAME: &str = "commit";
pub const ROUTE_VALIDATORS_TEST_HEADER_NAME: &str = "x-test-header";
pub const RUNTIME_CORRELATION_ID_HEADER_NAME: &str = "x-correlation-id";
pub const RUNTIME_FORWARDED_FOR_HEADER_NAME: &str = "x-forwarded-for";
pub const RUNTIME_REAL_IP_HEADER_NAME: &str = "x-real-ip";
pub const SERVER_ADMIN_ACCESS_COOKIE_NAME: &str = "admin_access_token";
pub const SERVER_ADMIN_ACTIVE_ADMIN_COUNT_SQL: &str = "SELECT COUNT(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id JOIN admin_roles role ON role.id = user_role.role_id WHERE role.name = \'admin\' AND users.is_banned = FALSE";
pub const SERVER_ADMIN_API_PREFIX: &str = "/api/v1/admin";
pub const SERVER_ADMIN_INSERT_USER_SQL: &str =
    "INSERT INTO admin_users (login, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id";
pub const SERVER_ADMIN_LOCK_LAST_ADMIN_SQL: &str =
    "SELECT pg_advisory_xact_lock(hashtext(\'admin_last_active_administrator\'))";
pub const SERVER_ADMIN_REVOKE_ACCESS_SESSION_SQL: &str = "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL";
pub const SERVER_ADMIN_REVOKE_USER_ACCESS_SESSIONS_SQL: &str =
    "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL";
pub const SERVER_ADMIN_REVOKE_USER_REFRESH_TOKENS_SQL: &str =
    "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL";
pub const SERVER_ADMIN_USER_IS_ADMIN_SQL: &str = "SELECT EXISTS (SELECT 1 FROM admin_user_roles user_role JOIN admin_roles role ON role.id = user_role.role_id WHERE user_role.user_id = $1 AND role.name = \'admin\')";
pub const SERVER_ADMIN_RATE_LIMIT_AUDIT_READ: &str = "audit_read";
pub const SERVER_ADMIN_RATE_LIMIT_MUTATION: &str = "mutation";
pub const SERVER_ADMIN_RATE_LIMIT_REFRESH_IP: &str = "refresh_ip";
pub const SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP: &str = "sign_in_ip";
pub const SERVER_ADMIN_RATE_LIMIT_SIGN_IN_IP_LOGIN: &str = "sign_in_ip_login";
str_constants_macros::define_str_constants! {
    fragments {
        AUTH = "/auth";
        ROLES = "/roles";
        USERS = "/users";
    }
    constants {
        ADMIN_API_PATHS_AUDIT = ["/audit-log"];
        ADMIN_API_PATHS_AUTH_ME = [AUTH, "/me"];
        ADMIN_API_PATHS_AUTH_REFRESH = [AUTH, "/refresh"];
        ADMIN_API_PATHS_AUTH_SESSION = [AUTH, "/sessions/{session_id}"];
        ADMIN_API_PATHS_AUTH_SESSIONS = [AUTH, "/sessions"];
        ADMIN_API_PATHS_AUTH_SIGN_IN = [AUTH, "/sign-in"];
        ADMIN_API_PATHS_AUTH_SIGN_OUT = [AUTH, "/sign-out"];
        ADMIN_API_PATHS_PERMISSIONS = ["/permissions"];
        ADMIN_API_PATHS_ROLE = [ROLES, "/{role_id}"];
        ADMIN_API_PATHS_ROLE_PERMISSIONS = [ROLES, "/{role_id}/permissions"];
        ADMIN_API_PATHS_ROLES = [ROLES];
        ADMIN_API_PATHS_SETTINGS = ["/system-settings"];
        ADMIN_API_PATHS_USER = [USERS, "/{user_id}"];
        ADMIN_API_PATHS_USER_BAN = [USERS, "/{user_id}/ban"];
        ADMIN_API_PATHS_USER_PASSWORD = [USERS, "/{user_id}/password"];
        ADMIN_API_PATHS_USER_ROLES = [USERS, "/{user_id}/roles"];
        ADMIN_API_PATHS_USERS = [USERS];
    }
}
str_constants_macros::define_str_constants! {
    fragments {
        ADMIN = "/admin";
    }
    constants {
        ADMIN_PAGE_PATHS_ASSETS = [ADMIN, "/assets"];
        ADMIN_PAGE_PATHS_AUDIT = [ADMIN, "/audit-log"];
        ADMIN_PAGE_PATHS_METRICS = [ADMIN, "/metrics"];
        ADMIN_PAGE_PATHS_OPEN_API = [ADMIN, "/swagger-ui"];
        ADMIN_PAGE_PATHS_OPEN_API_DOCUMENT = [ADMIN, "/openapi.json"];
        ADMIN_PAGE_PATHS_PERMISSIONS = [ADMIN, "/permissions"];
        ADMIN_PAGE_PATHS_ROLES = [ADMIN, "/roles"];
        ADMIN_PAGE_PATHS_ROOT = [ADMIN];
        ADMIN_PAGE_PATHS_SETTINGS = [ADMIN, "/system-settings"];
        ADMIN_PAGE_PATHS_SIGN_IN = [ADMIN, "/sign-in"];
        ADMIN_PAGE_PATHS_USERS = [ADMIN, "/users"];
        ADMIN_PAGE_PATHS_VERSION = [ADMIN, "/version"];
    }
}

pub const ADMIN_PAGE_PATHS_ALL: [&str; 10] = [
    ROOT,
    SIGN_IN,
    USERS,
    ROLES,
    PERMISSIONS,
    ADMIN_PAGE_PATHS_AUDIT,
    SETTINGS,
    METRICS,
    VERSION,
    ADMIN_PAGE_PATHS_OPEN_API,
];
str_constants_macros::define_str_constants! {
    fragments {
        AUDIT_LOG = "audit_log";
        CREATE = ":create";
        DELETE = ":delete";
        METRICS = "metrics";
        OPEN_API = "openapi";
        PERMISSIONS = "permissions";
        READ = ":read";
        ROLES = "roles";
        ROLE_PERMISSIONS = "role_permissions";
        SYSTEM_SETTINGS = "system_settings";
        UPDATE = ":update";
        USERS = "users";
        USER_ROLES = "user_roles";
    }
    constants {
        ADMIN_PERMISSION_VALUES_AUDIT_LOG_READ = [AUDIT_LOG, READ];
        ADMIN_PERMISSION_VALUES_METRICS_READ = [METRICS, READ];
        ADMIN_PERMISSION_VALUES_OPEN_API_READ = [OPEN_API, READ];
        ADMIN_PERMISSION_VALUES_PERMISSIONS_READ = [PERMISSIONS, READ];
        ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_CREATE = [ROLE_PERMISSIONS, CREATE];
        ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_DELETE = [ROLE_PERMISSIONS, DELETE];
        ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_READ = [ROLE_PERMISSIONS, READ];
        ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_UPDATE = [ROLE_PERMISSIONS, UPDATE];
        ADMIN_PERMISSION_VALUES_ROLES_CREATE = [ROLES, CREATE];
        ADMIN_PERMISSION_VALUES_ROLES_DELETE = [ROLES, DELETE];
        ADMIN_PERMISSION_VALUES_ROLES_READ = [ROLES, READ];
        ADMIN_PERMISSION_VALUES_ROLES_UPDATE = [ROLES, UPDATE];
        ADMIN_PERMISSION_VALUES_SYSTEM_SETTINGS_READ = [SYSTEM_SETTINGS, READ];
        ADMIN_PERMISSION_VALUES_SYSTEM_SETTINGS_UPDATE = [SYSTEM_SETTINGS, UPDATE];
        ADMIN_PERMISSION_VALUES_USER_ROLES_CREATE = [USER_ROLES, CREATE];
        ADMIN_PERMISSION_VALUES_USER_ROLES_DELETE = [USER_ROLES, DELETE];
        ADMIN_PERMISSION_VALUES_USER_ROLES_READ = [USER_ROLES, READ];
        ADMIN_PERMISSION_VALUES_USER_ROLES_UPDATE = [USER_ROLES, UPDATE];
        ADMIN_PERMISSION_VALUES_USERS_CREATE = [USERS, CREATE];
        ADMIN_PERMISSION_VALUES_USERS_DELETE = [USERS, DELETE];
        ADMIN_PERMISSION_VALUES_USERS_READ = [USERS, READ];
        ADMIN_PERMISSION_VALUES_USERS_UPDATE = [USERS, UPDATE];
    }
}
pub const TEST_VALUES_COMMIT: &str = "abc123";
pub const TEST_VALUES_OPEN_API_TABLE_EXAMPLE_PATH_PREFIX: &str = "/paths/~1table_example~1";
pub const TEST_VALUES_UNREACHABLE_DATABASE_URL: &str = "postgres://usr:pwd@127.0.0.1:1/unreachable";
pub const TEST_VALUES_WRONG_COMMIT: &str = "deadbeef";
pub const CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME: &str =
    "generate_derive_token_stream_builder";
pub const CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME: &str = "generate_pg_types";
pub const CODE_STYLE_GENERATE_WHERE_FILTERS_MACRO_NAME: &str = "generate_where_filters";
pub const CODE_STYLE_STRING_GUARD_ALLOWED_SYNTAX_FIXTURE: &str =
    "#[path = \"fixture.rs\"] mod fixture; fn f() { value.expect(\"12345678\"); }";
pub const CODE_STYLE_STRING_GUARD_DETECTION_FIXTURE: &str =
    "fn f() { consume(\"ordinary\"); outer!(inner(\"macro\")); }";
pub const CODE_STYLE_CI_WORKFLOW_PATH: &str = ".github/workflows/ci.yml";
pub const CODE_STYLE_WORKSPACE_MANIFEST_PATH: &str = "../Cargo.toml";
pub const CODE_STYLE_GENERATED_RUST_TOKEN_STREAM_IDENTIFIER: &str = "GeneratedRustTokenStream";
pub const CODE_STYLE_GENERATED_RUST_TOKEN_STREAM_REASON: &str = "public macro-helper API name describes generated Rust tokens and is already used across generator crates";
pub const CODE_STYLE_EXPECT_METHOD_NAME: &str = "expect";
pub const CODE_STYLE_PANIC_METHOD_NAME: &str = "panic";

pub const CODE_STYLE_CLIPPY_LINT_EXCEPTIONS: [&str; 22] = [
    SHARED_VALUES_DISALLOWED_FIELDS,
    SHARED_VALUES_UNNECESSARY_TRAILING_COMMA,
    SHARED_VALUES_MANUAL_POP_IF,
    SHARED_VALUES_ASSIGN_OPS,
    SHARED_VALUES_EXTEND_FROM_SLICE,
    SHARED_VALUES_MATCH_ON_VEC_ITEMS,
    SHARED_VALUES_MISALIGNED_TRANSMUTE,
    SHARED_VALUES_OPTION_MAP_OR_ERR_OK,
    SHARED_VALUES_PUB_ENUM_VARIANT_NAMES,
    SHARED_VALUES_RANGE_STEP_BY_ZERO,
    SHARED_VALUES_REGEX_MACRO,
    SHARED_VALUES_REPLACE_CONSTS,
    SHARED_VALUES_SHOULD_ASSERT_EQ,
    SHARED_VALUES_STRING_TO_STRING,
    SHARED_VALUES_UNSAFE_VECTOR_INITIALIZATION,
    SHARED_VALUES_UNSTABLE_AS_MUT_SLICE,
    SHARED_VALUES_UNSTABLE_AS_SLICE,
    SHARED_VALUES_UNUSED_COLLECT,
    SHARED_VALUES_WRONG_PUB_SELF_CONVENTION,
    SHARED_VALUES_MANUAL_NOOP_WAKER,
    SHARED_VALUES_MANUAL_OPTION_ZIP,
    SHARED_VALUES_USELESS_BORROWS_IN_FORMATTING,
];
pub const CODE_STYLE_STRING_LITERAL_MACRO_BOUNDARIES: &[&str] = &[
    SHARED_VALUES_ASSERT,
    SHARED_VALUES_ASSERT_EQ,
    SHARED_VALUES_ASSERT_NE,
    SHARED_VALUES_COMPILE_ERROR,
    SHARED_VALUES_CONCAT,
    SHARED_VALUES_DEBUG_ASSERT,
    SHARED_VALUES_DEBUG_ASSERT_EQ,
    SHARED_VALUES_DEBUG_ASSERT_NE,
    SHARED_VALUES_DEFINE_STR_CONSTANTS,
    SHARED_VALUES_ENV,
    SHARED_VALUES_EPRINT,
    SHARED_VALUES_EPRINTLN,
    CONFIG_TRACING_ERROR,
    SHARED_VALUES_ERROR_SPAN,
    SHARED_VALUES_FORMAT,
    SHARED_VALUES_FORMAT_ARGS,
    SHARED_VALUES_FORMAT_IDENT,
    CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
    CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
    SHARED_VALUES_GENERATE_SELF_UPPER_CAMEL_CASE_AND_SNAKE_CASE_STR_AND_TOKEN_STREAM,
    SHARED_VALUES_GENERATE_UPPER_CAMEL_CASE_AND_SNAKE_CASE_STR_AND_TOKEN_STREAM,
    CODE_STYLE_GENERATE_WHERE_FILTERS_MACRO_NAME,
    INCLUDE_BYTES,
    INCLUDE_STR,
    SHARED_VALUES_IMPL_TO_ERR_STRING_WITH,
    CONFIG_TRACING_INFO,
    SHARED_VALUES_INFO_SPAN,
    JSON,
    SHARED_VALUES_JOIN,
    SHARED_VALUES_MIGRATE,
    SHARED_VALUES_OPTION_ENV,
    CODE_STYLE_PANIC_METHOD_NAME,
    SHARED_VALUES_PARSE_QUOTE,
    SHARED_VALUES_PRINT,
    SHARED_VALUES_PRINTLN,
    SHARED_VALUES_QUERY,
    SHARED_VALUES_QUERY_AS,
    SHARED_VALUES_QUERY_SCALAR,
    SHARED_VALUES_QUOTE,
    SHARED_VALUES_QUOTE_SPANNED,
    SELECT_ALT_3,
    SHARED_VALUES_STRINGIFY,
    TODO,
    SHARED_VALUES_TP,
    CONFIG_TRACING_TRACE,
    SHARED_VALUES_TRACE_SPAN,
    UNIMPLEMENTED,
    SHARED_VALUES_UNREACHABLE,
    SHARED_VALUES_VIEW,
    CONFIG_TRACING_WARN,
    SHARED_VALUES_WARN_SPAN,
    WRITE_ALT,
    SHARED_VALUES_WRITELN,
];
pub const ADMIN_TABLE_USER_SORTS: [(&str, &str); 4] = [
    (LOGIN, SHARED_VALUES_LOGIN_2),
    (DISPLAY_NAME, SHARED_VALUES_DISPLAY_NAME_2),
    (SQL_NAMES_ID, ID),
    (STATUS_ALT, SHARED_VALUES_STATUS_2),
];
pub const ADMIN_TABLE_ROLE_SORTS: [(&str, &str); 3] = [
    (NAME, SHARED_VALUES_NAME_2),
    (SQL_NAMES_ID, ID),
    (SYSTEM, SHARED_VALUES_SYSTEM_2),
];
pub const ADMIN_TABLE_PERMISSION_SORTS: [(&str, &str); 2] =
    [(NAME, SHARED_VALUES_NAME_2), (SQL_NAMES_ID, ID)];
pub const ADMIN_TABLE_AUDIT_SORTS: [(&str, &str); 5] = [
    (CREATED_AT, SHARED_VALUES_TIME),
    (USER_ID, SHARED_VALUES_USER),
    (ACTION, SHARED_VALUES_ACTION_2),
    (RESOURCE, SHARED_VALUES_RESOURCE_2),
    (SUCCEEDED, RESULT),
];
pub const WORKSPACE_TEST_RUNNER_CARGO: &str = "cargo";
pub const WORKSPACE_TEST_RUNNER_FORMAT_QUERY_PART_FRAGMENT: &str =
    "QueryPartFragment :: try_from (format !";
pub const WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_WORKLOAD: &str =
    "alloc-workload-generate-pg-table-src";
pub const WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_WORKLOAD: &str =
    "alloc-workload-generate-pg-types-src";
pub const WORKSPACE_TEST_RUNNER_MAJOR_PAGE_FAULTS_PREFIX: &str = "codex_major_page_faults=";
pub const WORKSPACE_TEST_RUNNER_MEMUSAGE_PATH: &str = "/usr/lib/x86_64-linux-gnu/libmemusage.so";
pub const WORKSPACE_TEST_RUNNER_LIBMEMUSAGE_TOOL: &str = "libmemusage";
pub const WORKSPACE_TEST_RUNNER_VALGRIND_TOOL: &str = "valgrind";
pub const WORKSPACE_TEST_RUNNER_VALGRIND_PATH: &str = "/usr/bin/valgrind";
pub const WORKSPACE_TEST_RUNNER_HEAPTRACK_TOOL: &str = "heaptrack";
pub const WORKSPACE_TEST_RUNNER_HEAPTRACK_PATH: &str = "/usr/bin/heaptrack";
pub const WORKSPACE_TEST_RUNNER_LTRACE_TOOL: &str = "ltrace";
pub const WORKSPACE_TEST_RUNNER_LTRACE_PATH: &str = "/usr/bin/ltrace";
pub const WORKSPACE_TEST_RUNNER_PERF_TOOL: &str = "perf";
pub const WORKSPACE_TEST_RUNNER_PERF_PATH: &str = "/usr/bin/perf";
pub const WORKSPACE_TEST_RUNNER_TIME_PATH: &str = "/usr/bin/time";
pub const WORKSPACE_TEST_RUNNER_GENERATE_PG_TABLE_MEASUREMENT: &str =
    "macro_generation_generate_pg_table_test";
pub const WORKSPACE_TEST_RUNNER_GENERATE_PG_TYPES_MEASUREMENT: &str =
    "macro_generation_generate_pg_types_test";
pub const WORKSPACE_TEST_RUNNER_GENERATE_WHERE_FILTERS_MEASUREMENT: &str =
    "macro_generation_generate_where_filters_test";
pub const WORKSPACE_TEST_RUNNER_MINOR_PAGE_FAULTS_PREFIX: &str = "codex_minor_page_faults=";
pub const WORKSPACE_TEST_RUNNER_PEAK_RSS_PREFIX: &str = "codex_peak_rss_kb=";
pub const WORKSPACE_TEST_RUNNER_PG_CRUD_COMMON_QUERY_PART_WORKLOAD: &str =
    "alloc-workload-pg-crud-common-query_part";
pub const WORKSPACE_TEST_RUNNER_RESULT_ROOT: &str = "test_results/workspace_test_runner";
pub const WORKSPACE_TEST_RUNNER_STATIC_WORKSPACE_PROFILE: &str = "static_workspace";
pub const WORKSPACE_TEST_RUNNER_STD_FMT_WRITE_CALL: &str = "std :: fmt :: Write :: write_fmt";
pub const WORKSPACE_TEST_RUNNER_STRING_WITH_CAPACITY_CALL: &str = "String :: with_capacity";
pub const WORKSPACE_TEST_RUNNER_WHERE_FILTERS_QUERY_PART_WORKLOAD: &str =
    "alloc-workload-where-filters-query_part";

pub const WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS: [&str; 7] = [
    CLIPPY,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_ALL_TARGETS,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_EMPTY,
    SHARED_VALUES_D,
    SHARED_VALUES_WARNINGS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_FMT_CHECK_ARGS: [&str; 2] =
    [SHARED_VALUES_FMT, SHARED_VALUES_CHECK_2];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS: [&str; 4] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_FEATURES,
    TEST_UTILS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_DOC_ARGS: [&str; 5] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_DOC,
    SHARED_VALUES_ALL_FEATURES,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TBL_ARGS: [&str; 6] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    P,
    SHARED_VALUES_GENERATE_PG_TABLE_TEST,
    SHARED_VALUES_FEATURES,
    TEST_UTILS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TYPES_ARGS: [&str; 6] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    P,
    SHARED_VALUES_GENERATE_PG_TYPES_TEST,
    SHARED_VALUES_FEATURES,
    TEST_UTILS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_WH_FLTS_ARGS: [&str; 6] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    P,
    SHARED_VALUES_GENERATE_WHERE_FILTERS_TEST,
    SHARED_VALUES_FEATURES,
    TEST_UTILS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_IGNORED_ARGS: [&str; 7] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_NO_FAIL_FAST,
    SHARED_VALUES_EMPTY,
    SHARED_VALUES_IGNORED,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_STYLE_ARGS: [&str; 5] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    P,
    TESTS_ALT,
    SHARED_VALUES_LIB,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_WORKSPACE_ARGS: [&str; 5] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_NO_FAIL_FAST,
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_HEAVY_ARGS: [&str; 7] = [
    NEXTEST,
    SHARED_VALUES_RUN,
    SHARED_VALUES_NO_FAIL_FAST,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_P_2,
    SHARED_VALUES_HEAVY_LOAD,
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_IGNORED_ARGS: [&str; 9] = [
    NEXTEST,
    SHARED_VALUES_RUN,
    SHARED_VALUES_NO_FAIL_FAST,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_P_2,
    WORKSPACE_TEST_RUNNER_STATIC_WORKSPACE_PROFILE,
    SHARED_VALUES_RUN_IGNORED,
    SHARED_VALUES_ONLY,
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_WORKSPACE_ARGS: [&str; 7] = [
    NEXTEST,
    SHARED_VALUES_RUN,
    SHARED_VALUES_NO_FAIL_FAST,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_P_2,
    WORKSPACE_TEST_RUNNER_STATIC_WORKSPACE_PROFILE,
];

pub const FOUR_SPACES: &str = "    ";
pub const THREE_SPACES: &str = "   ";
pub const TWO_SPACES: &str = "  ";
pub const SPACE: &str = " ";
pub const TEXT: &str = " (";
pub const FAILED: &str = " --- FAILED";
pub const FAILED_ALT: &str = " ... FAILED";
pub const PATH: &str = " :: ";
pub const PATH_SQLX_PATH_TYPE_NAME: &str = " :: sqlx :: type_name ";
pub const TEXT_ALT: &str = " = ";
pub const DOLLAR_1: &str = " = $1";
pub const FROM: &str = " FROM ";
pub const INTO: &str = " INTO ";
pub const AND: &str = " and ";
pub const FROM_ALT: &str = " from ";
pub const HTTPS_A_EXAMPLE_BAD_NEWLINE_VALUE_HTTPS_B_EXAMPLE: &str =
    " https://a.example ,bad\nvalue,https://b.example";
pub const IN: &str = " in (";
pub const RETURNING: &str = " returning ";
pub const SET: &str = " set ";
pub const WHERE: &str = " where ";
pub const TEXT_ALT_3: &str = " {}";
pub const COMPONENTS_SCHEMAS: &str = "#/components/schemas/";
pub const INLINE: &str = "#[inline]";
pub const NEWTYPE_AS_REF_OWNED_DOES_NOT_SUPPORT_REFERENCE_INNER_TYPES_USE_AS: &str =
    "#[newtype(as_ref_owned)] does not support reference inner types; use as_ref_inner";
pub const NEWTYPE_FROM_INNER_CANNOT_BE_USED_FOR_STRING_WRAPPERS_IMPLEMENT_TRYFROM_STRING: &str = "#[newtype(from_inner)] cannot be used for String wrappers; implement TryFrom<String> with a length check instead";
pub const DOLLAR_1_ALT: &str = "$1";
pub const DOLLAR_1_DOLLAR_2: &str = "$1,$2";
pub const DOLLAR_2: &str = "$2";
pub const DOLLAR_3: &str = "$3";
pub const DOLLAR_REF: &str = "$ref";
pub const PERCENT_A_PERCENT_B: &str = "%a\\%\\_b";
pub const PERCENT_A_PERCENT_B_PERCENT: &str = "%a\\%\\_b%";
pub const STR: &str = "&str";
pub const TEXT_ALT_4: &str = "\'\'";
pub const A: &str = "\'a\'";
pub const ABC: &str = "\'abc\'";
pub const DOLLAR_1_DOLLAR_2_DOLLAR_3_DOLLAR_4: &str = "($1,$2),($3,$4)";
pub const QUESTION_M_S_ASTERISK_A_Z0_9_A_Z0_9_PLUS_S: &str =
    "(?m)^\\s*([a-z0-9][a-z0-9_-]+)\\s+(allow|warn|deny|forbid)\\b";
pub const QUESTION_M_S_ASTERISK_A_ZA_Z0_9_PLUS_WORKSPACE_S_ASTERISK: &str =
    "(?m)^\\s*[A-Za-z0-9_-]+\\.workspace\\s*=\\s*true\\s*$";
pub const QUESTION_M_S_ASTERISK_CLIPPY_PATH_A_Z0_9_A_Z0_9: &str =
    "(?m)^\\s*clippy::([a-z0-9][a-z0-9_-]+)\\s+(allow|warn|deny|forbid)\\b";
pub const VALUES: &str = ") values ";
pub const TEXT_ALT_5: &str = ")";
pub const ASTERISK: &str = "*";
pub const TEXT_ALT_6: &str = ", ";
pub const TRUE_FAT_ARROW: &str = ", true =>";
pub const TEXT_ALT_7: &str = ",";
pub const USES: &str = "- uses: ";
pub const HYPHEN: &str = "-";
pub const DRY_RUN: &str = "--dry-run";
pub const VERSION: &str = "--version";
pub const W: &str = "-W";
pub const F: &str = "-f";
pub const P: &str = "-p";
pub const DOT: &str = ".";
pub const TEXT_ALT_8: &str = "..";
pub const TEXT_ALT_9: &str = "../";
pub const INITIALIZE_ENVIRONMENT_FILES_SRC: &str = "../initialize_environment_files/src/";
pub const PG_CRUD_PG_TABLE: &str = "../pg_crud/pg_table/";
pub const PG_CRUD_PG_TABLE_SRC_LIB_RS: &str = "../pg_crud/pg_table/src/lib.rs";
pub const PG_CRUD_PG_TYPES: &str = "../pg_crud/pg_types/";
pub const PG_CRUD_WHERE_FILTERS: &str = "../pg_crud/where_filters/";
pub const SERVER_ENV: &str = "../server/.env";
pub const SERVER_ENVEXAMPLE: &str = "../server/.envexample";
pub const SERVER_ADMIN_SRC_AUTH_RS: &str = "../server_admin/src/auth.rs";
pub const SERVER_ADMIN_SRC_AUTH_AUDIT_RS: &str = "../server_admin/src/auth/audit.rs";
pub const SERVER_ADMIN_SRC_AUTH_HANDLERS_RS: &str = "../server_admin/src/auth/handlers.rs";
pub const SERVER_ADMIN_SRC_AUTH_RATE_LIMIT_RS: &str = "../server_admin/src/auth/rate_limit.rs";
pub const SERVER_ADMIN_SRC_AUTH_SESSION_RS: &str = "../server_admin/src/auth/session.rs";
pub const SERVER_ADMIN_SRC_CLEANUP_RS: &str = "../server_admin/src/cleanup.rs";
pub const SERVER_ADMIN_SRC_MIGRATIONS_RS: &str = "../server_admin/src/migrations.rs";
pub const SERVER_ADMIN_FRONTEND_SRC_APP: &str = "../server_admin_frontend/src/app/";
pub const STR_CONSTANTS_SRC_LIB_RS: &str = "../str_constants/src/lib.rs";
pub const TESTS_SRC_CODE_STYLE: &str = "../tests/src/code_style";
pub const WORKSPACE_TEST_RUNNER_SRC: &str = "../workspace_test_runner/src/";
pub const ENV: &str = ".env";
pub const ENV_EXAMPLE: &str = ".env.example";
pub const ENVEXAMPLE: &str = ".envexample";
pub const EXPECT_CALL: &str = ".expect() call";
pub const FLATTEN_COLLECT: &str = ".flatten().collect";
pub const GIT: &str = ".git";
pub const MAP_VEC_PATH_FROM: &str = ".map(Vec::from)";
pub const UNWRAP_CALL: &str = ".unwrap() call";
pub const SLASH: &str = "/";
pub const ADMIN_PERMISSIONS_RM: &str = "/admin_permissions/rm";
pub const ADMIN_ROLE_PERMISSIONS_RM: &str = "/admin_role_permissions/rm";
pub const ADMIN_ROLES_RM: &str = "/admin_roles/rm";
pub const ADMIN_SYSTEM_SETTINGS_RM: &str = "/admin_system_settings/rm";
pub const ADMIN_USER_ROLES_RM: &str = "/admin_user_roles/rm";
pub const ADMIN_USERS_RM: &str = "/admin_users/rm";
pub const API: &str = "/api/";
pub const API_V1: &str = "/api/v1";
pub const API_V1_GIT_INFO: &str = "/api/v1/git_info";
pub const API_V1_TEST: &str = "/api/v1/test";
pub const COMPONENTS_SCHEMAS_ALT: &str = "/components/schemas";
pub const CONFIG_LIB: &str = "/config_lib/";
pub const FIRST: &str = "/first";
pub const INITIALIZE_ENVIRONMENT_FILES: &str = "/initialize_environment_files/";
pub const ITEMS_CM: &str = "/items/cm";
pub const ITEMS_CO: &str = "/items/co";
pub const MACRO_CLIPPY_CHECK_COMMON: &str = "/macro_clippy_check_common/";
pub const MACROS_HELPERS: &str = "/macros_helpers/";
pub const METRICS: &str = "/metrics";
pub const MISSING: &str = "/missing";
pub const MISSING_PATH: &str = "/missing/path";
pub const MISSING_PATH_QUESTION_LIMIT_10: &str = "/missing/path?limit=10";
pub const NOT_AN_API_ROUTE: &str = "/not-an-api-route";
pub const OPENAPI_JSON: &str = "/openapi.json";
pub const READ: &str = "/read";
pub const ROUTE: &str = "/route";
pub const SECOND: &str = "/second";
pub const SRC: &str = "/src/";
pub const STATUS: &str = "/status";
pub const STR_CONSTANTS: &str = "/str_constants/";
pub const TABLE_EXAMPLE_CM: &str = "/table_example/cm";
pub const TABLE_EXAMPLE_UO: &str = "/table_example/uo";
pub const TESTS: &str = "/tests/";
pub const TESTS_SRC: &str = "/tests/src/";
pub const TESTS_SRC_CODE_STYLE_ALT: &str = "/tests/src/code_style/";
pub const TESTS_SRC_LIB_RS: &str = "/tests/src/lib.rs";
pub const UNKNOWN: &str = "/unknown";
pub const USERS_ID: &str = "/users/{id}";
pub const WORKSPACE_TEST_RUNNER: &str = "/workspace_test_runner/";
pub const WRITE: &str = "/write";
pub const VALUE_0: &str = "0";
pub const VALUE_0047F74E: &str = "0047f74e";
pub const VALUE_00A995A4: &str = "00a995a4";
pub const VALUE_0242E1A9: &str = "0242e1a9";
pub const VALUE_029CB682: &str = "029cb682";
pub const VALUE_02BCD1C2: &str = "02bcd1c2";
pub const VALUE_0375574D: &str = "0375574d";
pub const VALUE_0391AC99: &str = "0391ac99";
pub const VALUE_05562DA0: &str = "05562da0";
pub const VALUE_0685FF24: &str = "0685ff24";
pub const VALUE_06A340B9: &str = "06a340b9";
pub const VALUE_0721B23F: &str = "0721b23f";
pub const VALUE_07504636: &str = "07504636";
pub const VALUE_078C759D: &str = "078c759d";
pub const VALUE_07D9FD90: &str = "07d9fd90";
pub const VALUE_08EF120F: &str = "08ef120f";
pub const VALUE_0935C11D: &str = "0935c11d";
pub const UPDATE_OPERATIONS_REQUIRE_AT_LEAST_ONE_NON_PRIMARY_KEY_FIELD: &str =
    "09a11adc: update operations require at least one non-primary-key field";
pub const VALUE_0A4FE013: &str = "0a4fe013";
pub const VALUE_0AC617DE: &str = "0ac617de";
pub const VALUE_0C3975A1: &str = "0c3975a1";
pub const VALUE_0C6362A4: &str = "0c6362a4";
pub const VALUE_0CB93D7F: &str = "0cb93d7f";
pub const VALUE_0CC47B2E: &str = "0cc47b2e";
pub const VALUE_0D8DF630: &str = "0d8df630";
pub const VALUE_0D9E4B7A: &str = "0d9e4b7a";
pub const VALUE_0DFD9A91: &str = "0dfd9a91";
pub const VALUE_0EA8D516: &str = "0ea8d516";
pub const VALUE_0ED905FF: &str = "0ed905ff";
pub const VALUE_0F30CA53: &str = "0f30ca53";
pub const VALUE_0F51DC7A: &str = "0f51dc7a";
pub const VALUE_1: &str = "1";
pub const VALUE_10: &str = "10";
pub const VALUE_1066857A: &str = "1066857a";
pub const VALUE_10C8F7D2: &str = "10c8f7d2";
pub const VALUE_114A573A: &str = "114a573a";
pub const VALUE_11CFCB27: &str = "11cfcb27";
pub const VALUE_11DDBA38: &str = "11ddba38";
pub const VALUE_12: &str = "12";
pub const VALUE_122809BA: &str = "122809ba";
pub const VALUE_1234567890: &str = "1234567890";
pub const VALUE_12653C9A: &str = "12653c9a";
pub const VALUE_127_0_0_1: &str = "127.0.0.1";
pub const VALUE_127_0_0_1_32_PATH_1_128: &str = "127.0.0.1/32,::1/128";
pub const VALUE_127_0_0_1_3000: &str = "127.0.0.1:3000";
pub const VALUE_127_0_0_1_43210: &str = "127.0.0.1:43210";
pub const VALUE_127_0_0_1_8080: &str = "127.0.0.1:8080";
pub const VALUE_127_0_0_2_43210: &str = "127.0.0.2:43210";
pub const VALUE_12817D29: &str = "12817d29";
pub const VALUE_1282B56E: &str = "1282b56e";
pub const VALUE_12ED6F85: &str = "12ed6f85";
pub const VALUE_13: &str = "13";
pub const VALUE_13DF9134: &str = "13df9134";
pub const VALUE_13FE8A6D: &str = "13fe8a6d";
pub const VALUE_14F304D8: &str = "14f304d8";
pub const VALUE_153B847C: &str = "153b847c";
pub const VALUE_168060A3: &str = "168060a3";
pub const VALUE_1736F4DB: &str = "1736f4db";
pub const VALUE_174A5D2F: &str = "174a5d2f";
pub const VALUE_17862DA9: &str = "17862da9";
pub const VALUE_18E07769: &str = "18e07769";
pub const VALUE_192_0_2_10_443: &str = "192.0.2.10:443";
pub const VALUE_192_0_2_11_443: &str = "192.0.2.11:443";
pub const VALUE_19512C63: &str = "19512c63";
pub const VALUE_195B48F5: &str = "195b48f5";
pub const VALUE_1970FD5B: &str = "1970fd5b";
pub const VALUE_19855EFD: &str = "19855efd";
pub const VALUE_1A2BB321: &str = "1a2bb321";
pub const VALUE_1CA76F8D: &str = "1ca76f8d";
pub const VALUE_1CABE205: &str = "1cabe205";
pub const VALUE_1D706D27: &str = "1d706d27";
pub const VALUE_1D97B31C: &str = "1d97b31c";
pub const VALUE_1E53A0C7: &str = "1e53a0c7";
pub const VALUE_1E97AD3B: &str = "1e97ad3b";
pub const VALUE_1E9E38EF: &str = "1e9e38ef";
pub const VALUE_1FC8C9F0: &str = "1fc8c9f0";
pub const VALUE_1FE7A3B4: &str = "1fe7a3b4";
pub const VALUE_1FE80AD3: &str = "1fe80ad3";
pub const VALUE_2: &str = "2";
pub const VALUE_20: &str = "20";
pub const VALUE_200: &str = "200";
pub const VALUE_200_OK: &str = "200_ok";
pub const VALUE_201: &str = "201";
pub const VALUE_2024: &str = "2024";
pub const VALUE_2026_07_13T12_30_00: &str = "2026-07-13T12:30:00";
pub const VALUE_2028024D: &str = "2028024d";
pub const VALUE_203_0_113_1: &str = "203.0.113.1";
pub const VALUE_203_0_113_1_NOT_AN_IP: &str = "203.0.113.1,not-an-ip";
pub const VALUE_203_0_113_2: &str = "203.0.113.2";
pub const VALUE_203_0_113_7: &str = "203.0.113.7";
pub const VALUE_203_0_113_7_10_0_0_8_10_0_0: &str = "203.0.113.7, 10.0.0.8, 10.0.0.9";
pub const VALUE_203_0_113_9: &str = "203.0.113.9";
pub const VALUE_20948D87: &str = "20948d87";
pub const VALUE_20D018AB: &str = "20d018ab";
pub const VALUE_21044EBA: &str = "21044eba";
pub const VALUE_2199F0A7: &str = "2199f0a7";
pub const VALUE_21AF9E85: &str = "21af9e85";
pub const VALUE_230693F3: &str = "230693f3";
pub const VALUE_2306B26A: &str = "2306b26a";
pub const VALUE_2376F58E: &str = "2376f58e";
pub const VALUE_2480F8C4: &str = "2480f8c4";
pub const VALUE_24EC178B: &str = "24ec178b";
pub const VALUE_2592000: &str = "2592000";
pub const VALUE_262819A8: &str = "262819a8";
pub const VALUE_26FC4688: &str = "26fc4688";
pub const VALUE_271F96D4: &str = "271f96d4";
pub const VALUE_274479A7: &str = "274479a7";
pub const VALUE_274D2E0C: &str = "274d2e0c";
pub const VALUE_27CE5FBD: &str = "27ce5fbd";
pub const VALUE_27DB915C: &str = "27db915c";
pub const VALUE_28CCDFC4: &str = "28ccdfc4";
pub const VALUE_28FCE6C8: &str = "28fce6c8";
pub const VALUE_290B56BB: &str = "290b56bb";
pub const VALUE_29AC89D5: &str = "29ac89d5";
pub const VALUE_29FC2F21: &str = "29fc2f21";
pub const VALUE_2B24EF1A: &str = "2b24ef1a";
pub const VALUE_2BFB0B62: &str = "2bfb0b62";
pub const VALUE_2C080F6D: &str = "2c080f6d";
pub const VALUE_2D67B058: &str = "2d67b058";
pub const VALUE_2D94C01E: &str = "2d94c01e";
pub const VALUE_2E03ECCC: &str = "2e03eccc";
pub const VALUE_2E7A9C4F: &str = "2e7a9c4f";
pub const VALUE_2E7CD5FE: &str = "2e7cd5fe";
pub const VALUE_2E86AA15: &str = "2e86aa15";
pub const VALUE_2ECB63C1: &str = "2ecb63c1";
pub const VALUE_2F2A7B69: &str = "2f2a7b69";
pub const VALUE_2F4D7A8C_FAILED_CONVERTING_STRING_LENGTH: &str =
    "2f4d7a8c failed converting string length";
pub const VALUE_2F6EE062: &str = "2f6ee062";
pub const VALUE_2FB3E958: &str = "2fb3e958";
pub const VALUE_2TABLE: &str = "2table";
pub const VALUE_30B575C6: &str = "30b575c6";
pub const VALUE_3130E593: &str = "3130e593";
pub const VALUE_3176B0D5: &str = "3176b0d5";
pub const VALUE_319B3CB4: &str = "319b3cb4";
pub const VALUE_31E0437D: &str = "31e0437d";
pub const VALUE_31EA9A57: &str = "31ea9a57";
pub const VALUE_320C7D1E: &str = "320c7d1e";
pub const VALUE_321360D4: &str = "321360d4";
pub const VALUE_326A4DA9: &str = "326a4da9";
pub const VALUE_32858863: &str = "32858863";
pub const VALUE_34: &str = "34";
pub const VALUE_348C0E57: &str = "348c0e57";
pub const VALUE_350646F2: &str = "350646f2";
pub const VALUE_3600: &str = "3600";
pub const VALUE_360DE719: &str = "360de719";
pub const VALUE_3664ECFF: &str = "3664ecff";
pub const VALUE_371082FA: &str = "371082fa";
pub const VALUE_37B593CE: &str = "37b593ce";
pub const VALUE_385EED61: &str = "385eed61";
pub const VALUE_3879E38D: &str = "3879e38d";
pub const VALUE_38819B94: &str = "38819b94";
pub const VALUE_39A0D238: &str = "39a0d238";
pub const VALUE_39A84C10: &str = "39a84c10";
pub const VALUE_3A9D7E2C: &str = "3a9d7e2c";
pub const VALUE_3B41DE7F: &str = "3b41de7f";
pub const VALUE_3BFEB37C: &str = "3bfeb37c";
pub const VALUE_3C20B457: &str = "3c20b457";
pub const VALUE_3CC52AC5: &str = "3cc52ac5";
pub const VALUE_3D70A4F4: &str = "3d70a4f4";
pub const VALUE_3DB98D20: &str = "3db98d20";
pub const VALUE_3DC31CC6: &str = "3dc31cc6";
pub const VALUE_3DE105A4: &str = "3de105a4";
pub const VALUE_3DFCA278: &str = "3dfca278";
pub const VALUE_3E33C100: &str = "3e33c100";
pub const VALUE_3E7ADF2F: &str = "3e7adf2f";
pub const VALUE_3F1C7BB7: &str = "3f1c7bb7";
pub const VALUE_3F6E8A12: &str = "3f6e8a12";
pub const VALUE_3F98F927: &str = "3f98f927";
pub const VALUE_4: &str = "4";
pub const VALUE_4063A869: &str = "4063a869";
pub const VALUE_42: &str = "42";
pub const VALUE_429: &str = "429";
pub const VALUE_42D13F7A: &str = "42d13f7a";
pub const VALUE_4304AB24: &str = "4304ab24";
pub const VALUE_449C3781: &str = "449c3781";
pub const VALUE_44C8AD59: &str = "44c8ad59";
pub const VALUE_44D17AB0: &str = "44d17ab0";
pub const VALUE_467A6513: &str = "467a6513";
pub const VALUE_46BC13A9: &str = "46bc13a9";
pub const VALUE_46CC9E0A: &str = "46cc9e0a";
pub const VALUE_46F3BEC1: &str = "46f3bec1";
pub const VALUE_46FB1C80: &str = "46fb1c80";
pub const VALUE_473577D5: &str = "473577d5";
pub const VALUE_475AF63B: &str = "475af63b";
pub const VALUE_4805266C: &str = "4805266c";
pub const VALUE_480B06EB: &str = "480b06eb";
pub const VALUE_48495BE4: &str = "48495be4";
pub const VALUE_489F8964: &str = "489f8964";
pub const VALUE_48EFED01: &str = "48efed01";
pub const VALUE_491EF4D6: &str = "491ef4d6";
pub const VALUE_49780295: &str = "49780295";
pub const VALUE_4A1791D2: &str = "4a1791d2";
pub const VALUE_4AB6A54C: &str = "4ab6a54c";
pub const VALUE_4AFBE04B: &str = "4afbe04b";
pub const VALUE_4B6C3BD6: &str = "4b6c3bd6";
pub const VALUE_4BBD5367: &str = "4bbd5367";
pub const VALUE_4BD3F0A1: &str = "4bd3f0a1";
pub const VALUE_4BD3FC27: &str = "4bd3fc27";
pub const VALUE_4CD32371: &str = "4cd32371";
pub const VALUE_4D0FA8E3: &str = "4d0fa8e3";
pub const VALUE_4D60C385: &str = "4d60c385";
pub const VALUE_4E1B2430: &str = "4e1b2430";
pub const VALUE_4E4CE16D: &str = "4e4ce16d";
pub const VALUE_4E8C040F: &str = "4e8c040f";
pub const VALUE_4EB1C098: &str = "4eb1c098";
pub const VALUE_4F08B7EC: &str = "4f08b7ec";
pub const VALUE_4F19D0D2: &str = "4f19d0d2";
pub const VALUE_4F607799: &str = "4f607799";
pub const VALUE_502918C1: &str = "502918c1";
pub const VALUE_503936EC: &str = "503936ec";
pub const VALUE_509F61F8: &str = "509f61f8";
pub const VALUE_50C1E4A8: &str = "50c1e4a8";
pub const VALUE_50E91EC9: &str = "50e91ec9";
pub const DUPLICATE_ORDER_OPTION: &str = "511d995e: duplicate order option";
pub const VALUE_517FD0C9: &str = "517fd0c9";
pub const VALUE_51D66E2C: &str = "51d66e2c";
pub const VALUE_52C9A1DB: &str = "52c9a1db";
pub const VALUE_53224F39: &str = "53224f39";
pub const DUPLICATE_PG_TYPE_CONFIG_ENTRY: &str = "536036f9: duplicate pg type config entry";
pub const VALUE_53A63100: &str = "53a63100";
pub const VALUE_546AF7B6: &str = "546af7b6";
pub const VALUE_5472EA19: &str = "5472ea19";
pub const VALUE_54B9DC03: &str = "54b9dc03";
pub const VALUE_550E8400_E29B_41D4_A716_446655440000: &str = "550e8400-e29b-41d4-a716-446655440000";
pub const VALUE_56E16453: &str = "56e16453";
pub const VALUE_57A61CA4: &str = "57a61ca4";
pub const VALUE_57CF209A: &str = "57cf209a";
pub const VALUE_58530F0E: &str = "58530f0e";
pub const VALUE_5994E7E2: &str = "5994e7e2";
pub const VALUE_59C80912: &str = "59c80912";
pub const VALUE_5A0BB723: &str = "5a0bb723";
pub const VALUE_5A52AF33: &str = "5a52af33";
pub const VALUE_5A831A2F: &str = "5a831a2f";
pub const VALUE_5A83F2BE: &str = "5a83f2be";
pub const VALUE_5B218444: &str = "5b218444";
pub const VALUE_5B8439C1: &str = "5b8439c1";
pub const VALUE_5B8BBDD1: &str = "5b8bbdd1";
pub const VALUE_5C10C931: &str = "5c10c931";
pub const VALUE_5C53D969: &str = "5c53d969";
pub const EXPECTED_A_STRUCT: &str = "5c79ab10: expected a struct";
pub const VALUE_5CD39E4B: &str = "5cd39e4b";
pub const VALUE_5CFDE4DD: &str = "5cfde4dd";
pub const VALUE_5D0D5BF0: &str = "5d0d5bf0";
pub const VALUE_5DC6F142: &str = "5dc6f142";
pub const VALUE_5E68820E: &str = "5e68820e";
pub const VALUE_5E7A83EB: &str = "5e7a83eb";
pub const VALUE_5EDC807F: &str = "5edc807f";
pub const VALUE_5EEA7F90: &str = "5eea7f90";
pub const VALUE_5EF927D2: &str = "5ef927d2";
pub const VALUE_5F28D14C_GENERATED_FILE_COMPARISON_OFFSET_OVERFLOW: &str =
    "5f28d14c generated file comparison offset overflow";
pub const VALUE_5F8A6D17: &str = "5f8a6d17";
pub const VALUE_5FB0627D: &str = "5fb0627d";
pub const VALUE_60: &str = "60";
pub const VALUE_60D99C87: &str = "60d99c87";
pub const VALUE_617F08B9: &str = "617f08b9";
pub const VALUE_623CDE18: &str = "623cde18";
pub const VALUE_634C635B: &str = "634c635b";
pub const VALUE_6353255D: &str = "6353255d";
pub const VALUE_64C4CC46: &str = "64c4cc46";
pub const VALUE_64E815EE: &str = "64e815ee";
pub const VALUE_65F2F229: &str = "65f2f229";
pub const VALUE_65FF827E: &str = "65ff827e";
pub const VALUE_6676E082: &str = "6676e082";
pub const VALUE_66B5606B: &str = "66b5606b";
pub const VALUE_6716175C: &str = "6716175c";
pub const VALUE_67503E70: &str = "67503e70";
pub const VALUE_6764152A: &str = "6764152a";
pub const VALUE_676C00F1: &str = "676c00f1";
pub const VALUE_67824B65: &str = "67824b65";
pub const VALUE_67973E68: &str = "67973e68";
pub const VALUE_6804382F: &str = "6804382f";
pub const VALUE_6863201E: &str = "6863201e";
pub const VALUE_68C0E12B: &str = "68c0e12b";
pub const VALUE_68E4F52D: &str = "68e4f52d";
pub const VALUE_695A2C2A: &str = "695a2c2a";
pub const VALUE_6A9F03D2: &str = "6a9f03d2";
pub const VALUE_6B4A128F: &str = "6b4a128f";
pub const VALUE_6BFF799B: &str = "6bff799b";
pub const VALUE_6C20F49A: &str = "6c20f49a";
pub const VALUE_6C338824: &str = "6c338824";
pub const VALUE_6D41C8E2: &str = "6d41c8e2";
pub const VALUE_6D9384FE: &str = "6d9384fe";
pub const VALUE_6E15EDEC: &str = "6e15edec";
pub const VALUE_6E423E16: &str = "6e423e16";
pub const VALUE_6E9ABF44: &str = "6e9abf44";
pub const VALUE_6F2C8A91: &str = "6f2c8a91";
pub const VALUE_6F4580CE: &str = "6f4580ce";
pub const VALUE_6FEE9F6F: &str = "6fee9f6f";
pub const VALUE_703A8DF2: &str = "703a8df2";
pub const VALUE_70761471: &str = "70761471";
pub const VALUE_7091840D: &str = "7091840d";
pub const VALUE_72860BF4: &str = "72860bf4";
pub const VALUE_728B52B3: &str = "728b52b3";
pub const VALUE_72E4A18D: &str = "72e4a18d";
pub const VALUE_7324AF80: &str = "7324af80";
pub const VALUE_735A2858: &str = "735a2858";
pub const VALUE_7393AFCA: &str = "7393afca";
pub const VALUE_73F8BC91: &str = "73f8bc91";
pub const VALUE_741E5201: &str = "741e5201";
pub const VALUE_74C1509E: &str = "74c1509e";
pub const VALUE_7557A4B4: &str = "7557a4b4";
pub const VALUE_756F3FE9: &str = "756f3fe9";
pub const VALUE_760545B6: &str = "760545b6";
pub const VALUE_762C1D9E: &str = "762c1d9e";
pub const VALUE_76314DB5: &str = "76314db5";
pub const VALUE_763E1BD9: &str = "763e1bd9";
pub const VALUE_76F6F737: &str = "76f6f737";
pub const VALUE_773C5AF2: &str = "773c5af2";
pub const VALUE_7795AF9B: &str = "7795af9b";
pub const VALUE_799DC227: &str = "799dc227";
pub const VALUE_79EE6381: &str = "79ee6381";
pub const VALUE_7A86A253: &str = "7a86a253";
pub const VALUE_7AD6DD07: &str = "7ad6dd07";
pub const VALUE_7AE01090: &str = "7ae01090";
pub const VALUE_7B93D4A1_6F28_4C70_9A51_2E8D3F640C12: &str = "7b93d4a1-6f28-4c70-9a51-2e8d3f640c12";
pub const VALUE_7B9AC2E3: &str = "7b9ac2e3";
pub const VALUE_7BE5F201: &str = "7be5f201";
pub const VALUE_7C2035B3: &str = "7c2035b3";
pub const VALUE_7C2531FD: &str = "7c2531fd";
pub const VALUE_7C9B7F2B: &str = "7c9b7f2b";
pub const VALUE_7C9E8046: &str = "7c9e8046";
pub const VALUE_7CF3FFC0: &str = "7cf3ffc0";
pub const VALUE_7D924F8A: &str = "7d924f8a";
pub const VALUE_7DA3CAE4: &str = "7da3cae4";
pub const VALUE_7E4B3F19: &str = "7e4b3f19";
pub const VALUE_7ED49BA1: &str = "7ed49ba1";
pub const VALUE_7F3A1C4E: &str = "7f3a1c4e";
pub const VALUE_7F419767: &str = "7f419767";
pub const VALUE_804F13B2: &str = "804f13b2";
pub const VALUE_80CB3EA4: &str = "80cb3ea4";
pub const VALUE_8103CD5F: &str = "8103cd5f";
pub const VALUE_818B46E8: &str = "818b46e8";
pub const VALUE_819ACD53: &str = "819acd53";
pub const VALUE_81F86E3F: &str = "81f86e3f";
pub const VALUE_8215B5F6: &str = "8215b5f6";
pub const VALUE_821D4A76: &str = "821d4a76";
pub const VALUE_82EAEA37: &str = "82eaea37";
pub const VALUE_82F4AC08: &str = "82f4ac08";
pub const VALUE_83087942: &str = "83087942";
pub const VALUE_8342AD27: &str = "8342ad27";
pub const VALUE_8357484D: &str = "8357484d";
pub const VALUE_837F89A0: &str = "837f89a0";
pub const VALUE_8406B933: &str = "8406b933";
pub const VALUE_8457A8CA: &str = "8457a8ca";
pub const VALUE_847A138F: &str = "847a138f";
pub const VALUE_84E57AB6: &str = "84e57ab6";
pub const VALUE_84F6A0D2: &str = "84f6a0d2";
pub const VALUE_85098DC5: &str = "85098dc5";
pub const VALUE_8567A9DF: &str = "8567a9df";
pub const VALUE_8672240F: &str = "8672240f";
pub const DUPLICATE_HIDDEN_OPTION: &str = "8689c32f: duplicate hidden option";
pub const VALUE_869D28D7: &str = "869d28d7";
pub const VALUE_86D3D452: &str = "86d3d452";
pub const VALUE_86EB20CF: &str = "86eb20cf";
pub const VALUE_874153EC: &str = "874153ec";
pub const VALUE_87B2E8FB: &str = "87b2e8fb";
pub const VALUE_8895CA50: &str = "8895ca50";
pub const DUPLICATE_GENERATE_PG_TABLE_FRONTEND_ATTRIBUTE: &str =
    "88a934b8: duplicate generate_pg_table_frontend attribute";
pub const VALUE_88DD90B8: &str = "88dd90b8";
pub const VALUE_891D7CA2: &str = "891d7ca2";
pub const VALUE_895E12FC: &str = "895e12fc";
pub const VALUE_89A2C4DE: &str = "89a2c4de";
pub const VALUE_8AD86515: &str = "8ad86515";
pub const DUPLICATE_LABEL_OPTION: &str = "8af07b63: duplicate label option";
pub const VALUE_8AF67E13: &str = "8af67e13";
pub const VALUE_8AFB4FFD: &str = "8afb4ffd";
pub const VALUE_8B79A379: &str = "8b79a379";
pub const VALUE_8BA5F1E7: &str = "8ba5f1e7";
pub const VALUE_8BCE26E7: &str = "8bce26e7";
pub const VALUE_8C89E84F: &str = "8c89e84f";
pub const VALUE_8C9F2A17: &str = "8c9f2a17";
pub const VALUE_8CE7A316: &str = "8ce7a316";
pub const VALUE_8D6F70BB: &str = "8d6f70bb";
pub const VALUE_8DA011BA: &str = "8da011ba";
pub const VALUE_8DB37A2F: &str = "8db37a2f";
pub const VALUE_8DB74CFD: &str = "8db74cfd";
pub const VALUE_8DCF412E: &str = "8dcf412e";
pub const VALUE_8DFC4389: &str = "8dfc4389";
pub const VALUE_8E427AD7: &str = "8e427ad7";
pub const VALUE_8E781C83: &str = "8e781c83";
pub const VALUE_8E9C3DA1: &str = "8e9c3da1";
pub const VALUE_8F6B2F31: &str = "8f6b2f31";
pub const VALUE_8F72B01E: &str = "8f72b01e";
pub const VALUE_8FF56AEB: &str = "8ff56aeb";
pub const VALUE_900: &str = "900";
pub const VALUE_90DF57A8: &str = "90df57a8";
pub const VALUE_90E5793B: &str = "90e5793b";
pub const VALUE_9106C1E6: &str = "9106c1e6";
pub const VALUE_91C59B94: &str = "91c59b94";
pub const VALUE_924BDC58: &str = "924bdc58";
pub const VALUE_926CE310: &str = "926ce310";
pub const VALUE_92B71C4E: &str = "92b71c4e";
pub const VALUE_92F9C5EC: &str = "92f9c5ec";
pub const VALUE_93CBF4A2: &str = "93cbf4a2";
pub const VALUE_93CE4136: &str = "93ce4136";
pub const VALUE_940EB924: &str = "940eb924";
pub const VALUE_94149BDD: &str = "94149bdd";
pub const VALUE_947FAED1: &str = "947faed1";
pub const VALUE_94A7E1CB: &str = "94a7e1cb";
pub const VALUE_94BC0508: &str = "94bc0508";
pub const VALUE_95D4595A: &str = "95d4595a";
pub const VALUE_95EC6823: &str = "95ec6823";
pub const VALUE_96213542: &str = "96213542";
pub const VALUE_962197B5: &str = "962197b5";
pub const VALUE_964E3EF4: &str = "964e3ef4";
pub const VALUE_9665F80A: &str = "9665f80a";
pub const VALUE_971ACE15: &str = "971ace15";
pub const VALUE_974BC327: &str = "974bc327";
pub const VALUE_97B5AD2F: &str = "97b5ad2f";
pub const VALUE_9811C7C7_D7F5_4FB7_9D25_AFFB0BD4F5FB: &str = "9811c7c7-d7f5-4fb7-9d25-affb0bd4f5fb";
pub const VALUE_982F4D17: &str = "982f4d17";
pub const DUPLICATE_PLACEHOLDER_OPTION: &str = "9898d208: duplicate placeholder option";
pub const VALUE_98A0357B_D21A_4949_A101_C641528D2376: &str = "98a0357b-d21a-4949-a101-c641528d2376";
pub const VALUE_98C7E04A: &str = "98c7e04a";
pub const VALUE_98C9CD5E: &str = "98c9cd5e";
pub const DUPLICATE_FILTERABLE_OPTION: &str = "99307572: duplicate filterable option";
pub const VALUE_9A672AC2: &str = "9a672ac2";
pub const VALUE_9A6D2C1B_DIFF_LEN_HELPER_REQUIRES_DIFFERENT_LENGTHS: &str =
    "9a6d2c1b diff-len helper requires different lengths";
pub const VALUE_9AC6D79A: &str = "9ac6d79a";
pub const VALUE_9B0E24F1: &str = "9b0e24f1";
pub const VALUE_9B4AB8AD: &str = "9b4ab8ad";
pub const VALUE_9BF4CE17: &str = "9bf4ce17";
pub const VALUE_9CBA6537: &str = "9cba6537";
pub const VALUE_9D1C7E4A: &str = "9d1c7e4a";
pub const VALUE_9D5A2DB0: &str = "9d5a2db0";
pub const VALUE_9D6A20AF: &str = "9d6a20af";
pub const VALUE_9DCB60BC: &str = "9dcb60bc";
pub const VALUE_9EA072C4: &str = "9ea072c4";
pub const VALUE_9F0BE285: &str = "9f0be285";
pub const VALUE_9F27B9CB: &str = "9f27b9cb";
pub const VALUE_9F2DB59C: &str = "9f2db59c";
pub const VALUE_9F3F5164: &str = "9f3f5164";
pub const VALUE_9F4D2A7C: &str = "9f4d2a7c";
pub const VALUE_9F8D72A1: &str = "9f8d72a1";
pub const VALUE_9FF40F7E: &str = "9ff40f7e";
pub const TEXT_ALT_10: &str = "://";
pub const PATH_SEPARATOR: &str = "::";
pub const PATH_UTC_PATH_NOW: &str = "::Utc::now";
pub const PATH_TRANSMUTE: &str = "::transmute";
pub const HTTPONLY: &str = "; HttpOnly";
pub const SECURE: &str = "; Secure";
pub const NON_PATH_TARGET: &str = "<non-path target>";
pub const REDACTED: &str = "<redacted>";
pub const TUPLE: &str = "<tuple>";
pub const CURRENT_DATE: &str = "= current_date";
pub const CURRENT_TIME: &str = "= current_time";
pub const CURRENT_TIMESTAMP: &str = "= current_timestamp";
pub const CURRENT_DATE_ALT: &str = "> current_date";
pub const CURRENT_TIME_ALT: &str = "> current_time";
pub const CURRENT_TIMESTAMP_ALT: &str = "> current_timestamp";
pub const TEXT_ALT_11: &str = ">";
pub const API_ALT: &str = "API";
pub const ADMIN: &str = "Admin";
pub const ADMINJWTSECRET: &str = "AdminJwtSecret";
pub const ADMINOPAQUETOKEN: &str = "AdminOpaqueToken";
pub const ADMINPASSWORD: &str = "AdminPassword";
pub const ADMINPASSWORDHASH: &str = "AdminPasswordHash";
pub const ADMINPERMISSIONSRMPAYLOAD: &str = "AdminPermissionsRmPayload";
pub const ADMINREFRESHTOKEN: &str = "AdminRefreshToken";
pub const ADMINROLEPERMISSIONSRMPAYLOAD: &str = "AdminRolePermissionsRmPayload";
pub const ADMINROLESRMPAYLOAD: &str = "AdminRolesRmPayload";
pub const ADMINROUTE_PATH_AUDIT: &str = "AdminRoute::Audit";
pub const ADMINROUTE_PATH_METRICS: &str = "AdminRoute::Metrics";
pub const ADMINROUTE_PATH_OPENAPI: &str = "AdminRoute::OpenApi";
pub const ADMINROUTE_PATH_PERMISSIONS: &str = "AdminRoute::Permissions";
pub const ADMINROUTE_PATH_REFRESH: &str = "AdminRoute::Refresh";
pub const ADMINROUTE_PATH_ROLES: &str = "AdminRoute::Roles";
pub const ADMINROUTE_PATH_SETTINGS: &str = "AdminRoute::Settings";
pub const ADMINROUTE_PATH_SIGNIN: &str = "AdminRoute::SignIn";
pub const ADMINROUTE_PATH_SIGNOUT: &str = "AdminRoute::SignOut";
pub const ADMINROUTE_PATH_USERS: &str = "AdminRoute::Users";
pub const ADMINSYSTEMSETTINGSRMPAYLOAD: &str = "AdminSystemSettingsRmPayload";
pub const ADMINTOKENHASH: &str = "AdminTokenHash";
pub const ADMINUSERROLESRMPAYLOAD: &str = "AdminUserRolesRmPayload";
pub const ADMINUSERS: &str = "AdminUsers";
pub const ADMINUSERSRMPAYLOAD: &str = "AdminUsersRmPayload";
pub const ALL: &str = "All";
pub const ARC: &str = "Arc";
pub const ARC_PATH_NEW_OUTSIDE_APPROVED_CROSS_THREAD_STATE_CONSTRUCTION: &str =
    "Arc::new() outside approved cross-thread state construction";
pub const AS: &str = "As";
pub const ASNULLABLE: &str = "AsNullable";
pub const AUDIT_LOG: &str = "Audit log";
pub const BTREEMAP: &str = "BTreeMap";
pub const BTREESET: &str = "BTreeSet";
pub const BTREESET_STRING: &str = "BTreeSet<String>";
pub const BOUNDEDSTRING_DOES_NOT_SUPPORT_GENERICS: &str = "BoundedString does not support generics";
pub const BOUNDEDSTRING_SUPPORTS_ONLY_STRING_TUPLE_STRUCTS: &str =
    "BoundedString supports only String tuple structs";
pub const BOUNDEDSTRING_UTOIPA_REQUIRES_CHARS_SO_OPENAPI_LENGTH_SEMANTICS_MATCH_RUNTIME: &str =
    "BoundedString utoipa requires chars so OpenAPI length semantics match runtime";
pub const BOUNDEDSTRING: &str = "BoundedString";
pub const BOUNDEDVEC: &str = "BoundedVec";
pub const BOX: &str = "Box";
pub const CM_CHUNK_SIZE_2EE9377B: &str = "CM_CHUNK_SIZE_2EE9377B";
pub const CM_CHUNK_SIZE_A13F7C92: &str = "CM_CHUNK_SIZE_A13F7C92";
pub const CM_CONCURRENCY_7CCFD82D: &str = "CM_CONCURRENCY_7CCFD82D";
pub const CONFIG_LIB_TEST_ENV_VAR_4E8A7F21: &str = "CONFIG_LIB_TEST_ENV_VAR_4E8A7F21";
pub const CREATE_INDEX_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_CREATED_AT_IDX_ON: &str = "CREATE INDEX IF NOT EXISTS pg_table_idempotency_created_at_idx ON pg_table_idempotency(created_at)";
pub const CREATE_SCHEMA_ADMIN_MIGRATION_FRESH_TEST_CREATE_SCHEMA_ADMIN_MIGRATION_UPGRADE_TEST:
    &str = "CREATE SCHEMA admin_migration_fresh_test; CREATE SCHEMA admin_migration_upgrade_test";
pub const CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ACTOR_TEXT_NOT_NULL: &str = "CREATE TABLE IF NOT EXISTS pg_table_idempotency (actor TEXT NOT NULL, http_method TEXT NOT NULL CHECK (http_method IN (\'POST\',\'PATCH\',\'DELETE\')), route_path TEXT NOT NULL CHECK (route_path LIKE \'/%\'), idempotency_key TEXT NOT NULL, request_hash BYTEA NOT NULL CHECK (octet_length(request_hash) = 32), response_status SMALLINT, response_body BYTEA, state TEXT NOT NULL CHECK (state IN (\'pending\',\'completed\')), created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), completed_at TIMESTAMPTZ, PRIMARY KEY (actor,http_method,route_path,idempotency_key), CHECK ((state = \'pending\' AND response_status IS NULL AND response_body IS NULL AND completed_at IS NULL) OR (state = \'completed\' AND response_status IS NOT NULL AND response_body IS NOT NULL AND completed_at IS NOT NULL)))";
pub const CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_BIGINT: &str =
    "CREATE TABLE IF NOT EXISTS pg_table_idempotency_atomic_test (id BIGINT PRIMARY KEY)";
pub const CREATE_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_BIGINT_PRIMARY_KEY_REVISION: &str = "CREATE TABLE pg_table_optimistic_revision_test (id BIGINT PRIMARY KEY, revision BIGINT NOT NULL, value BIGINT NOT NULL)";
pub const CSRF_TOKEN_BOUND_TO_THE_ADMINISTRATOR_ACCESS_SESSION: &str =
    "CSRF token bound to the administrator access session";
pub const CSRF_VALIDATION_FAILED: &str = "CSRF validation failed";
pub const CARGO_TOML: &str = "Cargo.toml";
pub const CFG: &str = "Cfg";
pub const CLIENT: &str = "Client";
pub const CMERRORVARIANTS: &str = "CmErrorVariants";
pub const COERRORVARIANTS: &str = "CoErrorVariants";
pub const COMMIT: &str = "Commit";
pub const COMMONERRORVARIANTS: &str = "CommonErrorVariants";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const COW: &str = "Cow";
pub const DELETE_FROM_ADMIN_AUDIT_LOG: &str = "DELETE FROM admin_audit_log";
pub const DELETE_FROM_ADMIN_ROLE_PERMISSIONS_WHERE_ROLE_ID_DOLLAR_1: &str =
    "DELETE FROM admin_role_permissions WHERE role_id = $1";
pub const DELETE_FROM_ADMIN_ROLES_WHERE_ID_DOLLAR_1_AND_IS_SYSTEM_FALSE: &str =
    "DELETE FROM admin_roles WHERE id = $1 AND is_system = FALSE RETURNING TRUE";
pub const DELETE_FROM_ADMIN_USER_ROLES_WHERE_USER_ID_DOLLAR_1: &str =
    "DELETE FROM admin_user_roles WHERE user_id = $1";
pub const DELETE_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_RETURNING_TRUE: &str =
    "DELETE FROM admin_users WHERE id = $1 RETURNING TRUE";
pub const DELETE_FROM_PG_TABLE_IDEMPOTENCY_WHERE_ACTOR_DOLLAR_1_AND_HTTP_METHOD: &str = "DELETE FROM pg_table_idempotency WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4 AND request_hash=$5 AND state=\'pending\'";
pub const DENY: &str = "DENY";
pub const DROP_SCHEMA_IF_EXISTS_ADMIN_MIGRATION_FRESH_TEST_CASCADE_DROP_SCHEMA_IF: &str = "DROP SCHEMA IF EXISTS admin_migration_fresh_test CASCADE; DROP SCHEMA IF EXISTS admin_migration_upgrade_test CASCADE";
pub const DROP_SCHEMA_ADMIN_MIGRATION_FRESH_TEST_CASCADE_DROP_SCHEMA_ADMIN_MIGRATION_UPGRADE: &str = "DROP SCHEMA admin_migration_fresh_test CASCADE; DROP SCHEMA admin_migration_upgrade_test CASCADE";
pub const DROP_TABLE_IF_EXISTS_PG_TABLE_OPTIMISTIC_REVISION_TEST: &str =
    "DROP TABLE IF EXISTS pg_table_optimistic_revision_test";
pub const DROP_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST: &str =
    "DROP TABLE pg_table_optimistic_revision_test";
pub const DTOKENSTREAMBUILDER: &str = "DTokenStreamBuilder";
pub const DLOERRORVARIANTS: &str = "DloErrorVariants";
pub const DMERRORVARIANTS: &str = "DmErrorVariants";
pub const DYNARC: &str = "DynArc";
pub const ENUMFROMSTR_SUPPORTS_ONLY_ENUMS: &str = "EnumFromStr supports only enums";
pub const ENUMFROMSTR_SUPPORTS_ONLY_UNIT_VARIANTS: &str = "EnumFromStr supports only unit variants";
pub const ERR: &str = "Err";
pub const ERR_ERROR: &str = "Err(\"error\")";
pub const ERROR: &str = "Error";
pub const ERRORWITHSERDE: &str = "ErrorWithSerde";
pub const FROM_ALT_3: &str = "From";
pub const GET: &str = "GET";
pub const GITHUB: &str = "GiThUb";
pub const HASHMAP: &str = "HashMap";
pub const HASHSET: &str = "HashSet";
pub const HASHSET_STR: &str = "HashSet<&str>";
pub const HELLO_WORLD: &str = "Hello, world!";
pub const HELLOWORLD: &str = "HelloWorld";
pub const HTTPONLY_ADMINISTRATOR_ACCESS_TOKEN_COOKIE: &str =
    "HttpOnly administrator access token cookie";
pub const ID: &str = "ID";
pub const INSERT_INTO_ADMIN_ACCESS_SESSIONS_ID_USER_ID_TOKEN_IDENTIFIER_HASH_TOKEN: &str = "INSERT INTO admin_access_sessions (id, user_id, token_identifier_hash, token_context_hash, csrf_token_hash, expires_at) VALUES ($1, $2, $3, $4, $5, NOW() + ($6 * INTERVAL \'1 second\'))";
pub const INSERT_INTO_ADMIN_AUDIT_LOG_ACTION_RESOURCE_SUCCEEDED_CREATED_AT_SELECT_TEST: &str = "INSERT INTO admin_audit_log (action,resource,succeeded,created_at) SELECT \'test\',\'test\',TRUE,TIMESTAMPTZ \'2000-01-01 00:00:00+00\' FROM generate_series(1,3)";
pub const INSERT_INTO_ADMIN_AUDIT_LOG_USER_ID_USER_LOGIN_ACTION_RESOURCE_RESOURCE: &str = "INSERT INTO admin_audit_log (user_id, user_login, action, resource, resource_id, request_id, succeeded, details) VALUES ($1, $2, $3, $4, $5, $6, TRUE, $7)";
pub const INSERT_INTO_ADMIN_LOGIN_ATTEMPTS_LOGIN_SUCCEEDED_ATTEMPTED_AT_SELECT_OLD_VALUE: &str = "INSERT INTO admin_login_attempts (login,succeeded,attempted_at) SELECT \'old-\' || value::TEXT,FALSE,TIMESTAMPTZ \'2000-01-01 00:00:00+00\' FROM generate_series(1,3) value";
pub const INSERT_INTO_ADMIN_RATE_LIMITS_SCOPE_SUBJECT_WINDOW_STARTED_AT_REQUEST_COUNT: &str = "INSERT INTO admin_rate_limits (scope, subject, window_started_at, request_count) VALUES ($1, $2, NOW(), 1) ON CONFLICT (scope, subject) DO UPDATE SET window_started_at = CASE WHEN admin_rate_limits.window_started_at <= NOW() - make_interval(secs => $4) THEN NOW() ELSE admin_rate_limits.window_started_at END, request_count = CASE WHEN admin_rate_limits.window_started_at <= NOW() - make_interval(secs => $4) THEN 1 ELSE admin_rate_limits.request_count + 1 END RETURNING request_count <= $3";
pub const INSERT_INTO_ADMIN_RATE_LIMITS_SCOPE_SUBJECT_WINDOW_STARTED_AT_REQUEST_COUNT_ALT: &str = "INSERT INTO admin_rate_limits (scope,subject,window_started_at,request_count) SELECT \'test\',\'old-\' || value::TEXT,TIMESTAMPTZ \'2000-01-01 00:00:00+00\',1 FROM generate_series(1,3) value";
pub const INSERT_INTO_ADMIN_REFRESH_TOKENS_ID_USER_ID_TOKEN_HASH_EXPIRES_AT: &str = "INSERT INTO admin_refresh_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, NOW() + ($4 * INTERVAL \'1 second\'))";
pub const INSERT_INTO_ADMIN_ROLE_PERMISSIONS_ROLE_ID_PERMISSION_ID_SELECT_DOLLAR_1: &str = "INSERT INTO admin_role_permissions (role_id, permission_id) SELECT $1, permission_id FROM UNNEST($2::BIGINT[]) AS permission_id";
pub const INSERT_INTO_ADMIN_ROLES_NAME_IS_SYSTEM_VALUES_DOLLAR_1_FALSE_RETURNING: &str =
    "INSERT INTO admin_roles (name, is_system) VALUES ($1, FALSE) RETURNING id";
pub const INSERT_INTO_ADMIN_USER_ROLES_USER_ID_ROLE_ID_SELECT_DOLLAR_1: &str = "INSERT INTO admin_user_roles (user_id, role_id) SELECT $1, id FROM admin_roles WHERE name = \'admin\'";
pub const INSERT_INTO_ADMIN_USER_ROLES_USER_ID_ROLE_ID_SELECT_DOLLAR_1_ALT: &str = "INSERT INTO admin_user_roles (user_id, role_id) SELECT $1, role_id FROM UNNEST($2::BIGINT[]) AS role_id";
pub const INSERT_INTO_PG_TABLE_IDEMPOTENCY_ACTOR_HTTP_METHOD_ROUTE_PATH_IDEMPOTENCY_KEY: &str = "INSERT INTO pg_table_idempotency (actor,http_method,route_path,idempotency_key,request_hash,state) VALUES ($1,$2,$3,$4,$5,\'pending\') ON CONFLICT DO NOTHING RETURNING TRUE";
pub const INSERT_INTO_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_VALUES_1: &str =
    "INSERT INTO pg_table_idempotency_atomic_test (id) VALUES (1)";
pub const INSERT_INTO_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_REVISION_VALUE_VALUES_1: &str =
    "INSERT INTO pg_table_optimistic_revision_test (id,revision,value) VALUES (1,0,0)";
pub const IDEMPOTENCY_KEY: &str = "Idempotency-Key";
pub const IF_MATCH: &str = "If-Match";
pub const LD_PRELOAD: &str = "LD_PRELOAD";
pub const LOCK_TABLE_ADMIN_USERS_IN_EXCLUSIVE_MODE: &str =
    "LOCK TABLE admin_users IN EXCLUSIVE MODE";
pub const LOCATION: &str = "Location";
pub const MEMUSAGE_PROG_NAME: &str = "MEMUSAGE_PROG_NAME";
pub const MEMORY_USAGE_SUMMARY: &str = "Memory usage summary:";
pub const METRICS_ALT: &str = "Metrics";
pub const MUTEX_TYPE_USAGE: &str = "Mutex type usage";
pub const MUTEX: &str = "Mutex";
pub const NEWTYPE_FIELD_NOT_FOUND: &str = "Newtype field not found";
pub const NEWTYPE_REQUIRES_AT_LEAST_ONE_NEWTYPE_OPTION: &str =
    "Newtype requires at least one #[newtype(...)] option";
pub const NEWTYPE_SUPPORTS_ONLY_ONE_FIELD_TUPLE_STRUCTS: &str =
    "Newtype supports only one-field tuple structs";
pub const NONNULL: &str = "NonNull";
pub const NONPRIMARYKEYPGTYPEREADIDS: &str = "NonPrimaryKeyPgTypeReadIds";
pub const NONE: &str = "None";
pub const NULLABLE: &str = "Nullable";
pub const OK: &str = "Ok";
pub const OK_5: &str = "Ok(5)";
pub const ONLY: &str = "Only";
pub const OPTION: &str = "Option";
pub const OPTION_STR: &str = "Option<&str>";
pub const OPTION_TYPES_PATH_SOURCETEXTREF: &str = "Option<types::SourceTextRef>";
pub const OPTIONAL: &str = "Optional";
pub const ORDERBY: &str = "OrderBy";
pub const PATCH: &str = "PATCH";
pub const PATH_ALT: &str = "PATH";
pub const POST: &str = "POST";
pub const PUBLIC_VALUE_NEWLINE_SECRET_CHANGE_ME_NEWLINE: &str = "PUBLIC=value\nSECRET=change-me\n";
pub const PARAMETERS: &str = "Parameters";
pub const PAYLOADTRYNEWERROR: &str = "PayloadTryNewError";
pub const PERMISSIONS: &str = "Permissions";
pub const PHANTOMDATA: &str = "PhantomData";
pub const PIN: &str = "Pin";
pub const POSTGRESQL_IDEMPOTENCY_OPERATION_FAILED: &str = "PostgreSQL idempotency operation failed";
pub const REDACTED_ALT: &str = "REDACTED";
pub const RC: &str = "Rc";
pub const REQUEST_RATE_LIMIT_EXCEEDED: &str = "Request rate limit exceeded";
pub const RESVARIANTS: &str = "ResVariants";
pub const RESULT: &str = "Result";
pub const RETRY_AFTER: &str = "Retry-After";
pub const RMERRORVARIANTS: &str = "RmErrorVariants";
pub const ROERRORVARIANTS: &str = "RoErrorVariants";
pub const ROLES: &str = "Roles";
pub const ROOT_ADMIN: &str = "Root Admin";
pub const SECRET_CUSTOM_NEWLINE: &str = "SECRET=custom\n";
pub const SELECT: &str = "SELECT ";
pub const SELECT_SELECT_COUNT_ASTERISK_FROM_ADMIN_LOGIN_ATTEMPTS_SELECT_COUNT_ASTERISK_FROM: &str = "SELECT (SELECT COUNT(*) FROM admin_login_attempts),(SELECT COUNT(*) FROM admin_rate_limits),(SELECT COUNT(*) FROM admin_audit_log)";
pub const SELECT_SELECT_MAX_VERSION_FROM_ADMIN_MIGRATION_FRESH_TEST_SQLX_MIGRATIONS_WHERE: &str = "SELECT (SELECT MAX(version) FROM admin_migration_fresh_test._sqlx_migrations WHERE success = TRUE),(SELECT MAX(version) FROM admin_migration_upgrade_test._sqlx_migrations WHERE success = TRUE)";
pub const SELECT_COUNT_ASTERISK_FROM_ADMIN_LOGIN_ATTEMPTS_WHERE_LOGIN_DOLLAR_1_AND: &str = "SELECT COUNT(*) FROM admin_login_attempts WHERE login = $1 AND succeeded = FALSE AND attempted_at > NOW() - INTERVAL \'15 minutes\'";
pub const SELECT_COUNT_ASTERISK_FROM_ADMIN_PERMISSIONS_WHERE_ID_ANY_DOLLAR_1: &str =
    "SELECT COUNT(*) FROM admin_permissions WHERE id = ANY($1)";
pub const SELECT_COUNT_ASTERISK_FROM_ADMIN_ROLE_PERMISSIONS_LINK_LEFT_JOIN_ADMIN_ROLES: &str = "SELECT COUNT(*) FROM admin_role_permissions link LEFT JOIN admin_roles role ON role.id = link.role_id LEFT JOIN admin_permissions permission ON permission.id = link.permission_id WHERE role.id IS NULL OR permission.id IS NULL";
pub const SELECT_COUNT_ASTERISK_FROM_ADMIN_ROLES_WHERE_ID_ANY_DOLLAR_1: &str =
    "SELECT COUNT(*) FROM admin_roles WHERE id = ANY($1)";
pub const SELECT_COUNT_ASTERISK_FROM_ADMIN_USER_ROLES_LINK_LEFT_JOIN_ADMIN_USERS: &str = "SELECT COUNT(*) FROM admin_user_roles link LEFT JOIN admin_users usr ON usr.id = link.user_id LEFT JOIN admin_roles role ON role.id = link.role_id WHERE usr.id IS NULL OR role.id IS NULL";
pub const SELECT_COUNT_ASTERISK_FROM_ADMIN_USERS: &str = "SELECT COUNT(*) FROM admin_users";
pub const SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY: &str =
    "SELECT COUNT(*) FROM pg_table_idempotency";
pub const SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST: &str =
    "SELECT COUNT(*) FROM pg_table_idempotency_atomic_test";
pub const SELECT_COUNT_DISTINCT_USERS_ID_FROM_ADMIN_USERS_USERS_JOIN_ADMIN_USER: &str = "SELECT COUNT(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id WHERE user_role.role_id = $1 AND users.is_banned = FALSE";
pub const SELECT_DISTINCT_PERMISSION_NAME_FROM_ADMIN_PERMISSIONS_PERMISSION_JOIN_ADMIN_ROLE_PERMISSIONS: &str = "SELECT DISTINCT permission.name FROM admin_permissions permission JOIN admin_role_permissions role_permission ON role_permission.permission_id = permission.id JOIN admin_user_roles user_role ON user_role.role_id = role_permission.role_id WHERE user_role.user_id = $1 ORDER BY permission.name";
pub const SELECT_EXISTS_SELECT_1_FROM_ADMIN_ACCESS_SESSIONS_SESSION_JOIN_ADMIN_USERS: &str = "SELECT EXISTS (SELECT 1 FROM admin_access_sessions session JOIN admin_users users ON users.id = session.user_id WHERE session.id = $1 AND session.user_id = $2 AND session.token_context_hash = $3 AND session.revoked_at IS NULL AND session.expires_at > NOW() AND users.is_banned = FALSE)";
pub const SELECT_EXISTS_SELECT_1_FROM_ADMIN_USER_ROLES_WHERE_USER_ID_DOLLAR: &str =
    "SELECT EXISTS (SELECT 1 FROM admin_user_roles WHERE user_id = $1 AND role_id = $2)";
pub const SELECT_EXISTS_SELECT_1_FROM_ADMIN_USERS: &str =
    "SELECT EXISTS (SELECT 1 FROM admin_users)";
pub const SELECT_MAX_VERSION_FROM_SQLX_MIGRATIONS_WHERE_SUCCESS_TRUE: &str =
    "SELECT MAX(version) FROM _sqlx_migrations WHERE success = TRUE";
pub const SELECT_NOT_IS_BANNED_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_FOR: &str =
    "SELECT NOT is_banned FROM admin_users WHERE id = $1 FOR UPDATE";
pub const SELECT_CSRF_TOKEN_HASH_FROM_ADMIN_ACCESS_SESSIONS_WHERE_ID_DOLLAR_1: &str = "SELECT csrf_token_hash FROM admin_access_sessions WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL AND expires_at > NOW()";
pub const SELECT_ID_FROM_ADMIN_ROLES_WHERE_NAME_ADMIN_AND_IS_SYSTEM_TRUE: &str =
    "SELECT id FROM admin_roles WHERE name = \'admin\' AND is_system = TRUE";
pub const SELECT_ID_FROM_ADMIN_ROLES_WHERE_NAME_TEMPORARY_ROLE: &str =
    "SELECT id FROM admin_roles WHERE name = \'temporary_role\'";
pub const SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_LIMITED_USER: &str =
    "SELECT id FROM admin_users WHERE login = \'limited_user\'";
pub const SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ROOT_ADMIN: &str =
    "SELECT id FROM admin_users WHERE login = \'root_admin\'";
pub const SELECT_ID_CREATED_AT_PATH_TEXT_EXPIRES_AT_PATH_TEXT_FROM_ADMIN: &str = "SELECT id, created_at::TEXT, expires_at::TEXT FROM admin_access_sessions WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > NOW() ORDER BY created_at DESC";
pub const SELECT_ID_LOGIN_DISPLAY_NAME_IS_BANNED_FROM_ADMIN_USERS_ORDER_BY: &str =
    "SELECT id, login, display_name, is_banned FROM admin_users ORDER BY login LIMIT 500";
pub const SELECT_ID_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME: &str =
    "SELECT id, name FROM admin_permissions ORDER BY name";
pub const SELECT_ID_NAME_IS_SYSTEM_FROM_ADMIN_ROLES_ORDER_BY_NAME: &str =
    "SELECT id, name, is_system FROM admin_roles ORDER BY name";
pub const SELECT_ID_PASSWORD_HASH_IS_BANNED_FROM_ADMIN_USERS_WHERE_LOWER_LOGIN: &str =
    "SELECT id, password_hash, is_banned FROM admin_users WHERE lower(login) = lower($1)";
pub const SELECT_ID_USER_ID_USER_LOGIN_ACTION_RESOURCE_RESOURCE_ID_SUCCEEDED_DETAILS: &str = "SELECT id, user_id, user_login, action, resource, resource_id, succeeded, details, created_at::TEXT FROM admin_audit_log WHERE ($1::BIGINT IS NULL OR user_id = $1) AND ($2::TEXT IS NULL OR action = $2) AND ($3::TEXT IS NULL OR resource = $3) AND ($4::TIMESTAMPTZ IS NULL OR created_at >= $4::TIMESTAMPTZ) AND ($5::TIMESTAMPTZ IS NULL OR created_at <= $5::TIMESTAMPTZ) ORDER BY created_at DESC LIMIT 200";
pub const SELECT_IS_SYSTEM_FROM_ADMIN_ROLES_WHERE_ID_DOLLAR_1_FOR_UPDATE: &str =
    "SELECT is_system FROM admin_roles WHERE id = $1 FOR UPDATE";
pub const SELECT_LOGIN_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_AND_IS_BANNED: &str =
    "SELECT login FROM admin_users WHERE id = $1 AND is_banned = FALSE";
pub const SELECT_LOGIN_DISPLAY_NAME_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_AND: &str =
    "SELECT login, display_name FROM admin_users WHERE id = $1 AND is_banned = FALSE";
pub const SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ROOT_ADMIN: &str =
    "SELECT password_hash FROM admin_users WHERE login = \'root_admin\'";
pub const SELECT_REQUEST_HASH_STATE_RESPONSE_STATUS_RESPONSE_BODY_FROM_PG_TABLE_IDEMPOTENCY: &str = "SELECT request_hash,state,response_status,response_body FROM pg_table_idempotency WHERE actor=$1 AND http_method=$2 AND route_path=$3 AND idempotency_key=$4";
pub const SELECT_ROLE_NAME_FROM_ADMIN_ROLES_ROLE_JOIN_ADMIN_USER_ROLES_LINK: &str = "SELECT role.name FROM admin_roles role JOIN admin_user_roles link ON link.role_id = role.id WHERE link.user_id = $1 ORDER BY role.name";
pub const SELECT_SITE_NAME_TAB_TITLE_MAIN_LOGO_PRIMARY_COLOR_DEFAULT_ADMIN_ROUTE: &str = "SELECT site_name, tab_title, main_logo, primary_color, default_admin_route, organization_name, organization_contacts, support_url FROM admin_system_settings WHERE id = 1";
pub const SELECT_SUCCEEDED_COUNT_ASTERISK_FROM_ADMIN_AUDIT_LOG_GROUP_BY_SUCCEEDED_ORDER: &str =
    "SELECT succeeded, COUNT(*) FROM admin_audit_log GROUP BY succeeded ORDER BY succeeded";
pub const SELECT_USER_ID_FROM_ADMIN_REFRESH_TOKENS_WHERE_TOKEN_HASH_DOLLAR_1: &str = "SELECT user_id FROM admin_refresh_tokens WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW() FOR UPDATE";
pub const SET_LOCAL_APP_ADMIN_AUDIT_CLEANUP_ON: &str = "SET LOCAL app.admin_audit_cleanup = \'on\'";
pub const SELF: &str = "Self";
pub const SELF_V: &str = "Self{v}";
pub const SETTINGS: &str = "Settings";
pub const SHARED: &str = "Shared";
pub const SNAKECASE: &str = "SnakeCase";
pub const SOME_7: &str = "Some(7)";
pub const SOME_ABC: &str = "Some(\"abc\")";
pub const SOURCETEXT: &str = "SourceText";
pub const SQLCOLUMNREF: &str = "SqlColumnRef";
pub const SQLXPOSTGRESQUERY: &str = "SqlxPostgresQuery";
pub const STDARCCOMMONROUTESAPPSTATE: &str = "StdArcCommonRoutesAppState";
pub const STDLOCATIONDURATION: &str = "StdLocationDuration";
pub const STDOPTIONALOPTIONAL: &str = "StdOptionalOptional";
pub const STRING: &str = "String";
pub const TEST_FUTURE_CONCURRENCY_D281414B: &str = "TEST_FUTURE_CONCURRENCY_D281414B";
pub const TRUNCATE_ADMIN_ACCESS_SESSIONS_ADMIN_REFRESH_TOKENS_ADMIN_LOGIN_ATTEMPTS_ADMIN_RATE:
    &str = "TRUNCATE admin_access_sessions, admin_refresh_tokens, admin_login_attempts, admin_rate_limits, admin_audit_log, pg_table_idempotency";
pub const TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS: &str = "TRUNCATE admin_rate_limits, admin_audit_log, admin_login_attempts, admin_access_sessions, admin_refresh_tokens, admin_user_roles, admin_users RESTART IDENTITY CASCADE";
pub const TRUNCATE_PG_TABLE_IDEMPOTENCY: &str = "TRUNCATE pg_table_idempotency";
pub const TRUNCATE_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST: &str =
    "TRUNCATE pg_table_idempotency_atomic_test";
pub const TABLEEXAMPLEREAD: &str = "TableExampleRead";
pub const TABLEEXAMPLEUPDATE: &str = "TableExampleUpdate";
pub const TCPLISTENER: &str = "TcpListener";
pub const TCPSTREAM: &str = "TcpStream";
pub const TOOLCOMMAND: &str = "ToolCommand";
pub const TRAIT: &str = "Trait";
pub const TRYFROM: &str = "TryFrom";
pub const TRYFROMSTRINGERROR: &str = "TryFromStringError";
pub const UPDATE: &str = "UPDATE ";
pub const UPDATE_ADMIN_ACCESS_SESSIONS_SET_REVOKED_AT_NOW_WHERE_USER_ID_DOLLAR: &str = "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_access_sessions WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)";
pub const UPDATE_ADMIN_REFRESH_TOKENS_SET_REVOKED_AT_NOW_WHERE_TOKEN_HASH_DOLLAR: &str = "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1 AND user_id = $2 AND revoked_at IS NULL";
pub const UPDATE_ADMIN_REFRESH_TOKENS_SET_REVOKED_AT_NOW_WHERE_USER_ID_DOLLAR: &str = "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)";
pub const UPDATE_ADMIN_ROLES_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_AND: &str =
    "UPDATE admin_roles SET name = $2 WHERE id = $1 AND is_system = FALSE RETURNING TRUE";
pub const UPDATE_ADMIN_SYSTEM_SETTINGS_SET_SITE_NAME_COALESCE_DOLLAR_1_SITE_NAME: &str = "UPDATE admin_system_settings SET site_name = COALESCE($1, site_name), tab_title = COALESCE($2, tab_title), main_logo = COALESCE($3, main_logo), primary_color = COALESCE($4, primary_color), default_admin_route = COALESCE($5, default_admin_route), organization_name = COALESCE($6, organization_name), organization_contacts = COALESCE($7, organization_contacts), support_url = COALESCE($8, support_url) WHERE id = 1 RETURNING TRUE";
pub const UPDATE_ADMIN_USERS_SET_IS_BANNED_DOLLAR_2_WHERE_ID_DOLLAR_1: &str =
    "UPDATE admin_users SET is_banned = $2 WHERE id = $1 RETURNING TRUE";
pub const UPDATE_ADMIN_USERS_SET_LOGIN_COALESCE_DOLLAR_2_LOGIN_DISPLAY_NAME_COALESCE: &str = "UPDATE admin_users SET login = COALESCE($2, login), display_name = COALESCE($3, display_name) WHERE id = $1 RETURNING TRUE";
pub const UPDATE_ADMIN_USERS_SET_PASSWORD_HASH_DOLLAR_2_WHERE_ID_DOLLAR_1: &str =
    "UPDATE admin_users SET password_hash = $2 WHERE id = $1 RETURNING TRUE";
pub const UPDATE_PG_TABLE_IDEMPOTENCY_SET_CREATED_AT_TIMESTAMPTZ_2000_01_01_00: &str = "UPDATE pg_table_idempotency SET created_at=TIMESTAMPTZ \'2000-01-01 00:00:00+00\',completed_at=CASE WHEN state=\'completed\' THEN TIMESTAMPTZ \'2000-01-01 00:00:00+00\' ELSE NULL END";
pub const UPDATE_PG_TABLE_OPTIMISTIC_REVISION_TEST_SET_VALUE_DOLLAR_1_REVISION_REVISION: &str = "UPDATE pg_table_optimistic_revision_test SET value=$1,revision=revision+1 WHERE id=1 AND revision=$2 RETURNING revision";
pub const UDPSOCKET: &str = "UdpSocket";
pub const UMERRORVARIANTS: &str = "UmErrorVariants";
pub const UNKNOWN_VERSION: &str = "Unknown version";
pub const UOERRORVARIANTS: &str = "UoErrorVariants";
pub const UPPERCAMELCASE: &str = "UpperCamelCase";
pub const USERS: &str = "Users";
pub const UTOIPAADMINAUTHOPENAPI: &str = "UtoipaAdminAuthOpenApi";
pub const UTOIPAADMINOPENAPI: &str = "UtoipaAdminOpenApi";
pub const UTOIPACOMMONROUTESOPENAPIDOCUMENT: &str = "UtoipaCommonRoutesOpenApiDocument";
pub const V: &str = "V";
pub const VEC: &str = "Vec";
pub const VEC_STRING: &str = "Vec<String>";
pub const VERSION_ALT: &str = "Version";
pub const WITH_ATTEMPT_AS_INSERT_INTO_ADMIN_LOGIN_ATTEMPTS_LOGIN_IP_ADDRESS_SUCCEEDED: &str = "WITH attempt AS (INSERT INTO admin_login_attempts (login, ip_address, succeeded) VALUES ($1, $2, $3)) INSERT INTO admin_audit_log (user_login, action, resource, resource_id, request_id, succeeded, details) SELECT $1, \'sign_in\', \'session\', $1, $4, FALSE, jsonb_build_object(\'ip_address\', $2::INET::TEXT) WHERE $3 = FALSE";
pub const WITH_EXPIRED_AS_SELECT_ACTOR_HTTP_METHOD_ROUTE_PATH_IDEMPOTENCY_KEY_FROM: &str = "WITH expired AS (SELECT actor,http_method,route_path,idempotency_key FROM pg_table_idempotency WHERE (state=\'completed\' AND completed_at < NOW() - make_interval(secs => $1)) OR (state=\'pending\' AND created_at < NOW() - make_interval(secs => $2)) ORDER BY created_at LIMIT $3) DELETE FROM pg_table_idempotency target USING expired WHERE target.actor=expired.actor AND target.http_method=expired.http_method AND target.route_path=expired.route_path AND target.idempotency_key=expired.idempotency_key";
pub const WITH_EXPIRED_AS_SELECT_ID_FROM_ADMIN_ACCESS_SESSIONS_WHERE_EXPIRES_AT: &str = "WITH expired AS (SELECT id FROM admin_access_sessions WHERE expires_at < NOW() OR (revoked_at IS NOT NULL AND revoked_at < NOW() - make_interval(secs => $1)) ORDER BY expires_at LIMIT $2) DELETE FROM admin_access_sessions target USING expired WHERE target.id=expired.id";
pub const WITH_EXPIRED_AS_SELECT_ID_FROM_ADMIN_AUDIT_LOG_WHERE_CREATED_AT: &str = "WITH expired AS (SELECT id FROM admin_audit_log WHERE created_at < NOW() - make_interval(secs => $1) ORDER BY created_at LIMIT $2) DELETE FROM admin_audit_log target USING expired WHERE target.id=expired.id";
pub const WITH_EXPIRED_AS_SELECT_ID_FROM_ADMIN_LOGIN_ATTEMPTS_WHERE_ATTEMPTED_AT: &str = "WITH expired AS (SELECT id FROM admin_login_attempts WHERE attempted_at < NOW() - make_interval(secs => $1) ORDER BY attempted_at LIMIT $2) DELETE FROM admin_login_attempts target USING expired WHERE target.id=expired.id";
pub const WITH_EXPIRED_AS_SELECT_ID_FROM_ADMIN_REFRESH_TOKENS_WHERE_EXPIRES_AT: &str = "WITH expired AS (SELECT id FROM admin_refresh_tokens WHERE expires_at < NOW() OR (revoked_at IS NOT NULL AND revoked_at < NOW() - make_interval(secs => $1)) ORDER BY expires_at LIMIT $2) DELETE FROM admin_refresh_tokens target USING expired WHERE target.id=expired.id";
pub const WITH_EXPIRED_AS_SELECT_SCOPE_SUBJECT_FROM_ADMIN_RATE_LIMITS_WHERE_WINDOW: &str = "WITH expired AS (SELECT scope,subject FROM admin_rate_limits WHERE window_started_at < NOW() - make_interval(secs => $1) ORDER BY window_started_at LIMIT $2) DELETE FROM admin_rate_limits target USING expired WHERE target.scope=expired.scope AND target.subject=expired.subject";
pub const X_CSRF_TOKEN: &str = "X-CSRF-Token";
pub const REDACTED_ALT_3: &str = "[REDACTED]";
pub const A_Z_PLUS: &str = "[a-z]+";
pub const DEPENDENCIES_NEWLINE_APP_STATE_WORKSPACE_TRUE_NEWLINE_AXUM_WORKSPACE_TRUE_NEWLINE_FUTURES: &str = "[dependencies]\napp_state = { workspace = true }\naxum = { workspace = true }\nfutures = { workspace = true }\nfrontend_contract = { workspace = true }\nhttp = { workspace = true }\nsqlx = { workspace = true }\nreqwest = { workspace = true }\nserde = { workspace = true }\nserde_json = { workspace = true }\nthiserror = { workspace = true }\nutoipa = { workspace = true }\ntracing = { workspace = true }\nwhere_filters = { workspace = true }\ngit_info = { workspace = true }\nlocation_lib = { workspace = true }\nlocation_macros = { workspace = true }\nmetrics = { workspace = true }\nlocation = { workspace = true }\npg_crud = { workspace = true, features = [\"test-utils\"] }\npg_crud_common = { workspace = true }\npg_table = { workspace = true }\npg_types_numeric = { workspace = true }\npg_types_text_misc = { workspace = true }\ngenerate_pg_table = { workspace = true }\noptml = { workspace = true }\nroute_validators = { workspace = true }\nserver_runtime = { workspace = true }\nto_err_string = { workspace = true }\n";
pub const DEPENDENCIES_NEWLINE_CHRONO_WORKSPACE_TRUE_NEWLINE_UUID_WORKSPACE_TRUE_NEWLINE_SQLX_WORKSPACE: &str = "[dependencies]\nchrono = { workspace = true }\nuuid = { workspace = true }\nsqlx = { workspace = true }\nserde = { workspace = true }\nserde_json = { workspace = true }\nfrontend_contract = { workspace = true }\nthiserror = { workspace = true }\nlocation_lib = { workspace = true }\nlocation_macros = { workspace = true }\nlocation = { workspace = true }\npg_crud_common = { workspace = true }\npg_types_common = { workspace = true }\nwhere_filters = { workspace = true }\noptml = { workspace = true }\nschemars = { workspace = true }\nto_err_string = { workspace = true }\nutoipa = { workspace = true }\n[features]\ntest-utils = []";
pub const DEPENDENCIES_NEWLINE_SQLX_WORKSPACE_TRUE_NEWLINE_SERDE_WORKSPACE_TRUE_NEWLINE_SCHEMARS_WORKSPACE: &str = "[dependencies]\nsqlx = { workspace = true }\nserde = { workspace = true }\nschemars = { workspace = true }\nutoipa = { workspace = true }\nlocation_lib = { workspace = true }\nlocation_macros = { workspace = true }\nlocation = { workspace = true }\npg_crud_common = { workspace = true }\nwhere_filters = { workspace = true }\nto_err_string = { workspace = true }\n[features]\ntest-utils = []";
pub const WORKSPACE_DEPENDENCIES: &str = "[workspace.dependencies]";
pub const WORKSPACE_NEWLINE_MEMBERS_OUTSIDE_NEWLINE: &str =
    "[workspace]\nmembers = [\"../outside\"]\n";
pub const WORKSPACE_NEWLINE_MEMBERS_SERVICE_NEWLINE: &str =
    "[workspace]\nmembers = [\"service\"]\n";
pub const A_ZA_Z0_9_PLUS_AS_A_ZA_Z0_9_PLUS: &str = "\"([A-Za-z0-9]+As[A-Za-z0-9]+)\"";
pub const VALUE_42_ALT: &str = "\"42\"";
pub const TEXT_ALT_12: &str = "\"\"";
pub const ABC_ALT: &str = "\"abc\"";
pub const ABCD: &str = "\"abcd\"";
pub const CORRECT_PASSWORD: &str = "\"correct-password\"";
pub const DIFFERENT_PASSWORD: &str = "\"different-password\"";
pub const B_0_9A_FA_F_8_0_9A_FA_F_4_4: &str =
    "\\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}\\b";
pub const D_PLUS: &str = "\\d+";
pub const NEWLINE: &str = "\n";
pub const NEWLINE_CARRIAGE_RETURN_TAB: &str = "\n\r\t";
pub const NEWLINE_CONST_SOURCE_TEXT_MAX_LEN_USIZE_1024_NEWLINE_DERIVE_NEWTYPE_PATH: &str = "\nconst SOURCE_TEXT_MAX_LEN: usize = 1024;\n#[derive(newtype::BoundedString)]\n#[bounded_string(max = SOURCE_TEXT_MAX_LEN)]\nstruct SourceText(String);\n";
pub const NEWLINE_FN_DIRECT_ARROW_STRING_NEWLINE_STRING_PATH_NEW_NEWLINE_NEWLINE_FN: &str = "\nfn direct() -> String {\n    String::new()\n}\nfn list() -> Vec<String> {\n    Vec::new()\n}\nfn optional() -> Option<&\'static str> {\n    None\n}\nstruct Helper;\nimpl Helper {\n    fn nested() -> Result<types::SourceText, String> {\n        Ok(types::SourceText::try_from(String::new()).expect(\"d3a1b7c9\"))\n    }\n    fn get(self) -> String {\n        String::new()\n    }\n}\nimpl AsRef<str> for Helper {\n    fn as_ref(&self) -> &str {\n        \"\"\n    }\n}\n";
pub const NEWLINE_STRUCT_HELPERSTATE_NEWLINE_NAMES_VEC_STRING_NEWLINE_SEEN_STD_PATH_COLLECTIONS:
    &str = "\nstruct HelperState {\n    names: Vec<String>,\n    seen: std::collections::BTreeSet<String>,\n    refs: Option<std::collections::HashSet<&\'static str>>,\n    wrapped: types::SourceTextList,\n}\nstruct SourceTextList(Vec<String>);\n";
pub const NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_FN_DEMO_NEWLINE_LET_PATH_CB: &str = "\nstruct SourceText(Box<str>);\nfn demo() {\n    let _path_cb = |path: std::path::PathBuf| path;\n    let _syn_cb = |value: syn::Type| value;\n    let _inferred_cb = |value| value;\n    let _wrapped_cb = |value: SourceText| value;\n}\n";
pub const NEWLINE_STRUCT_SOURCETEXT_BOX_STR_NEWLINE_IMPL_FROM_STRING_FOR_SOURCETEXT_NEWLINE: &str = "\nstruct SourceText(Box<str>);\nimpl From<String> for SourceText {\n    fn from(value: String) -> Self {\n        Self(value.into_boxed_str())\n    }\n}\n";
pub const S_ASTERISK_A_ZA_Z_A_ZA_Z0_9_ASTERISK_S_ASTERISK: &str =
    "^\\s*([A-Za-z][A-Za-z0-9_]*)\\s*,";
pub const UNDERSCORE: &str = "_";
pub const TABLE: &str = "_table";
pub const TEST: &str = "_test";
pub const A_ALT: &str = "a";
pub const A_PERCENT_B: &str = "a%_b";
pub const A164AEDD: &str = "a164aedd";
pub const A1A1382A: &str = "a1a1382a";
pub const A1D306DE: &str = "a1d306de";
pub const A2D6139E: &str = "a2d6139e";
pub const A2FCBAD4: &str = "a2fcbad4";
pub const A2FD8473: &str = "a2fd8473";
pub const A3040FA0: &str = "a3040fa0";
pub const A3A08AEB: &str = "a3a08aeb";
pub const A3D7F1C8: &str = "a3d7f1c8";
pub const A3E1F57C: &str = "a3e1f57c";
pub const A422E8D4: &str = "a422e8d4";
pub const A452843A: &str = "a452843a";
pub const A46F7336: &str = "a46f7336";
pub const A4D77F54: &str = "a4d77f54";
pub const A4E3B8D1: &str = "a4e3b8d1";
pub const A51F0D3B: &str = "a51f0d3b";
pub const A58F09DC: &str = "a58f09dc";
pub const A59D73C1: &str = "a59d73c1";
pub const A61329BF: &str = "a61329bf";
pub const A6413C9D: &str = "a6413c9d";
pub const A6D4F2C9: &str = "a6d4f2c9";
pub const A75BC224: &str = "a75bc224";
pub const A7F9C3E1: &str = "a7f9c3e1";
pub const A82438CC: &str = "a82438cc";
pub const A8E1C6F3: &str = "a8e1c6f3";
pub const A8F22481: &str = "a8f22481";
pub const A95D3C17: &str = "a95d3c17";
pub const A9651F69: &str = "a9651f69";
pub const A_PERCENT_B_PERCENT: &str = "a\\%\\_b%";
pub const AA12CD88: &str = "aa12cd88";
pub const AA7735DB: &str = "aa7735db";
pub const AA9FF040: &str = "aa9ff040";
pub const AARCH_64_SOFTFLOAT_NEON: &str = "aarch_64_softfloat_neon";
pub const AB: &str = "ab";
pub const AB892FC5: &str = "ab892fc5";
pub const ABC_ALT_3: &str = "abc";
pub const ABCC9A72: &str = "abcc9a72";
pub const ABCD_ALT: &str = "abcd";
pub const ABFD8FBC: &str = "abfd8fbc";
pub const ABORT_TRANSMUTE_POLICY_VIOLATIONS: &str = "abort/transmute policy violations:";
pub const AC15D6B9: &str = "ac15d6b9";
pub const ACCEPTED_202: &str = "accepted_202";
pub const ACCESS: &str = "access";
pub const ACCUMULATOR_9189F86E_PUSH: &str = "accumulator_9189f86e.push";
pub const ACTION: &str = "action";
pub const ACTOR_A: &str = "actor-a";
pub const ACTOR_ATOMIC: &str = "actor-atomic";
pub const ACTOR_B: &str = "actor-b";
pub const ACTOR_CONCURRENT: &str = "actor-concurrent";
pub const AD1DE295: &str = "ad1de295";
pub const ADF2B8C1: &str = "adf2b8c1";
pub const ADMIN_ALT: &str = "admin";
pub const ADMIN_CLIENT_1: &str = "admin-client/1";
pub const ADMIN_CLIENT_2: &str = "admin-client/2";
pub const ADMIN_USER_1: &str = "admin.user-1";
pub const ADMIN_ACCESS_TOKEN: &str = "admin_access_token=";
pub const ADMIN_ACCESS_TOKEN_INVALID_JWT_TOKEN: &str = "admin_access_token=invalid.jwt.token";
pub const ADMIN_COOKIE: &str = "admin_cookie";
pub const ADMIN_CSRF: &str = "admin_csrf";
pub const ADMIN_CSRF_TOKEN: &str = "admin_csrf_token";
pub const ADMIN_CSRF_TOKEN_ALT: &str = "admin_csrf_token=";
pub const ADMIN_MIGRATION_FRESH_TEST: &str = "admin_migration_fresh_test";
pub const ADMIN_MIGRATION_UPGRADE_TEST: &str = "admin_migration_upgrade_test";
pub const ADMIN_REFRESH_TOKEN: &str = "admin_refresh_token";
pub const ADMIN_REFRESH_TOKEN_ALT: &str = "admin_refresh_token=";
pub const ADMINISTRATOR_PASSWORD_LENGTH_IS_INVALID: &str =
    "administrator password length is invalid";
pub const ADMINISTRATOR_ROUTE_PATH_IS_TOO_LONG: &str = "administrator route path is too long";
pub const AE1262BB: &str = "ae1262bb";
pub const AE89C3BD: &str = "ae89c3bd";
pub const AE91F62C: &str = "ae91f62c";
pub const AEB6AD70: &str = "aeb6ad70";
pub const AEBF6DC8: &str = "aebf6dc8";
pub const AED15D30: &str = "aed15d30";
pub const AF066E8B: &str = "af066e8b";
pub const AF5A7EC4: &str = "af5a7ec4";
pub const AFE20C19: &str = "afe20c19";
pub const ALL_ALT: &str = "all";
pub const ALREADY_REPORTED_208: &str = "already_reported_208";
pub const AND_ALT: &str = "and ";
pub const AND_NOT: &str = "and not ";
pub const APPLICATION_JSON: &str = "application/json";
pub const APPLICATION_PROBLEM_PLUS_JSON: &str = "application/problem+json";
pub const AQUASECURITY_TRIVY_ACTION: &str = "aquasecurity/trivy-action@";
pub const ARG_IS_NOT_STRING_LITERAL: &str = "arg is not string literal";
pub const ARGUMENTS: &str = "arguments";
pub const ARIA_LABEL_FILTER_ROWS: &str = "aria-label=\"Filter rows\"";
pub const ARIA_LABEL_NEXT_PAGE: &str = "aria-label=\"Next page\"";
pub const ARIA_LABEL_PREVIOUS_PAGE: &str = "aria-label=\"Previous page\"";
pub const ARIA_LABEL_ROWS_PER_PAGE: &str = "aria-label=\"Rows per page\"";
pub const ARIA_LABEL_SORT_FIELD: &str = "aria-label=\"Sort field\"";
pub const ARIA_LABEL_TOGGLE_SORT_DIRECTION: &str = "aria-label=\"Toggle sort direction\"";
pub const AS_REF: &str = "as_ref";
pub const AS_REF_INNER: &str = "as_ref_inner";
pub const AS_REF_OWNED: &str = "as_ref_owned";
pub const AS_REF_STR: &str = "as_ref_str";
pub const AS_REF_TARGET: &str = "as_ref_target";
pub const AS_SLICE: &str = "as_slice";
pub const ASYNC_FUNCTIONS_CONTAIN_BLOCKING_EXECUTOR_CALLS: &str =
    "async functions contain blocking executor calls:";
pub const ASYNC: &str = "async";
pub const AUDIT_LOG_ALT: &str = "audit_log";
pub const AUTHENTICATION_FAILED: &str = "authentication failed";
pub const AUTHENTICATION_REFRESH_REJECTED: &str = "authentication refresh rejected";
pub const AUTHENTICATION_REFRESH_RETRY_IS_DELAYED: &str = "authentication refresh retry is delayed";
pub const AUTHENTICATION_REFRESH_STATE_IS_UNAVAILABLE: &str =
    "authentication refresh state is unavailable";
pub const AUTHENTICATION_REQUIRED: &str = "authentication required";
pub const AUTHORIZATION_FAILED: &str = "authorization failed";
pub const B: &str = "b";
pub const B_ALT: &str = "b\'\'";
pub const B_A: &str = "b\'a\'";
pub const B_ABC: &str = "b\'abc\'";
pub const B048535E: &str = "b048535e";
pub const B1BA49CC: &str = "b1ba49cc";
pub const B2604D91: &str = "b2604d91";
pub const B26F4A08: &str = "b26f4a08";
pub const B319E84D: &str = "b319e84d";
pub const B3A7C1E4: &str = "b3a7c1e4";
pub const B41052BC: &str = "b41052bc";
pub const B482B167: &str = "b482b167";
pub const B4E7C2A9: &str = "b4e7c2a9";
pub const B67815EC: &str = "b67815ec";
pub const B6B47A2C: &str = "b6b47a2c";
pub const B6DBA95D: &str = "b6dba95d";
pub const B6E2A9F4: &str = "b6e2a9f4";
pub const B7C2E5F8: &str = "b7c2e5f8";
pub const B7C84E2A: &str = "b7c84e2a";
pub const B871BD8F_7810_4D4B_94A1_5458D3016907: &str = "b871bd8f-7810-4d4b-94a1-5458d3016907";
pub const B8C71E43: &str = "b8c71e43";
pub const B8F8EAF1: &str = "b8f8eaf1";
pub const B93D2A8C: &str = "b93d2a8c";
pub const B9A203E6: &str = "b9a203e6";
pub const B9DA972A: &str = "b9da972a";
pub const B_42: &str = "b\"42\"";
pub const B_ALT_3: &str = "b\"\"";
pub const B_ABC_ALT: &str = "b\"abc\"";
pub const BACKGROUND_TASK_SHUTDOWN_TIMED_OUT: &str = "background task shutdown timed out";
pub const BAD: &str = "bad";
pub const BAD_GATEWAY_502: &str = "bad_gateway_502";
pub const BAD_REQ_400: &str = "bad_req_400";
pub const BB258755: &str = "bb258755";
pub const BB6C239E: &str = "bb6c239e";
pub const UNSUPPORTED_GENERATE_PG_TABLE_FRONTEND_OPTION: &str =
    "bc1d3b08: unsupported generate_pg_table_frontend option";
pub const BD9180CA: &str = "bd9180ca";
pub const BD9F5208: &str = "bd9f5208";
pub const BEB11586: &str = "beb11586";
pub const BENCHES: &str = "benches";
pub const BENCHMARK_TABLE: &str = "benchmark_table";
pub const BF0D6F55: &str = "bf0d6f55";
pub const BF2E4A7C: &str = "bf2e4a7c";
pub const BF4BCC30: &str = "bf4bcc30";
pub const BFCD929A: &str = "bfcd929a";
pub const BIND: &str = "bind";
pub const BLOCK_IN_PLACE: &str = "block_in_place";
pub const BLOCK_ON: &str = "block_on";
pub const BLOCKING_CALL_INSIDE_ASYNC_FUNCTION: &str = "blocking call inside async function";
pub const BOOL_PARSE: &str = "bool parse";
pub const BOOL_ENUM_TO_TOKENS: &str = "bool_enum_to_tokens";
pub const BOUNDED_STRING: &str = "bounded_string";
pub const BUILD_DEPENDENCIES: &str = "build-dependencies";
pub const BUILD_GENERATE_PG_TABLE_INPUT_MODEL_STAGE: &str =
    "build_generate_pg_table_input_model_stage";
pub const C02AE58B: &str = "c02ae58b";
pub const C0745B58: &str = "c0745b58";
pub const C0E03C6D: &str = "c0e03c6d";
pub const C19BE784: &str = "c19be784";
pub const C19F58A4: &str = "c19f58a4";
pub const C1D4F7A2: &str = "c1d4f7a2";
pub const C1D74A8E: &str = "c1d74a8e";
pub const C245193E: &str = "c245193e";
pub const C3AF0891: &str = "c3af0891";
pub const C3AF72F5: &str = "c3af72f5";
pub const C4E9A2D7: &str = "c4e9a2d7";
pub const C52D0E93: &str = "c52d0e93";
pub const C563853A: &str = "c563853a";
pub const C5C45332: &str = "c5c45332";
pub const C5D09740: &str = "c5d09740";
pub const C5D0BF17: &str = "c5d0bf17";
pub const C5F103DA: &str = "c5f103da";
pub const C6E4F7A1: &str = "c6e4f7a1";
pub const C6FD2BC8: &str = "c6fd2bc8";
pub const C71F2A8D: &str = "c71f2a8d";
pub const C7685B19: &str = "c7685b19";
pub const C81A6F20: &str = "c81a6f20";
pub const C836AD25: &str = "c836ad25";
pub const C84E9D1F: &str = "c84e9d1f";
pub const C86A4310: &str = "c86a4310";
pub const C89F19A5: &str = "c89f19a5";
pub const C8B3565C: &str = "c8b3565c";
pub const C8D2F1A3: &str = "c8d2f1a3";
pub const C90CBA14: &str = "c90cba14";
pub const C95E27D1: &str = "c95e27d1";
pub const C9711EFD: &str = "c9711efd";
pub const C9D73CAB: &str = "c9d73cab";
pub const CABD480A: &str = "cabd480a";
pub const CAE226CD: &str = "cae226cd";
pub const CAFEBABE: &str = "cafebabe";
pub const CALLOC: &str = "calloc|";
pub const CANT_SUPPORT_NULLABLE_VARIANTS: &str = "cant support nullable variants: ";
pub const CARGO_PLUS_NIGHTLY_UDEPS_WORKSPACE_ALL_TARGETS_ALL_FEATURES_LOCKED: &str =
    "cargo +nightly udeps --workspace --all-targets --all-features --locked";
pub const CARGO_LLVM_COV_WORKSPACE_ALL_FEATURES_ALL_TARGETS_SUMMARY_ONLY: &str =
    "cargo llvm-cov --workspace --all-features --all-targets --summary-only";
pub const CARGO_MACHETE: &str = "cargo machete";
pub const CB6830BC: &str = "cb6830bc";
pub const CB693A3F: &str = "cb693a3f";
pub const CBA1B5FB: &str = "cba1b5fb";
pub const CBBF6ACF: &str = "cbbf6acf";
pub const CC0E9FF2: &str = "cc0e9ff2";
pub const CC0F2F3E: &str = "cc0f2f3e";
pub const CC4670A2: &str = "cc4670a2";
pub const CD596C44: &str = "cd596c44";
pub const CD734995: &str = "cd734995";
pub const CE417390: &str = "ce417390";
pub const CE4826F4: &str = "ce4826f4";
pub const CFG_ALT: &str = "cfg";
pub const CHARS: &str = "chars";
pub const CLEANUP_BATCH_SIZE_MUST_BE_BETWEEN_1_AND_10000: &str =
    "cleanup batch size must be between 1 and 10000";
pub const CLEANUP_RETENTION_MUST_BE_GREATER_THAN_ZERO: &str =
    "cleanup retention must be greater than zero";
pub const CLIENT_ADDRESS: &str = "client-address=";
pub const CLIPPY: &str = "clippy";
pub const CLIPPY_DRIVER: &str = "clippy-driver";
pub const CLOSURE_PARAMETER: &str = "closure parameter";
pub const CM: &str = "cm";
pub const CO: &str = "co";
pub const CODE_STYLE: &str = "code_style";
pub const COLUMN: &str = "column";
pub const COLUMN_1: &str = "column_1";
pub const COMPILATION: &str = "compilation";
pub const COMPILE_ERROR_TOKEN_STREAM_CALL_CONTAINS_STRING_LITERALS: &str =
    "compile_error_token_stream call contains string literals";
pub const COMPONENTS: &str = "components";
pub const CONFIG_LIB_MACROS: &str = "config_lib_macros";
pub const CONFLICT: &str = "conflict";
pub const CONFLICT_409: &str = "conflict_409";
pub const CONNECT: &str = "connect";
pub const CONTAINS_FOR_LOOP_USE_ITERATOR_METHODS_INSTEAD: &str =
    "contains `for` loop; use iterator methods instead";
pub const CONTAINS_TODO: &str = "contains todo!()";
pub const CONTAINS_UNIMPLEMENTED: &str = "contains unimplemented!()";
pub const CONTINUE_100: &str = "continue_100";
pub const CORRECT_PASSWORD_ALT: &str = "correct password";
pub const CRATE: &str = "crate";
pub const CRATE_DIR: &str = "crate_dir";
pub const CREATED_201: &str = "created_201";
pub const CREATED_AT: &str = "created_at";
pub const CSRF: &str = "csrf";
pub const D02BA9F0: &str = "d02ba9f0";
pub const DUPLICATE_SORTABLE_OPTION: &str = "d1b677d4: duplicate sortable option";
pub const D1F5B9C7: &str = "d1f5b9c7";
pub const D293F783: &str = "d293f783";
pub const D2A8C4E1: &str = "d2a8c4e1";
pub const D2B9CC45: &str = "d2b9cc45";
pub const D2F3B74A: &str = "d2f3b74a";
pub const D34A7BC1: &str = "d34a7bc1";
pub const PRIMARY_KEY_TYPE_MUST_BE_NON_NULLABLE: &str =
    "d3b03ca2: primary key type must be non-nullable";
pub const D53D8FF0: &str = "d53d8ff0";
pub const D58ED6A5: &str = "d58ed6a5";
pub const D5A0693B: &str = "d5a0693b";
pub const D5B2B269: &str = "d5b2b269";
pub const D5EC6712: &str = "d5ec6712";
pub const D5F1A4E7: &str = "d5f1a4e7";
pub const D6288F19_0A24_42AD_9E69_36036D9F2C1D: &str = "d6288f19-0a24-42ad-9e69-36036d9f2c1d";
pub const D6619712: &str = "d6619712";
pub const FRONTEND_LABEL_MUST_NOT_BE_EMPTY: &str = "d78d2e63: frontend label must not be empty";
pub const D7A3C5B1: &str = "d7a3c5b1";
pub const D7A590E3: &str = "d7a590e3";
pub const D7E1862C: &str = "d7e1862c";
pub const D80FC31B: &str = "d80fc31b";
pub const D81F6A42: &str = "d81f6a42";
pub const D86085DB: &str = "d86085db";
pub const D870B82E: &str = "d870b82e";
pub const D8A26635_C478_4A2A_ACF4_BF1765702889: &str = "d8a26635-c478-4a2a-acf4-bf1765702889";
pub const D9154402: &str = "d9154402";
pub const D93BEB69: &str = "d93beb69";
pub const D94F091A: &str = "d94f091a";
pub const DA271038: &str = "da271038";
pub const DA504E54: &str = "da504e54";
pub const DATABASE: &str = "database";
pub const DATE_NAIVE: &str = "date_naive";
pub const DB05C4BE: &str = "db05c4be";
pub const DB75B4FB: &str = "db75b4fb";
pub const DBA097B9: &str = "dba097b9";
pub const DBD02F72: &str = "dbd02f72";
pub const DBE97EF3: &str = "dbe97ef3";
pub const DBG_FOUND: &str = "dbg!() found:";
pub const DBG: &str = "dbg";
pub const DC191318: &str = "dc191318";
pub const DC39BA13: &str = "dc39ba13";
pub const DCB22948: &str = "dcb22948";
pub const DDF0983A: &str = "ddf0983a";
pub const DE729A31: &str = "de729a31";
pub const DE790942: &str = "de790942";
pub const DEA5CBCF: &str = "dea5cbcf";
pub const DEBUG_TRANSPARENT: &str = "debug_transparent";
pub const DEFAULT_FEATURES: &str = "default-features";
pub const DEFAULT_OVERRIDES_DEFAULT_FIELDS: &str = "default_overrides_default_fields";
pub const DELETE_FROM: &str = "delete from ";
pub const DELETE_FROM_USERS_WHERE_ID_DOLLAR_1_RETURNING_ID: &str =
    "delete from users where id = $1 returning id";
pub const DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE: &str =
    "delete from users where id in ($1,$2) and active = true returning id";
pub const DELETE_FROM_USERS_WHERE_ID_IN_DOLLAR_1_DOLLAR_2_RETURNING_ID: &str =
    "delete from users where id in ($1,$2) returning id";
pub const DEPENDENCIES: &str = "dependencies";
pub const DEPRECATED_LLVM_INTRINSIC: &str = "deprecated_llvm_intrinsic";
pub const DEREF: &str = "deref";
pub const DEREF_INNER_AND_DEREF_TARGET_CANNOT_BE_COMBINED: &str =
    "deref_inner and deref_target cannot be combined";
pub const DEREF_INNER: &str = "deref_inner";
pub const DEREF_MUT_INNER_REQUIRES_DEREF_INNER: &str = "deref_mut_inner requires deref_inner";
pub const DEREF_MUT_INNER: &str = "deref_mut_inner";
pub const DEREF_MUT_TARGET_REQUIRES_DEREF_TARGET: &str = "deref_mut_target requires deref_target";
pub const DEREF_MUT_TARGET: &str = "deref_mut_target";
pub const DEREF_TARGET: &str = "deref_target";
pub const DERIVE: &str = "derive";
pub const DESCRIPTION: &str = "description";
pub const DEV_DEPENDENCIES: &str = "dev-dependencies";
pub const DF43C793: &str = "df43c793";
pub const DF91B04D: &str = "df91b04d";
pub const DFF79E9D: &str = "dff79e9d";
pub const DIRECT_COMMAND_PATH_NEW_USAGE_EXISTS_OUTSIDE_MACROS_HELPERS_PATH_TOOL_COMMAND: &str =
    "direct Command::new usage exists outside macros_helpers::tool_command:";
pub const DIRECT_ENVIRONMENT_OR_FILESYSTEM_ACCESS_EXISTS_OUTSIDE_APPROVED_CONFIGURATION_TOOLING_TEST_AND: &str = "direct environment or filesystem access exists outside approved configuration, tooling, test, and persistence boundaries:";
pub const DISPLAY: &str = "display";
pub const DISPLAY_NAME: &str = "display_name";
pub const DLO: &str = "dlo";
pub const DM: &str = "dm";
pub const DOTTED_WORKSPACE_DEPENDENCY_STYLE_FOUND: &str =
    "dotted workspace dependency style found:";
pub const DUPLICATE_NEWTYPE_OPTION: &str = "duplicate newtype option";
pub const DUPLICATE: &str = "duplicate";
pub const DUPLICATE_A: &str = "duplicate-a";
pub const DUPLICATE_B: &str = "duplicate-b";
pub const DUPLICATE_FEATURES: &str = "duplicate_features";
pub const DUPLICATED_STRING_LITERALS_FOUND_IN_NON_POLICY_TEST_CODE: &str =
    "duplicated string literals found in non-policy test code:";
pub const E098A1FF: &str = "e098a1ff";
pub const E0C9257D: &str = "e0c9257d";
pub const E117FA5A: &str = "e117fa5a";
pub const E1394CD0: &str = "e1394cd0";
pub const E1B22572: &str = "e1b22572";
pub const E1C2D84A: &str = "e1c2d84a";
pub const E1D07F53: &str = "e1d07f53";
pub const E28698F2: &str = "e28698f2";
pub const E2A6B9C4: &str = "e2a6b9c4";
pub const E2C94D67: &str = "e2c94d67";
pub const E2D99B73: &str = "e2d99b73";
pub const E3E42AA5: &str = "e3e42aa5";
pub const E3F8A1C5: &str = "e3f8a1c5";
pub const E411F376: &str = "e411f376";
pub const E45F75C2: &str = "e45f75c2";
pub const E5C23C45: &str = "e5c23c45";
pub const E5E1F7CB: &str = "e5e1f7cb";
pub const E6175D82: &str = "e6175d82";
pub const E6640036: &str = "e6640036";
pub const E7150F4C: &str = "e7150f4c";
pub const E76640C4: &str = "e76640c4";
pub const E7A3D5C1: &str = "e7a3d5c1";
pub const E7D5F988: &str = "e7d5f988";
pub const E8714250: &str = "e8714250";
pub const E8B3A6D2: &str = "e8b3a6d2";
pub const E97B25B9: &str = "e97b25b9";
pub const EB08DFFC: &str = "eb08dffc";
pub const EB24448C: &str = "eb24448c";
pub const EBF4E1B2: &str = "ebf4e1b2";
pub const EC1E77D5: &str = "ec1e77d5";
pub const ED2F56FB: &str = "ed2f56fb";
pub const ED8BC4D0: &str = "ed8bc4d0";
pub const EDABBC24: &str = "edabbc24";
pub const EDITION_2024_NEWLINE: &str = "edition = \"2024\"\n";
pub const EDITION: &str = "edition";
pub const EF71E50A: &str = "ef71e50a";
pub const EMIT_GENERATE_PG_TABLE_FINAL_STAGE: &str = "emit_generate_pg_table_final_stage";
pub const EMIT_GENERATE_PG_TABLE_TESTS_STAGE: &str = "emit_generate_pg_table_tests_stage";
pub const ENUM: &str = "enum";
pub const EO_HASHMAP_K_STRING_V_LOCATION: &str = "eo_hashmap_k_string_v_location";
pub const EO_HASHMAP_K_STRING_V_TO_ERR_STRING: &str = "eo_hashmap_k_string_v_to_err_string";
pub const EO_HASHMAP_K_STRING_V_TO_ERR_STRING_SERDE: &str =
    "eo_hashmap_k_string_v_to_err_string_serde";
pub const EO_LOCATION: &str = "eo_location";
pub const EO_TO_ERR_STRING: &str = "eo_to_err_string";
pub const EO_TO_ERR_STRING_SERDE: &str = "eo_to_err_string_serde";
pub const EO_VEC_LOCATION: &str = "eo_vec_location";
pub const EO_VEC_TO_ERR_STRING: &str = "eo_vec_to_err_string";
pub const EO_VEC_TO_ERR_STRING_SERDE: &str = "eo_vec_to_err_string_serde";
pub const ERRORS_WITH_LOCATION_DOES_NOT_ACCEPT_ARGUMENTS: &str =
    "errors_with_location does not accept arguments";
pub const ERRORS_WITH_LOCATION_SUPPORTS_ONLY_VARIANTS_WITH_NAMED_FIELDS: &str =
    "errors_with_location supports only variants with named fields";
pub const ERRORS_WITH_LOCATION_VARIANT_ALREADY_HAS_A_LOCATION_FIELD: &str =
    "errors_with_location variant already has a location field";
pub const EXAMPLE: &str = "example";
pub const EXECUTOR: &str = "executor";
pub const EXISTING_REQUEST_ID: &str = "existing-request-id";
pub const EXPECT_ERROR: &str = "expect_error";
pub const EXPECT_OK: &str = "expect_ok";
pub const EXPECTATION_FAILED_417: &str = "expectation_failed_417";
pub const F00DBABE: &str = "f00dbabe";
pub const F11E0324: &str = "f11e0324";
pub const F133A4CA: &str = "f133a4ca";
pub const F170AA14: &str = "f170aa14";
pub const F1A92B49: &str = "f1a92b49";
pub const F1C7A4E3: &str = "f1c7a4e3";
pub const F20C4A91: &str = "f20c4a91";
pub const F24FCA72: &str = "f24fca72";
pub const F29CC79A: &str = "f29cc79a";
pub const F2A8C5D3: &str = "f2a8c5d3";
pub const F2C7A91B: &str = "f2c7a91b";
pub const F2CC7D6B: &str = "f2cc7d6b";
pub const F341CDE7: &str = "f341cde7";
pub const F37A3AB4: &str = "f37a3ab4";
pub const F39BDCC6: &str = "f39bdcc6";
pub const F39C05AA: &str = "f39c05aa";
pub const F3B5A711: &str = "f3b5a711";
pub const F3D821A6: &str = "f3d821a6";
pub const F459312E: &str = "f459312e";
pub const F4C1D7A9_SAME_LEN_HELPER_REQUIRES_EQUAL_LENGTHS: &str =
    "f4c1d7a9 same-len helper requires equal lengths";
pub const F4C2A9E1: &str = "f4c2a9e1";
pub const F4CAB210: &str = "f4cab210";
pub const F4E61B29: &str = "f4e61b29";
pub const F50EF817: &str = "f50ef817";
pub const F542A3CB: &str = "f542a3cb";
pub const F5C41DD8: &str = "f5c41dd8";
pub const F5D2CB68: &str = "f5d2cb68";
pub const F60721A2: &str = "f60721a2";
pub const F66647AB: &str = "f66647ab";
pub const F68E33F3: &str = "f68e33f3";
pub const F698FD6D: &str = "f698fd6d";
pub const F6A51733: &str = "f6a51733";
pub const F6F6FB24: &str = "f6f6fb24";
pub const F728192D: &str = "f728192d";
pub const F771AC2D: &str = "f771ac2d";
pub const F797718F: &str = "f797718f";
pub const F7C0E2A9: &str = "f7c0e2a9";
pub const F7D8C961: &str = "f7d8c961";
pub const F83D470A_GENERATED_FILE_COMPARISON_READ_LENGTH_EXCEEDS_BUFFER: &str =
    "f83d470a generated file comparison read length exceeds buffer";
pub const F87F82B6: &str = "f87f82b6";
pub const F96BCC6E: &str = "f96bcc6e";
pub const F9B0CD83: &str = "f9b0cd83";
pub const F9C2D4A8: &str = "f9c2d4a8";
pub const F9F9AF71: &str = "f9f9af71";
pub const FAC2138B: &str = "fac2138b";
pub const FAILED_DEPENDENCY_424: &str = "failed_dependency_424";
pub const FALSE_FAT_ARROW: &str = "false =>";
pub const FALSE: &str = "false";
pub const FB5AEE1D: &str = "fb5aee1d";
pub const FBF14346: &str = "fbf14346";
pub const FC65B7C4: &str = "fc65b7c4";
pub const FCBA80E1: &str = "fcba80e1";
pub const FCD3DD3F: &str = "fcd3dd3f";
pub const FD5E40C9: &str = "fd5e40c9";
pub const FD6A65B0: &str = "fd6a65b0";
pub const FD9F7861: &str = "fd9f7861";
pub const FDBF7411: &str = "fdbf7411";
pub const FE53A6B9_2D7E_4605_9F5A_7F5C21CC01E6: &str = "fe53a6b9-2d7e-4605-9f5a-7f5c21cc01e6";
pub const FE54B186: &str = "fe54b186";
pub const FE89C42A: &str = "fe89c42a";
pub const FEAD1583: &str = "fead1583";
pub const FEATURE: &str = "feature";
pub const FEATURES: &str = "features = ";
pub const FEATURES_ALT: &str = "features";
pub const FILTERABLE: &str = "filterable";
pub const FIRST_ALT: &str = "first";
pub const FIXED_TEST_TOKEN: &str = "fixed-test-token";
pub const FOR_LOOPS_FOUND_USE_ITERATOR_METHODS_SUCH_AS_MAP_FILTER_FOLD_TRY: &str = "for loops found; use iterator methods such as `map`, `filter`, `fold`, `try_fold`, `for_each`, or `try_for_each` instead:";
pub const FORBIDDEN_403: &str = "forbidden_403";
pub const FORMATTING: &str = "formatting";
pub const FOUND_302: &str = "found_302";
pub const FREE: &str = "free|";
pub const FROM_ALT_4: &str = "from";
pub const FROM_INNER: &str = "from_inner";
pub const FRONTEND_CONTRACT_SRC_LIB_RS: &str = "frontend_contract/src/lib.rs";
pub const FUTURES: &str = "futures";
pub const FUZZY_PROVENANCE_CASTS: &str = "fuzzy_provenance_casts";
pub const GATEWAY_TIMEOUT_504: &str = "gateway_timeout_504";
pub const GENERATE_PG_TABLE: &str = "generate_pg_table";
pub const GENERATE_PG_TABLE_PATH_CM_ERROR_VARIANTS: &str = "generate_pg_table::cm_error_variants";
pub const GENERATE_PG_TABLE_PATH_CM_LOGIC: &str = "generate_pg_table::cm_logic";
pub const GENERATE_PG_TABLE_PATH_CO_ERROR_VARIANTS: &str = "generate_pg_table::co_error_variants";
pub const GENERATE_PG_TABLE_PATH_CO_LOGIC: &str = "generate_pg_table::co_logic";
pub const GENERATE_PG_TABLE_PATH_COMMON_ERROR_VARIANTS: &str =
    "generate_pg_table::common_error_variants";
pub const GENERATE_PG_TABLE_PATH_COMMON_LOGIC: &str = "generate_pg_table::common_logic";
pub const GENERATE_PG_TABLE_PATH_DLO_ERROR_VARIANTS: &str = "generate_pg_table::dlo_error_variants";
pub const GENERATE_PG_TABLE_PATH_DLO_LOGIC: &str = "generate_pg_table::dlo_logic";
pub const GENERATE_PG_TABLE_PATH_DM_ERROR_VARIANTS: &str = "generate_pg_table::dm_error_variants";
pub const GENERATE_PG_TABLE_PATH_DM_LOGIC: &str = "generate_pg_table::dm_logic";
pub const GENERATE_PG_TABLE_PATH_RM_ERROR_VARIANTS: &str = "generate_pg_table::rm_error_variants";
pub const GENERATE_PG_TABLE_PATH_RM_LOGIC: &str = "generate_pg_table::rm_logic";
pub const GENERATE_PG_TABLE_PATH_RO_ERROR_VARIANTS: &str = "generate_pg_table::ro_error_variants";
pub const GENERATE_PG_TABLE_PATH_RO_LOGIC: &str = "generate_pg_table::ro_logic";
pub const GENERATE_PG_TABLE_PATH_UM_ERROR_VARIANTS: &str = "generate_pg_table::um_error_variants";
pub const GENERATE_PG_TABLE_PATH_UM_LOGIC: &str = "generate_pg_table::um_logic";
pub const GENERATE_PG_TABLE_PATH_UO_ERROR_VARIANTS: &str = "generate_pg_table::uo_error_variants";
pub const GENERATE_PG_TABLE_PATH_UO_LOGIC: &str = "generate_pg_table::uo_logic";
pub const GENERATE_PG_TABLE_TESTS: &str = "generate_pg_table_Tests";
pub const GENERATE_PG_TABLE_TESTS_RS: &str = "generate_pg_table_Tests.rs";
pub const GENERATE_PG_TABLE_COMMON: &str = "generate_pg_table_common";
pub const GENERATE_PG_TABLE_FRONTEND: &str = "generate_pg_table_frontend";
pub const GENERATE_PG_TABLE_PRIMARY_KEY: &str = "generate_pg_table_primary_key";
pub const GENERATE_PG_TABLE_SRC: &str = "generate_pg_table_src";
pub const GENERATE_PG_TABLE_TEST_CNT: &str = "generate_pg_table_test_cnt";
pub const GENERATE_PG_TYPES_SRC: &str = "generate_pg_types_src";
pub const GENERATE_PG_TYPES_TEST_CNT: &str = "generate_pg_types_test_cnt";
pub const GENERATE_WHERE_FILTERS_PG_TYPES: &str = "generate_where_filters_pg_types";
pub const GENERATE_WHERE_FLTS_TEST_CNT: &str = "generate_where_flts_test_cnt";
pub const GET_ALT: &str = "get";
pub const GET_MACRO_ATTR_RS: &str = "get_macro_attr.rs";
pub const GETTER: &str = "getter";
pub const GONE_410: &str = "gone_410";
pub const GROWTH: &str = "growth";
pub const HEAP_PEAK: &str = "heap peak:";
pub const HEAP_TOTAL: &str = "heap total:";
pub const HEAVY_LOAD: &str = "heavy-load";
pub const HELLOWORLD_ALT: &str = "helloWorld";
pub const HELLO_WORLD_ALT: &str = "hello_world";
pub const HELP: &str = "help";
pub const HIDDEN: &str = "hidden";
pub const HTTP: &str = "http";
pub const HTTP_BLOCKED_EXAMPLE: &str = "http://blocked.example";
pub const HTTP_LOCALHOST: &str = "http://localhost";
pub const HTTP_LOCALHOST_ADMIN_SIGN_IN: &str = "http://localhost/admin/sign-in";
pub const HTTP_VERSION_NOT_SUPPORTED_505: &str = "http_version_not_supported_505";
pub const HTTPS: &str = "https";
pub const HTTPS_ADMIN_EXAMPLE_COM: &str = "https://admin.example.com";
pub const ID_DOLLAR_1: &str = "id = $1";
pub const ID_NAME: &str = "id,name";
pub const ID_REVISION: &str = "id,revision";
pub const IDEMPOTENCY_METHOD_MUST_BE_POST_PATCH_OR_DELETE: &str =
    "idempotency method must be POST, PATCH, or DELETE";
pub const IDEMPOTENCY_RESERVATION_IS_UNAVAILABLE_FOR_COMPLETION: &str =
    "idempotency reservation is unavailable for completion";
pub const IDEMPOTENCY_RESPONSE_EXCEEDS_THE_STORAGE_LIMIT: &str =
    "idempotency response exceeds the storage limit";
pub const IDEMPOTENCY_RESPONSE_STATUS_IS_OUTSIDE_SMALLINT: &str =
    "idempotency response status is outside SMALLINT";
pub const IDEMPOTENCY_ROUTE_MUST_START_WITH_A_SLASH: &str =
    "idempotency route must start with a slash";
pub const IDEMPOTENCY_TEXT_MUST_NOT_BE_EMPTY: &str = "idempotency text must not be empty";
pub const IDEMPOTENCY_KEY_ALT: &str = "idempotency-key";
pub const IF_MATCH_ALT: &str = "if-match";
pub const IM_A_TEAPOT_418: &str = "im_a_teapot_418";
pub const IM_USED_226: &str = "im_used_226";
pub const IMPL_TRY_FROM_NON_EMPTY_STRING: &str = "impl_try_from_non_empty_string";
pub const IMPL_TRY_FROM_PARSE: &str = "impl_try_from_parse";
pub const IMPL_TRY_FROM_PARSE_STRING_ERROR: &str = "impl_try_from_parse_string_error";
pub const IMPL_TRY_FROM_SECRET_URL: &str = "impl_try_from_secret_url";
pub const IMPLICIT_PROVENANCE_CASTS: &str = "implicit_provenance_casts";
pub const INCLUDE_BYTES: &str = "include_bytes";
pub const INCLUDE_STR_OR_INCLUDE_BYTES_FOUND_OUTSIDE_EXPLICIT_GENERATED_TEST_FIXTURE_ALLOWLIST:
    &str =
    "include_str!() or include_bytes!() found outside explicit generated/test fixture allowlist:";
pub const INCLUDE_STR: &str = "include_str";
pub const INSERT_INTO: &str = "insert into ";
pub const INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_RETURNING_ID: &str =
    "insert into users (id,name) values ($1,$2) returning id";
pub const INSERT_INTO_USERS_ID_NAME_VALUES_DOLLAR_1_DOLLAR_2_DOLLAR_3: &str =
    "insert into users (id,name) values ($1,$2),($3,$4) returning id";
pub const INSUFFICIENT_STORAGE_507: &str = "insufficient_storage_507";
pub const INTEGRATION_TEST: &str = "integration-test";
pub const INTEGRATION_TEST_ADMIN: &str = "integration-test-admin";
pub const INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES: &str =
    "integration-test-jwt-secret-at-least-32-bytes";
pub const INTENTIONAL_SERIALIZATION_FAILURE: &str = "intentional serialization failure";
pub const INTERNAL_ERROR: &str = "internal error";
pub const INTERNAL_SERVER_ERROR: &str = "internal server error";
pub const INTERNAL_SERVER_ERROR_500: &str = "internal_server_error_500";
pub const INTO_INNER: &str = "into_inner";
pub const INTO_INNER_FROM: &str = "into_inner_from";
pub const INTO_VEC: &str = "into_vec";
pub const INVALID_REQUEST: &str = "invalid request";
pub const IS_NULL: &str = "is null";
pub const IS_BANNED: &str = "is_banned";
pub const JOBS_NEWLINE: &str = "jobs:\n";
pub const KDFGSDFGDSFGEY: &str = "kdfgsdfgdsfgey";
pub const KESDFGSDGFDFGY: &str = "kesdfgsdgfdfgy";
pub const KESDFSFDSFSD: &str = "kesdfsfdsfsd";
pub const KEY_A: &str = "key-a";
pub const KEY_ATOMIC: &str = "key-atomic";
pub const KEY_CONCURRENT: &str = "key-concurrent";
pub const KSDFGADSFGSDFGDFGEY: &str = "ksdfgadsfgsdfgdfgey";
pub const KSDFGDSFGSDFGEY: &str = "ksdfgdsfgsdfgey";
pub const KSDFSDFSDFSDFEY: &str = "ksdfsdfsdfsdfey";
pub const LABEL: &str = "label";
pub const LEFT: &str = "left";
pub const LEN: &str = "len";
pub const LENGTH_REQUIRED_411: &str = "length_required_411";
pub const LIB: &str = "lib";
pub const LIMIT: &str = "limit";
pub const LINE1_NEWLINE_LINE2_NEWLINE_LINE3: &str = "line1\nline2\nline3";
pub const LINKER_INFO: &str = "linker_info";
pub const LINTS: &str = "lints";
pub const LITERAL_PERCENT_VALUE: &str = "literal%value";
pub const LLVM_COV: &str = "llvm-cov";
pub const LOCATION_ALT: &str = "location";
pub const LOCATION_RS: &str = "location.rs";
pub const LOCATION_LIB: &str = "location_lib";
pub const LOCATION_TO_SCHEMA: &str = "location_to_schema";
pub const LOCKED_423: &str = "locked_423";
pub const LOGIN: &str = "login";
pub const LONG_PRODUCTION_STRING_LITERALS_MUST_BE_DEFINED_ONCE_AND_REUSED: &str =
    "long production string literals must be defined once and reused:";
pub const LOOP_DETECTED_508: &str = "loop_detected_508";
pub const LOSSY_PROVENANCE_CASTS: &str = "lossy_provenance_casts";
pub const LOWER: &str = "lower";
pub const MACHETE: &str = "machete";
pub const MACRO_GENERATION: &str = "macro-generation";
pub const MACRO_RULES_FOUND_USE_WORKSPACE_PROC_MACRO_CRATES_INSTEAD: &str =
    "macro_rules found; use workspace proc-macro crates instead:";
pub const MACRO_RULES: &str = "macro_rules";
pub const MACROS_HELPERS_SRC_PANIC_IF_ERR_RS: &str = "macros_helpers/src/panic_if_err.rs";
pub const MACROS_HELPERS_SRC_TOOL_COMMAND_RS: &str = "macros_helpers/src/tool_command.rs";
pub const MACROS_HELPERS_RS_EXT_PATH: &str = "macros_helpers_rs_ext_path";
pub const MACROS_HELPERS_SHOULD_WRITE_DIFF: &str = "macros_helpers_should_write_diff";
pub const MACROS_HELPERS_SHOULD_WRITE_DIFF_LEN: &str = "macros_helpers_should_write_diff_len";
pub const MACROS_HELPERS_SHOULD_WRITE_LARGE_DIFF: &str = "macros_helpers_should_write_large_diff";
pub const MACROS_HELPERS_SHOULD_WRITE_LARGE_SAME: &str = "macros_helpers_should_write_large_same";
pub const MACROS_HELPERS_SHOULD_WRITE_MISSING: &str = "macros_helpers_should_write_missing";
pub const MACROS_HELPERS_SHOULD_WRITE_SAME: &str = "macros_helpers_should_write_same";
pub const MACROS_HELPERS_SHOULD_WRITE_SAME_LEN_DIFF: &str =
    "macros_helpers_should_write_same_len_diff";
pub const MACROS_HELPERS_SKIP: &str = "macros_helpers_skip";
pub const MACROS_HELPERS_TRY_RUN_RUSTFMT: &str = "macros_helpers_try_run_rustfmt";
pub const MACROS_HELPERS_TRY_WRITE: &str = "macros_helpers_try_write";
pub const MACROS_HELPERS_TRY_WRITE_FILE: &str = "macros_helpers_try_write_file";
pub const MACROS_HELPERS_TRY_WRITE_PATH: &str = "macros_helpers_try_write_path";
pub const MACROS_HELPERS_TRY_WRITE_PATH_PASSTHROUGH: &str =
    "macros_helpers_try_write_path_passthrough";
pub const MACROS_HELPERS_WRITE: &str = "macros_helpers_write";
pub const MACROS_HELPERS_WRITE_FILE: &str = "macros_helpers_write_file";
pub const MACROS_HELPERS_WRITE_FILE_OUTCOME_CHANGED: &str =
    "macros_helpers_write_file_outcome_changed";
pub const MACROS_HELPERS_WRITE_FILE_OUTCOME_UNCHANGED: &str =
    "macros_helpers_write_file_outcome_unchanged";
pub const MACROS_HELPERS_WRITE_IF_CHANGED: &str = "macros_helpers_write_if_changed";
pub const MACROS_HELPERS_WRITE_IF_CHANGED_DIFF: &str = "macros_helpers_write_if_changed_diff";
pub const MACROS_HELPERS_WRITE_IF_NEEDED_DIFF: &str = "macros_helpers_write_if_needed_diff";
pub const MACROS_HELPERS_WRITE_IF_NEEDED_EQ: &str = "macros_helpers_write_if_needed_eq";
pub const MACROS_HELPERS_WRITE_OUTCOME_CHANGED: &str = "macros_helpers_write_outcome_changed";
pub const MACROS_HELPERS_WRITE_OUTCOME_INTO_PATH_CHANGED: &str =
    "macros_helpers_write_outcome_into_path_changed";
pub const MACROS_HELPERS_WRITE_OUTCOME_INTO_PATH_UNCHANGED: &str =
    "macros_helpers_write_outcome_into_path_unchanged";
pub const MACROS_HELPERS_WRITE_OUTCOME_UNCHANGED: &str = "macros_helpers_write_outcome_unchanged";
pub const MACROS_HELPERS_WRITE_PATH: &str = "macros_helpers_write_path";
pub const MALLOC: &str = "malloc|";
pub const MATCHING_REQUEST_IS_STILL_IN_PROGRESS: &str = "matching request is still in progress";
pub const MAX: &str = "max";
pub const MAX_AGE_31536000_INCLUDESUBDOMAINS: &str = "max-age=31536000; includeSubDomains";
pub const MAXITEMS: &str = "maxItems";
pub const MEASURE: &str = "measure";
pub const MEMBERS_NOT_SORTED: &str = "members not sorted:";
pub const MEMBERS: &str = "members";
pub const METHOD_NOT_ALLOWED: &str = "method not allowed";
pub const METHOD_NOT_ALLOWED_405: &str = "method_not_allowed_405";
pub const MICRO: &str = "micro";
pub const MICROSECOND: &str = "microsecond";
pub const MIN: &str = "min";
pub const MINITEMS: &str = "minItems";
pub const MINUTE: &str = "minute";
pub const MISDIRECTED_REQ_421: &str = "misdirected_req_421";
pub const MISSING_REVISION: &str = "missing-revision";
pub const MISSING_DIR: &str = "missing_dir";
pub const MOVED_PERMANENTLY_301: &str = "moved_permanently_301";
pub const MULTI_STATUS_207: &str = "multi_status_207";
pub const MULTIPLE_CHOICES_300: &str = "multiple_choices_300";
pub const MULTIPLE_SUPERTRAIT_UPCASTABLE: &str = "multiple_supertrait_upcastable";
pub const MUST_NOT_SUSPEND: &str = "must_not_suspend";
pub const NAME_DOLLAR_1_REVISION_REVISION_PLUS_1: &str = "name = $1, revision = revision + 1";
pub const NAME_DOLLAR_2: &str = "name = $2";
pub const NAME_DOLLAR_2_ALT: &str = "name = $2,";
pub const NAME_CASE_END: &str = "name = case ... end,";
pub const NAME_CASE_WHEN_ID_DOLLAR_1_THEN_DOLLAR_2_ELSE_NAME_END: &str =
    "name = case when id = $1 then $2 else name end,";
pub const NAME: &str = "name";
pub const NANOS: &str = "nanos";
pub const NEGATIVE_CONTENT_TYPE: &str = "negative-content-type";
pub const NEGATIVE_MALFORMED: &str = "negative-malformed";
pub const NEGATIVE_OVERSIZED: &str = "negative-oversized";
pub const NET: &str = "net";
pub const NETWORK_AUTHENTICATION_REQUIRED_511: &str = "network_authentication_required_511";
pub const NEVER_PRINT_THIS_VALUE: &str = "never-print-this-value";
pub const NEW: &str = "new";
pub const NEWTYPE: &str = "newtype";
pub const NEXTEST: &str = "nextest";
pub const NO_CACHE_NO_STORE_MUST_REVALIDATE: &str = "no-cache, no-store, must-revalidate";
pub const NO_REFERRER: &str = "no-referrer";
pub const NO_STORE: &str = "no-store";
pub const NO_CNT_204: &str = "no_cnt_204";
pub const NON_ENGLISH_SYMBOLS: &str = "non-english symbols:";
pub const NON_ASCII_U_E9: &str = "non_ascii_\u{e9}";
pub const NON_AUTHORITATIVE_INFORMATION_203: &str = "non_authoritative_information_203";
pub const NON_EXHAUSTIVE_OMITTED_PATTERNS: &str = "non_exhaustive_omitted_patterns";
pub const NOPE: &str = "nope";
pub const NOSNIFF: &str = "nosniff";
pub const NOT: &str = "not ";
pub const NOT_A_NUMBER: &str = "not-a-number";
pub const NOT_A_URL: &str = "not-a-url";
pub const NOT_AN_IP: &str = "not-an-ip";
pub const NOT_ACCEPTABLE_406: &str = "not_acceptable_406";
pub const NOT_EXTENDED_510: &str = "not_extended_510";
pub const NOT_FOUND_404: &str = "not_found_404";
pub const NOT_IMPLEMENTED_501: &str = "not_implemented_501";
pub const NOT_MODIFIED_304: &str = "not_modified_304";
pub const NUL_FREE: &str = "nul_free";
pub const OK_ALT: &str = "ok";
pub const OLD: &str = "old";
pub const ONLY_FIXTURE_VALUE_ONE_IS_ACCEPTED: &str = "only fixture value one is accepted";
pub const ONLY_ONE_TO_ERR_STRING_MODE_CAN_BE_SELECTED: &str =
    "only one to_err_string mode can be selected";
pub const OPT_ATTR_IS_NONE: &str = "opt attr is None";
pub const OR: &str = "or ";
pub const OR_NOT: &str = "or not ";
pub const ORDER_BY_ID: &str = "order by id";
pub const ORDER: &str = "order";
pub const ORDER_BY: &str = "order_by";
pub const OTHER_1_ADMIN_ACCESS_TOKEN_EXPECTED_ADMIN_ACCESS_TOKEN_SUFFIX_WRONG: &str =
    "other=1; admin_access_token=expected; admin_access_token_suffix=wrong";
pub const OVERSIZED: &str = "oversized";
pub const PACKAGE: &str = "package";
pub const PAGINATION: &str = "pagination";
pub const PANIC_CALL: &str = "panic!() call";
pub const PARSE_FAILED: &str = "parse failed";
pub const PARSE_GENERATE_PG_TABLE_INPUT_STAGE: &str = "parse_generate_pg_table_input_stage";
pub const PARTIAL_CNT_206: &str = "partial_cnt_206";
pub const PASSWORD: &str = "password";
pub const PASSWORD_HASH: &str = "password_hash";
pub const PATCH_ALT: &str = "patch";
pub const PATH_ALT_3: &str = "path = \"";
pub const PATH_ALT_4: &str = "path = \"./";
pub const PATH_ALT_5: &str = "path";
pub const PATHS: &str = "paths";
pub const PAYLOAD_TOO_LARGE_413: &str = "payload_too_large_413";
pub const PAYMENT_REQUIRED_402: &str = "payment_required_402";
pub const PENDING: &str = "pending";
pub const PERCENT_PERCENT_2FPASSWORD: &str = "percent%2Fpassword";
pub const PERCENT_PERCENT_40NAME: &str = "percent%40name";
pub const PERMANENT_REDIRECT_308: &str = "permanent_redirect_308";
pub const PERMISSION: &str = "permission";
pub const PERMISSIONS_NEWLINE_CONTENTS_READ: &str = "permissions:\n  contents: read";
pub const PG_CRUD_PG_CRUD_COMMON_SRC_LIB_RS: &str = "pg_crud/pg_crud_common/src/lib.rs";
pub const PG_CRUD_PG_CRUD_COMMON_SRC_SQL_IDENTIFIER_RS: &str =
    "pg_crud/pg_crud_common/src/sql_identifier.rs";
pub const PG_CRUD_PG_TABLE_GENERATE_PG_TABLE_SRC_SRC_LIB_RS: &str =
    "pg_crud/pg_table/generate_pg_table_src/src/lib.rs";
pub const PG_CRUD_PG_TYPES_GENERATE_PG_TYPES_SRC_SRC_LIB_RS: &str =
    "pg_crud/pg_types/generate_pg_types_src/src/lib.rs";
pub const PG_CRUD_WHERE_FILTERS_GENERATE_WHERE_FILTERS_SRC_SRC_LIB_RS: &str =
    "pg_crud/where_filters/generate_where_filters_src/src/lib.rs";
pub const PG_CRUD_WHERE_FILTERS_SRC_LIB_RS: &str = "pg_crud/where_filters/src/lib.rs";
pub const PG_CRUD_COMMON: &str = "pg_crud_common";
pub const PG_CRUD_COMMON_PGTYPE_READ: &str = "pg_crud_common.PgType.Read";
pub const PG_CRUD_COMMON_PGTYPE_SELECT: &str = "pg_crud_common.PgType.Select";
pub const PG_CRUD_COMMON_QUERY_PART: &str = "pg_crud_common_query_part";
pub const PG_TABLE_COLS_USING_PG_TYPES: &str = "pg_table_cols_using_pg_types";
pub const PLACEHOLDER: &str = "placeholder";
pub const POST_ALT: &str = "post";
pub const POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION: &str =
    "postgres://admin:integration-only@127.0.0.1/admin_integration";
pub const POSTGRES_ADMIN_PRODUCTION_SECRET_DB_EXAMPLE_COM_APP_TEST: &str =
    "postgres://admin:production-secret@db.example.com/app_test";
pub const POSTGRES_ADMIN_PRODUCTION_SECRET_LOCALHOST_POSTGRES: &str =
    "postgres://admin:production-secret@localhost/postgres";
pub const POSTGRES_ADMIN_PRODUCTION_SECRET_LOCALHOST_PRODUCTION: &str =
    "postgres://admin:production-secret@localhost/production";
pub const POSTGRES_DB: &str = "postgres://db";
pub const POSTGRES_PERCENT_PERCENT_40NAME_PERCENT_PERCENT_2FPASSWORD_PATH_1_TEST_FRAGMENT: &str =
    "postgres://percent%40name:percent%2Fpassword@[::1]/test#fragment";
pub const POSTGRES_USER_SECRET_PATH_1_TEST_CI_FRAGMENT: &str =
    "postgres://user:secret@[::1]/test_ci#fragment";
pub const POSTGRES_USER_SECRET_LOCALHOST_TEST: &str = "postgres://user:secret@localhost/test";
pub const POSTGRES_USERNAME_PASSWORD_LOCALHOST_TEST_QUESTION_SSLMODE_DISABLE: &str =
    "postgres://username:password@localhost/test?sslmode=disable";
pub const POSTGRES_USERNAME_LOCALHOST_TEST: &str = "postgres://username@localhost/test";
pub const POSTGRES_USR_PWD_LOCALHOST_5432_DB: &str = "postgres://usr:pwd@localhost:5432/db";
pub const POSTGRESQL_USER_SECRET_127_0_0_1_5432_APP_TEST_QUESTION_SSLMODE: &str =
    "postgresql://user:secret@127.0.0.1:5432/app_test?sslmode=disable";
pub const PRECONDITION_FAILED_412: &str = "precondition_failed_412";
pub const PRECONDITION_REQUIRED_428: &str = "precondition_required_428";
pub const PRIMARY_KEY: &str = "primary key";
pub const PRINTF: &str = "printf";
pub const PROC_MACRO: &str = "proc-macro";
pub const PROC_MACRO_ALT: &str = "proc_macro";
pub const PROC_MACRO_ATTRIBUTE: &str = "proc_macro_attribute";
pub const PROC_MACRO_DERIVE: &str = "proc_macro_derive";
pub const PROCESSING_102: &str = "processing_102";
pub const PRODUCTION_SECRET: &str = "production-secret";
pub const PROGRAM: &str = "program";
pub const PROPERTIES: &str = "properties";
pub const PROXY_AUTHENTICATION_REQUIRED_407: &str = "proxy_authentication_required_407";
pub const PUBLIC_TUPLE_WRAPPERS_MUST_NOT_EXPOSE_INNER_FIELDS_INITIALIZE_THEM_THROUGH_FROM: &str =
    "public tuple wrappers must not expose inner fields; initialize them through From/TryFrom:";
pub const PUBLIC: &str = "public";
pub const PUBLISH: &str = "publish";
pub const PUT: &str = "put";
pub const QWE: &str = "qwe";
pub const RANGE_NOT_SATISFIABLE_416: &str = "range_not_satisfiable_416";
pub const RATE_LIMITED: &str = "rate limited";
pub const RAW_EXTERNAL_OR_PRIMITIVE_TYPES_FOUND_IN_DOMAIN_BOUNDARIES_USE_REPOSITORY_DOMAIN: &str = "raw external or primitive types found in domain boundaries; use repository domain wrapper types initialized with From/TryFrom:";
pub const RAW_TEXT_CONTAINERS_FOUND_IN_HELPER_STRUCT_FIELDS_USE_REPOSITORY_WRAPPER_TYPES: &str =
    "raw text containers found in helper struct fields; use repository wrapper types:";
pub const RAW_TEXT_RETURN_TYPES_FOUND_IN_HELPER_FUNCTIONS_USE_REPOSITORY_WRAPPER_TYPES: &str =
    "raw text return types found in helper functions; use repository wrapper types:";
pub const REALLOC: &str = "realloc|";
pub const REFERRER_POLICY: &str = "referrer-policy";
pub const REFRESH: &str = "refresh";
pub const RELEASE: &str = "release";
pub const REQ_HEADER_FIELDS_TOO_LARGE_431: &str = "req_header_fields_too_large_431";
pub const REQ_TIMEOUT_408: &str = "req_timeout_408";
pub const REQUEST_BODY_IS_TOO_LARGE: &str = "request body is too large";
pub const REQUEST_FAILED: &str = "request failed";
pub const REQUEST_ID_MUST_BE_NON_EMPTY_ASCII_UP_TO_128_BYTES: &str =
    "request id must be non-empty ASCII up to 128 bytes";
pub const REQUEST_PRECONDITION_IS_REQUIRED: &str = "request precondition is required";
pub const REQUEST_RATE_LIMIT_EXCEEDED_ALT: &str = "request rate limit exceeded";
pub const REQUEST_TIMEOUT_MUST_BE_GREATER_THAN_ZERO: &str =
    "request timeout must be greater than zero";
pub const REQUEST_TIMEOUT: &str = "request timeout";
pub const REQUEST_VALIDATION_FAILED: &str = "request validation failed";
pub const REQWEST: &str = "reqwest";
pub const RESET_CNT_205: &str = "reset_cnt_205";
pub const RESOLVING_TO_ITEMS_SHADOWING_SUPERTRAIT_ITEMS: &str =
    "resolving_to_items_shadowing_supertrait_items";
pub const RESOURCE_BUDGET_EXHAUSTED: &str = "resource budget exhausted";
pub const RESOURCE_BUDGET_MAXIMUM_MUST_BE_GREATER_THAN_ZERO: &str =
    "resource budget maximum must be greater than zero";
pub const RESOURCE_BUDGET_RESERVATION_OVERFLOW: &str = "resource budget reservation overflow";
pub const RESOURCE_NOT_FOUND: &str = "resource not found";
pub const RESOURCE_PRECONDITION_FAILED: &str = "resource precondition failed";
pub const RESOURCE_STATE_CONFLICT: &str = "resource state conflict";
pub const RESOURCE: &str = "resource";
pub const RESPONSES: &str = "responses";
pub const RETRY_AFTER_SECONDS_MUST_BE_GREATER_THAN_ZERO: &str =
    "retry-after seconds must be greater than zero";
pub const REVISION_MUST_BE_A_DECIMAL_INTEGER: &str = "revision must be a decimal integer";
pub const REVISION_MUST_NOT_BE_NEGATIVE: &str = "revision must not be negative";
pub const REVISION: &str = "revision";
pub const RHYSD_ACTIONLINT: &str = "rhysd/actionlint@";
pub const RIGHT: &str = "right";
pub const RM: &str = "rm";
pub const RO: &str = "ro";
pub const ROLE: &str = "role";
pub const ROOT: &str = "root";
pub const ROOT_ADMIN_ALT: &str = "root_admin";
pub const ROUTE_READ: &str = "route_read";
pub const RS: &str = "rs";
pub const RTY: &str = "rty";
pub const RUN_HISTORY_MAXIMUM_LENGTH_MUST_BE_GREATER_THAN_ZERO: &str =
    "run history maximum length must be greater than zero";
pub const RUN_INTERVAL_MUST_BE_GREATER_THAN_ZERO: &str = "run interval must be greater than zero";
pub const RUNTIME_ARC_USAGE_MUST_BE_LIMITED_TO_EXPLICIT_CROSS_THREAD_SHARED_STATE: &str =
    "runtime Arc usage must be limited to explicit cross-thread shared state:";
pub const RUNTIME_CODE_CONTAINS_MUTEX_USE_IT_ONLY_FOR_JUSTIFIED_INTERIOR_MUTABILITY: &str =
    "runtime code contains Mutex; use it only for justified interior mutability:";
pub const RUNTIME_CODE_CONTAINS_FORBIDDEN_EXPECT_UNWRAP_PANIC_CALLS_USE_RESULT_WITH_A: &str = "runtime code contains forbidden expect/unwrap/panic calls; use Result with a thiserror-like error enum instead:";
pub const RUNTIME_CODE_PERFORMS_AN_UNBOUNDED_FILE_OR_HTTP_RESPONSE_READ: &str =
    "runtime code performs an unbounded file or HTTP response read:";
pub const RUST: &str = "rust";
pub const RUSTC: &str = "rustc";
pub const RUSTFMT: &str = "rustfmt";
pub const RUSTFMT_TOML: &str = "rustfmt.toml";
pub const SAME: &str = "same";
pub const SCHEMAS: &str = "schemas";
pub const SEARCH_PATH: &str = "search_path";
pub const SEC: &str = "sec";
pub const SECOND_ALT: &str = "second";
pub const SECRET_CANNOT_BE_COMBINED_WITH_FORMATTING_TOKEN_OR_ERROR_STRING_FORWARDING: &str =
    "secret cannot be combined with formatting, token, or error-string forwarding";
pub const SECRET: &str = "secret";
pub const SECRET_VALUE: &str = "secret-value";
pub const SECS: &str = "secs";
pub const SEE_OTHER_303: &str = "see_other_303";
pub const SELECT_ALT: &str = "select ";
pub const SELECT_ID_NAME_FROM_USERS_ORDER_BY_ID: &str = "select id,name from users order by id";
pub const SELECT_ID_NAME_FROM_USERS_WHERE_ID_DOLLAR_1: &str =
    "select id,name from users where id = $1";
pub const SELECT_ALT_3: &str = "select";
pub const SELF_ALT: &str = "self";
pub const SEMVER_CHECKS: &str = "semver-checks";
pub const SERDE: &str = "serde";
pub const SERDE_JSON: &str = "serde_json";
pub const SERVER_GRACEFUL_SHUTDOWN_TIMED_OUT: &str = "server graceful shutdown timed out";
pub const SERVER_RETURNED_AN_ERROR_RESPONSE: &str = "server returned an error response";
pub const SERVER_SRC_MAIN_RS: &str = "server/src/main.rs";
pub const SERVER_ADMIN_SRC_LIB_RS: &str = "server_admin/src/lib.rs";
pub const SERVER_ADMIN_SRC_PASSWORD_RS: &str = "server_admin/src/password.rs";
pub const SERVER_ADMIN_FRONTEND_SRC_APP_RS: &str = "server_admin_frontend/src/app.rs";
pub const SERVER_ADMIN_FRONTEND_SRC_APP_FORMS_RS: &str = "server_admin_frontend/src/app/forms.rs";
pub const SERVER_ADMIN_FRONTEND_SRC_APP_PAGES_RS: &str = "server_admin_frontend/src/app/pages.rs";
pub const SERVER_ADMIN_FRONTEND_SRC_APP_TABLES_RS: &str = "server_admin_frontend/src/app/tables.rs";
pub const SERVER_ADMIN_FRONTEND_SRC_LIB_RS: &str = "server_admin_frontend/src/lib.rs";
pub const SERVER_RUNTIME_SRC_BOUNDED_READ_RS: &str = "server_runtime/src/bounded_read.rs";
pub const SERVER_RUNTIME_SRC_HEALTH_RS: &str = "server_runtime/src/health.rs";
pub const SERVER_RUNTIME_SRC_LIB_RS: &str = "server_runtime/src/lib.rs";
pub const SERVICE: &str = "service";
pub const SERVICE_ENV: &str = "service/.env";
pub const SERVICE_ENV_EXAMPLE: &str = "service/.env.example";
pub const SERVICE_UNAVAILABLE_503: &str = "service_unavailable_503";
pub const SESSION: &str = "session";
pub const SHADOWING_SUPERTRAIT_ITEMS: &str = "shadowing_supertrait_items";
pub const SIGN_IN: &str = "sign_in";
pub const SIGN_OUT: &str = "sign_out";
pub const SIMPLE_CONSTANT_ALIASES_FOUND_USE_THE_SOURCE_CONSTANT_DIRECTLY: &str =
    "simple constant aliases found; use the source constant directly:";
pub const SLEEP: &str = "sleep";
pub const SORTABLE: &str = "sortable";
pub const SPAWN_RESULT_IS_DISCARDED_RETAIN_AND_SUPERVISE_ITS_HANDLE: &str =
    "spawn result is discarded; retain and supervise its handle";
pub const SPAWNED_TASK_HANDLES_ARE_DISCARDED: &str = "spawned task handles are discarded:";
pub const SQL_SELECT_BUILDER_128_COLUMNS: &str = "sql_select_builder_128_columns";
pub const SQLX_PATH_TYPE_NAME: &str = "sqlx :: type_name";
pub const SQLX: &str = "sqlx";
pub const SQLX_PATH_PATH_TYPE_NAME: &str = "sqlx::::type_name";
pub const SRC_ALT: &str = "src";
pub const SRC_ERROR_RS: &str = "src/error.rs";
pub const SRC_GENERATED: &str = "src/generated";
pub const SRC_GENERATED_TXT: &str = "src/generated.txt";
pub const SRC_LIB_RS: &str = "src/lib.rs";
pub const STACK_PEAK: &str = "stack peak:";
pub const STATIC: &str = "static";
pub const STATUS_ALT: &str = "status";
pub const STD: &str = "std";
pub const STD_PATH_ENV_PATH: &str = "std::env::";
pub const STD_PATH_FS_PATH: &str = "std::fs::";
pub const STD_PATH_PROCESS_PATH_COMMAND_PATH_NEW: &str = "std::process::Command::new";
pub const STD_PATH_PROCESS_PATH_ABORT: &str = "std::process::abort";
pub const STR_ALT: &str = "str";
pub const STRICT_TRANSPORT_SECURITY: &str = "strict-transport-security";
pub const STRING_CONSTANTS_FOUND_OUTSIDE_STR_CONSTANTS: &str =
    "string constants found outside str_constants:";
pub const STRING_WRAPPERS_MUST_VALIDATE_LENGTH_USE_TRYFROM_STRING_WITH_A_LENGTH_CHECK: &str = "string wrappers must validate length; use TryFrom<String> with a length check instead of From<String>:";
pub const STRING_ALT: &str = "string";
pub const STRUCT_A: &str = "struct A ;";
pub const STRUCT_A_NEWLINE: &str = "struct A;\n";
pub const STRUCT_B: &str = "struct B;";
pub const STRUCT_DIDWRITE: &str = "struct DidWrite ;";
pub const STRUCT_PATHINPUT: &str = "struct PathInput ;";
pub const STRUCT_SKIPWRITE: &str = "struct SkipWrite;";
pub const STRUCT_TRYDIDWRITE: &str = "struct TryDidWrite ;";
pub const STRUCT: &str = "struct";
pub const SUCCEEDED: &str = "succeeded";
pub const SUMMARY_TXT: &str = "summary.txt";
pub const SUPER: &str = "super";
pub const SUPERTRAIT_ITEM_SHADOWING_DEFINITION: &str = "supertrait_item_shadowing_definition";
pub const SUPERTRAIT_ITEM_SHADOWING_USAGE: &str = "supertrait_item_shadowing_usage";
pub const SWITCHING_PROTOCOLS_101: &str = "switching_protocols_101";
pub const SYN_FIELD_RS: &str = "syn_field.rs";
pub const SYSTEM: &str = "system";
pub const SYSTEM_SETTINGS: &str = "system_settings";
pub const TABLE_ALT: &str = "table";
pub const TABLE_NAME: &str = "table-name";
pub const TABLE_NAME_ALT: &str = "table.name";
pub const TABLE_2: &str = "table_2";
pub const TABLE_EXAMPLE_CREATE: &str = "table_example:create";
pub const TABLE_EXAMPLE_DELETE: &str = "table_example:delete";
pub const TABLE_EXAMPLE_READ: &str = "table_example:read";
pub const TABLE_EXAMPLE_UPDATE: &str = "table_example:update";
pub const TABLE_NAMES_CLONED_TABLE_NAMES_ITER_MAP: &str =
    "table_names_cloned = table_names.iter().map";
pub const TAIL_CALL_TRACK_CALLER: &str = "tail_call_track_caller";
pub const TARGET: &str = "target";
pub const TARGET_MACRO_CHECK: &str = "target/macro-check";
pub const TARGET_MEASURE_GENERATE_PG_TABLE_WITH_TESTS: &str =
    "target/measure/generate_pg_table_with_tests";
pub const TASK: &str = "task";
pub const TEMPORARY_REDIRECT_307: &str = "temporary_redirect_307";
pub const TEST_ALT: &str = "test ";
pub const TEST_ALT_3: &str = "test";
pub const TEST_AUDIENCE: &str = "test-audience";
pub const TEST_ISSUER: &str = "test-issuer";
pub const TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES: &str =
    "test-only-admin-jwt-secret-with-32-bytes";
pub const TEST_ONLY_SECRET_WITH_SUFFICIENT_ENTROPY: &str =
    "test-only-secret-with-sufficient-entropy";
pub const TEST_UTILS: &str = "test-utils";
pub const TEST_ALT_4: &str = "test_";
pub const TEST_HLP_RS: &str = "test_hlp.rs";
pub const TEST_UNSTABLE_LINT: &str = "test_unstable_lint";
pub const TESTS_ALT: &str = "tests";
pub const TEXT_PLAIN: &str = "text/plain";
pub const THREAD: &str = "thread";
pub const TIMEOUT_MINUTES: &str = "timeout-minutes:";
pub const TMP_A_B_C: &str = "tmp/a/b/c";
pub const TO_ERR_STRING: &str = "to_err_string";
pub const TO_ERR_STRING_AS_REF_STR: &str = "to_err_string_as_ref_str";
pub const TO_ERR_STRING_DEBUG: &str = "to_err_string_debug";
pub const TO_ERR_STRING_DISPLAY: &str = "to_err_string_display";
pub const TO_TOKENS: &str = "to_tokens";
pub const TODO_UNIMPLEMENTED_FOUND: &str = "todo!/unimplemented! found:";
pub const TODO: &str = "todo";
pub const TOKIO: &str = "tokio";
pub const TOKIO_PATH_FS_PATH: &str = "tokio::fs::";
pub const TOKIO_PATH_TIME_PATH_SLEEP: &str = "tokio::time::sleep";
pub const TOO_LONG: &str = "too long";
pub const TOO_BIG: &str = "too-big";
pub const TOO_MANY_REQS_429: &str = "too_many_reqs_429";
pub const TRIM: &str = "trim";
pub const TRUE: &str = "true";
pub const TRYBUILD_ROUTE_CONTRACT_ASTERISK_RS: &str = "trybuild/route_contract_*.rs";
pub const TUPLE_WRAPPERS_OVER_EXTERNAL_TYPES_MUST_INCLUDE_THE_EXTERNAL_CRATE_NAME: &str =
    "tuple wrappers over external types must include the external crate name:";
pub const TWO_OR_MORE_SUPPORTED_ATTRS: &str = "two or more supported attrs!";
pub const TXT: &str = "txt";
pub const TYPE_ALIASES_FOUND_USE_EXPLICIT_TYPES_AT_USAGE_SITES: &str =
    "type aliases found; use explicit types at usage sites:";
pub const TYPES_PATH_SOURCETEXT: &str = "types::SourceText";
pub const TYPES_PATH_SOURCETEXTLIST: &str = "types::SourceTextList";
pub const TYPES_PATH_SOURCETEXTREF: &str = "types::SourceTextRef";
pub const TYPES_PATH_STDSOURCETEXTHASHSET_OR_TYPES_PATH_STDSOURCETEXTREFSET: &str =
    "types::StdSourceTextHashSet or types::StdSourceTextRefSet";
pub const TYPES_PATH_STDSOURCETEXTSET: &str = "types::StdSourceTextSet";
pub const UDEPS: &str = "udeps";
pub const UNAUTHORIZED_401: &str = "unauthorized_401";
pub const UNAVAILABLE: &str = "unavailable";
pub const UNAVAILABLE_FOR_LEGAL_REASONS_451: &str = "unavailable_for_legal_reasons_451";
pub const UNIMPLEMENTED: &str = "unimplemented";
pub const UNIT_TESTS_CONTAIN_EXTERNAL_SERVICE_CLIENTS_USE_DETERMINISTIC_LOCAL_FAKES_INSTEAD: &str =
    "unit tests contain external-service clients; use deterministic local fakes instead:";
pub const UNIT_TESTS_USE_NONDETERMINISTIC_TIME_SLEEP_OR_RANDOMNESS_WITHOUT_A_REVIEWED_OWNER: &str =
    "unit tests use nondeterministic time, sleep, or randomness without a reviewed owner:";
pub const UNKNOWN_BOUNDED_STRING_OPTION: &str = "unknown bounded_string option";
pub const UNKNOWN_NEWTYPE_OPTION: &str = "unknown newtype option";
pub const UNKNOWN_ALT: &str = "unknown";
pub const UNKNOWN_USER_AGENT: &str = "unknown-user-agent";
pub const UNKNOWN_READ: &str = "unknown:read";
pub const UNPROCESSABLE_ENTITY_422: &str = "unprocessable_entity_422";
pub const UNQUALIFIED_LOCAL_IMPORTS: &str = "unqualified_local_imports";
pub const UNREACHABLE_CFG_SELECT_PREDICATES: &str = "unreachable_cfg_select_predicates";
pub const UNSUPPORTED_MEDIA_TYPE_415: &str = "unsupported_media_type_415";
pub const UNUSED: &str = "unused";
pub const UNWRAP: &str = "unwrap";
pub const UNWRAP_CALL_ALT: &str = "unwrap() call";
pub const UNWRAP_FOUND: &str = "unwrap() found:";
pub const UO: &str = "uo";
pub const UPDATE_ALT: &str = "update ";
pub const UPDATE_USERS_SET_NAME_DOLLAR_1_REVISION_REVISION_PLUS_1_WHERE_ID: &str = "update users set name = $1, revision = revision + 1 where id = $2 and revision = $3 returning id,revision";
pub const UPDATE_USERS_SET_NAME_DOLLAR_2_WHERE_ID_DOLLAR_1_RETURNING_ID: &str =
    "update users set name = $2 where id = $1 returning id,name";
pub const UPDATE_USERS_SET_NAME_CASE_END_WHERE_ID_IN_DOLLAR_1_DOLLAR: &str =
    "update users set name = case ... end, where id in ($1,$2) returning id,name";
pub const UPDATE_ONE: &str = "update_one";
pub const UPDATED_AT: &str = "updated_at";
pub const UPGRADE_REQUIRED_426: &str = "upgrade_required_426";
pub const UPPER: &str = "upper";
pub const URI_TOO_LONG_414: &str = "uri_too_long_414";
pub const USE_IMPORTS_FOUND_OUTSIDE_EXPLICIT_FACADE_RE_EXPORT_FILES_PREFER_EXPLICIT_PATHS: &str = "use imports found outside explicit facade re-export files; prefer explicit paths at usage sites:";
pub const USE_PROXY_305: &str = "use_proxy_305";
pub const USER: &str = "user";
pub const USER_ID: &str = "user_id";
pub const USERNAME: &str = "username";
pub const USERS_ALT: &str = "users";
pub const UTOIPA: &str = "utoipa";
pub const UUID_PATH_UUID_PATH_NEW_V4: &str = "uuid::Uuid::new_v4";
pub const V_USIZE: &str = "v:usize";
pub const VALIDATE_GENERATE_PG_TABLE_FIELDS_MODEL_STAGE: &str =
    "validate_generate_pg_table_fields_model_stage";
pub const VALIDATION_FAILED: &str = "validation failed";
pub const VALSDFGDSAFGDSGUE: &str = "valsdfgdsafgdsgue";
pub const VALSDFGDSGDUE: &str = "valsdfgdsgdue";
pub const VALSFDSFDSFDSUE: &str = "valsfdsfdsfdsue";
pub const VALUSDFGDSGDSFGDE: &str = "valusdfgdsgdsfgde";
pub const VARIANT: &str = "variant";
pub const VARIANT_ALSO_NEGOTIATES_506: &str = "variant_also_negotiates_506";
pub const VASDFGDGDFGLUE: &str = "vasdfgdgdfglue";
pub const VASFDSDFSDFLUE: &str = "vasfdsdfsdflue";
pub const VERSION_ALT_3: &str = "version";
pub const WHEN_ID_DOLLAR_1_THEN_DOLLAR_2: &str = "when id = $1 then $2 ";
pub const WHERE_ALT: &str = "where ";
pub const WHERE_ID_IN_DOLLAR_1_DOLLAR_2_AND_ACTIVE_TRUE: &str =
    "where id in ($1,$2) and active = true";
pub const WHERE_ID_IN_DOLLAR_1_DOLLAR_2: &str = "where id in ($1,$2)";
pub const WHERE_FILTERS_PGTYPEWHEREBETWEEN: &str = "where_filters.PgTypeWhereBetween";
pub const WHERE_FILTERS_PGTYPEWHEREEQ: &str = "where_filters.PgTypeWhereEq";
pub const WHERE_FILTERS_PGTYPEWHEREGREATERTHAN: &str = "where_filters.PgTypeWhereGreaterThan";
pub const WHERE_FILTERS_PGTYPEWHEREIN: &str = "where_filters.PgTypeWhereIn";
pub const WHERE_FILTERS_QUERY_PART: &str = "where_filters_query_part";
pub const WHERE_MANY: &str = "where_many";
pub const WITH_NOT_EQUALS_1_ARG: &str = "with != 1 arg";
pub const WORKSPACE_TRUE: &str = "workspace = true";
pub const WORKSPACE: &str = "workspace";
pub const WORKSPACE_TEST_RUNNER_ALT: &str = "workspace_test_runner";
pub const WRITE_ALT: &str = "write";
pub const WRITE_STRING_INTO_FILE_RS: &str = "write_string_into_file.rs";
pub const WRITE_TOKEN_STREAM_INTO_FILE_RS: &str = "write_token_stream_into_file.rs";
pub const WRONG_AUDIENCE: &str = "wrong-audience";
pub const X: &str = "x";
pub const X_COMMIT: &str = "x-commit";
pub const X_CONTENT_TYPE_OPTIONS: &str = "x-content-type-options";
pub const X_CSRF_TOKEN_ALT: &str = "x-csrf-token";
pub const X_FORWARDED_PROTO: &str = "x-forwarded-proto";
pub const X_FRAME_OPTIONS: &str = "x-frame-options";
pub const XYZ: &str = "xyz";
pub const TEXT_ALT_13: &str = "{";
pub const DISPLAY_NAME_ADMIN_LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE: &str =
    "{\"display_name\":\"Admin\",\"login\":\"admin\",\"password\":\"secret\",\"unknown\":true}";
pub const DISPLAY_NAME_ADMIN_UNKNOWN_TRUE: &str = "{\"display_name\":\"Admin\",\"unknown\":true}";
pub const DISPLAY_NAME_UPDATED_USER: &str = "{\"display_name\":\"Updated User\"}";
pub const IS_BANNED_TRUE_UNKNOWN_TRUE: &str = "{\"is_banned\":true,\"unknown\":true}";
pub const IS_BANNED_TRUE: &str = "{\"is_banned\":true}";
pub const LOGIN_ALT: &str = "{\"login\":";
pub const LOGIN_ADMIN_PASSWORD_PASSWORD: &str = "{\"login\":\"admin\",\"password\":\"password\"}";
pub const LOGIN_ADMIN_PASSWORD_SECRET_UNKNOWN_TRUE: &str =
    "{\"login\":\"admin\",\"password\":\"secret\",\"unknown\":true}";
pub const LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD: &str = "{\"login\":\"limited_user\",\"display_name\":\"Limited User\",\"password\":\"limited-password\"}";
pub const LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD: &str =
    "{\"login\":\"limited_user\",\"password\":\"limited-password\"}";
pub const LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD: &str =
    "{\"login\":\"locked_user\",\"password\":\"wrong-password\"}";
pub const LOGIN_ROOT_ADMIN_PASSWORD_CORRECT_PASSWORD: &str =
    "{\"login\":\"root_admin\",\"password\":\"correct-password\"}";
pub const LOGIN_ROOT_ADMIN_PASSWORD_WRONG_PASSWORD: &str =
    "{\"login\":\"root_admin\",\"password\":\"wrong-password\"}";
pub const NAME_ADMINISTRATOR_UNKNOWN_TRUE: &str = "{\"name\":\"administrator\",\"unknown\":true}";
pub const NAME_RENAMED_ROLE: &str = "{\"name\":\"renamed_role\"}";
pub const NAME_TEMPORARY_ROLE: &str = "{\"name\":\"temporary_role\"}";
pub const OPERATION_RM: &str = "{\"operation\":\"rm\"}";
pub const PASSWORD_SECRET_UNKNOWN_TRUE: &str = "{\"password\":\"secret\",\"unknown\":true}";
pub const PERMISSION_IDS_1_UNKNOWN_TRUE: &str = "{\"permission_ids\":[1],\"unknown\":true}";
pub const ROLE_IDS_1_UNKNOWN_TRUE: &str = "{\"role_ids\":[1],\"unknown\":true}";
pub const SITE_NAME_ADMIN_UNKNOWN_TRUE: &str = "{\"site_name\":\"Admin\",\"unknown\":true}";
pub const VALUE_1_ALT: &str = "{\"value\":1}";
pub const VALUE_7: &str = "{\"value\":7}";
pub const VALUE_1_2: &str = "{\"value\":[1,2]}";
pub const COLUMN_ALT: &str = "{column},";
pub const V_ALT: &str = "{v}";
pub const TEXT_ALT_14: &str = "{}";
pub const USER_AGENT: &str = "|user-agent=";
pub const TEXT_ALT_15: &str = "~";
pub const ASTERISK_ALT: &str = "~*";
pub const VALUE_1_ALT_3: &str = "~1";
pub const U_3053_U_3093_U_306B_U_3061_U_306F: &str = "\u{3053}\u{3093}\u{306b}\u{3061}\u{306f}";
pub const U_1F30D_U_1F680_U_2728_RUST_U_1F496_U_1F980: &str =
    "\u{1f30d}\u{1f680}\u{2728} Rust \u{1f496}\u{1f980}";
pub const U_1F496: &str = "\u{1f496}";
pub const U_1F600: &str = "\u{1f600}";
pub const VALUE_08708789: &str = "08708789";
pub const VALUE_7565757: &str = "7565757";
pub const VALUE_97697697: &str = "97697697";
pub const VALUE_123: &str = "123";
pub const POSTGRES: &str = "postgres";
pub const POSTGRESQL: &str = "postgresql";
pub const LOCALHOST: &str = "localhost";
pub const PATH_1: &str = "::1";
pub const FAILED_TO_WAIT_FOR_CTRL_C_SIGNAL: &str = "failed to wait for ctrl-c signal";
pub const VALUE_127_0_0_1_32: &str = "127.0.0.1/32";
pub const VALUE_5C81D907: &str = "5c81d907";
pub const DELETE: &str = "DELETE";
pub const I64ASNONNULLINT8: &str = "I64AsNonNullInt8";
pub const I64ASNONNULLBIGSERIALINITIALIZATIONBYPG: &str = "I64AsNonNullBigSerialInitializationByPg";
pub const STATUSCODE: &str = "StatusCode";
pub const HEADER: &str = "header";
pub const HEADERMAP: &str = "HeaderMap";
pub const ROUTE_VALIDATORS: &str = "route_validators";
pub const CHECK_BODY_SIZE: &str = "check_body_size";
pub const MONGODB_DB: &str = "mongodb://db";
pub const REDIS_DB: &str = "redis://db";
pub const GITHUB_ALT: &str = "GITHUB";
pub const DEBUG: &str = "DeBuG";
pub const TRUTHY: &str = "truthy";
pub const VALUE_128: &str = "128";
pub const VALUE_1K: &str = "1k";
pub const HTTPS_EXAMPLE_COM: &str = "https://example.com";
pub const NAN: &str = "nan";
pub const STD_PATH_FS_PATH_READ: &str = "std::fs::read";
pub const STD_PATH_FS_PATH_READ_TO_STRING: &str = "std::fs::read_to_string";
pub const TOKIO_PATH_FS_PATH_READ: &str = "tokio::fs::read";
pub const TOKIO_PATH_FS_PATH_READ_TO_STRING: &str = "tokio::fs::read_to_string";
pub const TOKIO_PATH_SPAWN: &str = "tokio::spawn";
pub const TOKIO_PATH_TASK_PATH_SPAWN_BLOCKING: &str = "tokio::task::spawn_blocking";
pub const STD_PATH_THREAD_PATH_SPAWN: &str = "std::thread::spawn";
pub const RAND_PATH_THREAD_RANGE: &str = "rand::thread_range";
pub const STD_PATH_THREAD_PATH_SLEEP: &str = "std::thread::sleep";
pub const STD_PATH_TIME_PATH_SYSTEMTIME_PATH_NOW: &str = "std::time::SystemTime::now";
pub const COMPILE_ERROR_TOKEN_STREAM: &str = "compile_error_token_stream";
pub const TOML: &str = "toml";
pub const YML: &str = "yml";
pub const YAML: &str = "yaml";
pub const JSON: &str = "json";
pub const BLOCKING_RECV: &str = "blocking_recv";
pub const BLOCKING_SEND: &str = "blocking_send";
pub const TRACING_PATH_DISPATCHER_PATH_SETGLOBALDEFAULTERROR: &str =
    "tracing::dispatcher::SetGlobalDefaultError";
pub const TRACING_PATH_LOG_PATH_TRACING_PATH_LOG_PATH_SETLOGGERERROR: &str =
    "tracing::log::tracing::log::SetLoggerError";
pub const VALUE_979FA4B2: &str = "979fa4b2";
pub const VALUE_589EA31D: &str = "589ea31d";

#[cfg(test)]
#[test]
fn generated_common_route_constants_have_expected_values() {
    assert_eq!(COMMON_ROUTES_HEALTH, "/health");
    assert_eq!(COMMON_ROUTES_HEALTH_CHECK, "/health_check");
    assert_eq!(COMMON_ROUTES_HEALTH_LIVE, "/health/live");
    assert_eq!(COMMON_ROUTES_HEALTH_READY, "/health/ready");
}

#[cfg(test)]
#[test]
fn generated_admin_api_path_constants_have_expected_values() {
    assert!(
        [
            (ADMIN_API_PATHS_AUDIT, "/audit-log"),
            (ADMIN_API_PATHS_AUTH_ME, "/auth/me"),
            (ADMIN_API_PATHS_AUTH_REFRESH, "/auth/refresh"),
            (ADMIN_API_PATHS_AUTH_SESSION, "/auth/sessions/{session_id}"),
            (ADMIN_API_PATHS_AUTH_SESSIONS, "/auth/sessions"),
            (ADMIN_API_PATHS_AUTH_SIGN_IN, "/auth/sign-in"),
            (ADMIN_API_PATHS_AUTH_SIGN_OUT, "/auth/sign-out"),
            (ADMIN_API_PATHS_PERMISSIONS, "/permissions"),
            (ADMIN_API_PATHS_ROLE, "/roles/{role_id}"),
            (
                ADMIN_API_PATHS_ROLE_PERMISSIONS,
                "/roles/{role_id}/permissions",
            ),
            (ADMIN_API_PATHS_ROLES, "/roles"),
            (ADMIN_API_PATHS_SETTINGS, "/system-settings"),
            (ADMIN_API_PATHS_USER, "/users/{user_id}"),
            (ADMIN_API_PATHS_USER_BAN, "/users/{user_id}/ban"),
            (ADMIN_API_PATHS_USER_PASSWORD, "/users/{user_id}/password"),
            (ADMIN_API_PATHS_USER_ROLES, "/users/{user_id}/roles"),
            (ADMIN_API_PATHS_USERS, "/users"),
        ]
        .into_iter()
        .all(|(actual, expected)| actual == expected)
    );
}

#[cfg(test)]
#[test]
fn generated_admin_page_path_constants_have_expected_values() {
    assert!(
        [
            (ADMIN_PAGE_PATHS_ASSETS, "/admin/assets"),
            (ADMIN_PAGE_PATHS_AUDIT, "/admin/audit-log"),
            (ADMIN_PAGE_PATHS_METRICS, "/admin/metrics"),
            (ADMIN_PAGE_PATHS_OPEN_API, "/admin/swagger-ui"),
            (ADMIN_PAGE_PATHS_OPEN_API_DOCUMENT, "/admin/openapi.json"),
            (ADMIN_PAGE_PATHS_PERMISSIONS, "/admin/permissions"),
            (ADMIN_PAGE_PATHS_ROLES, "/admin/roles"),
            (ADMIN_PAGE_PATHS_ROOT, "/admin"),
            (ADMIN_PAGE_PATHS_SETTINGS, "/admin/system-settings"),
            (ADMIN_PAGE_PATHS_SIGN_IN, "/admin/sign-in"),
            (ADMIN_PAGE_PATHS_USERS, "/admin/users"),
            (ADMIN_PAGE_PATHS_VERSION, "/admin/version"),
        ]
        .into_iter()
        .all(|(actual, expected)| actual == expected)
    );
}

#[cfg(test)]
#[test]
fn generated_admin_permission_constants_have_expected_values() {
    assert!(
        [
            (ADMIN_PERMISSION_VALUES_AUDIT_LOG_READ, "audit_log:read"),
            (ADMIN_PERMISSION_VALUES_METRICS_READ, "metrics:read"),
            (ADMIN_PERMISSION_VALUES_OPEN_API_READ, "openapi:read"),
            (ADMIN_PERMISSION_VALUES_PERMISSIONS_READ, "permissions:read"),
            (
                ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_CREATE,
                "role_permissions:create",
            ),
            (
                ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_DELETE,
                "role_permissions:delete",
            ),
            (
                ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_READ,
                "role_permissions:read",
            ),
            (
                ADMIN_PERMISSION_VALUES_ROLE_PERMISSIONS_UPDATE,
                "role_permissions:update",
            ),
            (ADMIN_PERMISSION_VALUES_ROLES_CREATE, "roles:create"),
            (ADMIN_PERMISSION_VALUES_ROLES_DELETE, "roles:delete"),
            (ADMIN_PERMISSION_VALUES_ROLES_READ, "roles:read"),
            (ADMIN_PERMISSION_VALUES_ROLES_UPDATE, "roles:update"),
            (
                ADMIN_PERMISSION_VALUES_SYSTEM_SETTINGS_READ,
                "system_settings:read",
            ),
            (
                ADMIN_PERMISSION_VALUES_SYSTEM_SETTINGS_UPDATE,
                "system_settings:update",
            ),
            (
                ADMIN_PERMISSION_VALUES_USER_ROLES_CREATE,
                "user_roles:create",
            ),
            (
                ADMIN_PERMISSION_VALUES_USER_ROLES_DELETE,
                "user_roles:delete",
            ),
            (ADMIN_PERMISSION_VALUES_USER_ROLES_READ, "user_roles:read"),
            (
                ADMIN_PERMISSION_VALUES_USER_ROLES_UPDATE,
                "user_roles:update",
            ),
            (ADMIN_PERMISSION_VALUES_USERS_CREATE, "users:create"),
            (ADMIN_PERMISSION_VALUES_USERS_DELETE, "users:delete"),
            (ADMIN_PERMISSION_VALUES_USERS_READ, "users:read"),
            (ADMIN_PERMISSION_VALUES_USERS_UPDATE, "users:update"),
        ]
        .into_iter()
        .all(|(actual, expected)| actual == expected)
    );
}
