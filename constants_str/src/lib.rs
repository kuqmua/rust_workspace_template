//! Reusable messages, test text, macro diagnostics, and technical string fragments.
//!
//! Domain values are owned by typed APIs: administrator routes and frontend paths by
//! `server_admin_contract` route/path types, permissions by `AdminPermission`, configuration keys
//! by `server_config::config::Config` fields interpreted by `TryFromEnv`, and table column names by the
//! generated table descriptors. The remaining `ENV_NAMES_*` constants support infrastructure and
//! conformance tests; `catalog::SQL_NAMES_ID` is a documented generic SQL-protocol token validated through
//! `pg_crud_common::domain_types::PgSqlIdentifier`, not an application-schema declaration.

pub mod catalog;
pub mod integration_fixtures;
pub mod test_fixtures;

pub const GIT_INFO_PROJECT_GIT_COMMIT_ID: &str =
    git_version::git_version!(args = ["--always", "--abbrev=40"]);
pub const GIT_INFO_PROJECT_GIT_COMMIT_LINK: &str = git_version::git_version!(
    args = ["--always", "--abbrev=40"],
    prefix = "https://github.com/kuqmua/rust_workspace_template/tree/"
);

pub const MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 4] = [
    catalog::SHARED_VALUES_CHECK,
    catalog::SHARED_VALUES_ALL_TARGETS,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_OFFLINE,
];
pub const MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 24] = [
    catalog::CLIPPY,
    catalog::SHARED_VALUES_ALL_TARGETS,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_LOCKED,
    catalog::SHARED_VALUES_OFFLINE,
    catalog::SHARED_VALUES_EMPTY,
    catalog::SHARED_VALUES_D,
    catalog::SHARED_VALUES_WARNINGS,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_BOOL_ASSERT_COMPARISON,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_CLONE_ON_COPY,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_COLLAPSIBLE_IF,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_LET_AND_RETURN,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_RESULT_LARGE_ERR,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_SINGLE_CALL_FN,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_USELESS_BORROWS_IN_FORMATTING,
    catalog::SHARED_VALUES_A,
    catalog::SHARED_VALUES_CLIPPY_WRITE_LITERAL,
];
pub const MACRO_CLIPPY_CARGO_FMT_ARGS: [&str; 1] = [catalog::SHARED_VALUES_FMT];
pub const MACRO_CLIPPY_CARGO_TEST_LIB_ARGS: [&str; 4] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LIB,
    catalog::SHARED_VALUES_LOCKED,
    catalog::SHARED_VALUES_OFFLINE,
];

pub const PG_CRUD_SERDE_BETWEEN_FIELDS: &[&str] =
    &[catalog::PG_CRUD_START_FIELD, catalog::PG_CRUD_END_FIELD];
pub const PG_CRUD_SERDE_PG_TYPE_WHERE_FIELDS: &[&str] =
    &[catalog::PG_CRUD_OPERATOR_FIELD, catalog::PG_CRUD_V_FIELD];

pub const CODE_STYLE_CLIPPY_LINT_EXCEPTIONS: [&str; 0] = [];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_SETS: &[&[&str]] = &[&["identifier", "type0", "vis"]];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_SUFFIXES: [&str; 1] =
    ["macro_helpers/src/syn_field.rs"];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_REASONS: [&str; 1] =
    ["macro generators consume the parsed field descriptor across crate boundaries"];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_STRUCT_NAMES: [&str; 1] = ["SynField"];
pub const CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES: [&str; 10] = [
    "/file_storage/src/adapters.rs",
    "/init_env_files/src/write_content.rs",
    CODE_STYLE_MACRO_CLIPPY_FS_OWNER_SUFFIX,
    CODE_STYLE_MACROS_HELPER_WRITE_STRING_FS_OWNER_SUFFIX,
    "/macro_helpers/src/write_token_stream_into_file.rs",
    "/workspace_scaffold/src/main.rs",
    "/workspace_scaffold/src/template_fs_copy_template_tree.rs",
    "/workspace_scaffold/src/template_fs_insert_once.rs",
    "/workspace_scaffold/src/template_fs_replace_file.rs",
    "/workspace_scaffold/src/template_fs_write_text.rs",
];
pub const CODE_STYLE_DIRECT_FS_OWNER_REASONS: [&str; 10] = [
    "file storage adapter owns persisted file lifecycle operations",
    "environment initializer write adapter owns environment-file writes",
    "macro Clippy fixture builder owns temporary crate filesystem operations",
    "generated string writer owns generated source file comparison and updates",
    "token stream writer owns rustfmt execution for generated source files",
    "workspace scaffold entry point owns command-line parsing and dispatch",
    "workspace scaffold copy adapter owns bounded template traversal and copying",
    "workspace scaffold insertion adapter owns generated file updates",
    "workspace scaffold replacement adapter owns generated file updates",
    "workspace scaffold write adapter owns generated file updates",
];
pub const CODE_STYLE_DOMAIN_FIXTURE_PATH: &str = "../tests/src/domain_type_policy_fixture.rs";
pub const CODE_STYLE_BOUNDED_TYPES_SRC: &str = "../bounded_types/src";
pub const CODE_STYLE_LEPTOS_CRATE: &str = "leptos";
pub const CODE_STYLE_MACRO_CLIPPY_FS_OWNER_SUFFIX: &str = "/macro_clippy_check_common/src/lib.rs";
pub const CODE_STYLE_MACROS_HELPER_TEST_FS_OWNER_SUFFIX: &str = "/macro_helpers/src/test_helper.rs";
pub const CODE_STYLE_MACROS_HELPER_WRITE_STRING_FS_OWNER_SUFFIX: &str =
    "/macro_helpers/src/write_string_into_file.rs";
pub const CODE_STYLE_PRELUDE_MODULE: &str = "prelude";
pub const CODE_STYLE_TESTS_SRC_ROOT: &str = "../tests/src";
pub const CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES: [&str; 0] = [];
pub const CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX: &str =
    "/workspace_scaffold/src/domain_types.rs";
pub const CODE_STYLE_STRING_LITERAL_MACRO_BOUNDARIES: &[&str] = &[
    catalog::SHARED_VALUES_ASSERT,
    catalog::SHARED_VALUES_ASSERT_EQ,
    catalog::SHARED_VALUES_ASSERT_NE,
    catalog::SHARED_VALUES_COMPILE_ERROR,
    catalog::SHARED_VALUES_CONCAT,
    catalog::SHARED_VALUES_DEBUG_ASSERT,
    catalog::SHARED_VALUES_DEBUG_ASSERT_EQ,
    catalog::SHARED_VALUES_DEBUG_ASSERT_NE,
    catalog::SHARED_VALUES_DEFINE_STR_CONSTANTS,
    catalog::SHARED_VALUES_ENV,
    catalog::SHARED_VALUES_EPRINT,
    catalog::SHARED_VALUES_EPRINTLN,
    catalog::CONFIG_TRACING_ERROR,
    catalog::SHARED_VALUES_ERROR_SPAN,
    catalog::SHARED_VALUES_FORMAT,
    catalog::SHARED_VALUES_FORMAT_ARGS,
    catalog::SHARED_VALUES_FORMAT_IDENT,
    catalog::CODE_STYLE_GENERATE_DERIVE_TOKEN_STREAM_BUILDER_MACRO_NAME,
    catalog::CODE_STYLE_GENERATE_PG_TYPES_MACRO_NAME,
    catalog::SHARED_VALUES_GENERATE_SELF_UPPER_CAMEL_CASE_AND_SNAKE_CASE_STR_AND_TOKEN_STREAM,
    catalog::SHARED_VALUES_GENERATE_UPPER_CAMEL_CASE_AND_SNAKE_CASE_STR_AND_TOKEN_STREAM,
    catalog::CODE_STYLE_GENERATE_WHERE_FILTERS_MACRO_NAME,
    catalog::INCLUDE_BYTES,
    catalog::INCLUDE_STR,
    catalog::SHARED_VALUES_IMPL_TO_ERR_STRING_WITH,
    catalog::CONFIG_TRACING_INFO,
    catalog::SHARED_VALUES_INFO_SPAN,
    integration_fixtures::JSON,
    catalog::SHARED_VALUES_JOIN,
    catalog::SERVICE_MODE_MIGRATE,
    catalog::SHARED_VALUES_OPTION_ENV,
    catalog::CODE_STYLE_PANIC_METHOD_NAME,
    catalog::SHARED_VALUES_PARSE_QUOTE,
    catalog::SHARED_VALUES_PRINT,
    catalog::SHARED_VALUES_PRINTLN,
    catalog::SHARED_VALUES_QUERY,
    catalog::SHARED_VALUES_QUERY_AS,
    catalog::SHARED_VALUES_QUERY_SCALAR,
    catalog::SHARED_VALUES_QUOTE,
    catalog::SHARED_VALUES_QUOTE_SPANNED,
    catalog::SELECT_ALT_3,
    catalog::SHARED_VALUES_STRINGIFY,
    catalog::TODO,
    catalog::SHARED_VALUES_TP,
    catalog::CONFIG_TRACING_TRACE,
    catalog::SHARED_VALUES_TRACE_SPAN,
    catalog::UNIMPLEMENTED,
    catalog::SHARED_VALUES_UNREACHABLE,
    catalog::SHARED_VALUES_VIEW,
    catalog::CONFIG_TRACING_WARN,
    catalog::SHARED_VALUES_WARN_SPAN,
    catalog::WRITE_ALT,
    catalog::SHARED_VALUES_WRITELN,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS: [&str; 7] = [
    catalog::CLIPPY,
    catalog::SHARED_VALUES_LOCKED,
    catalog::SHARED_VALUES_ALL_TARGETS,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_EMPTY,
    catalog::SHARED_VALUES_D,
    catalog::SHARED_VALUES_WARNINGS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_FMT_CHECK_ARGS: [&str; 2] =
    [catalog::SHARED_VALUES_FMT, catalog::SHARED_VALUES_CHECK_2];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS: [&str; 7] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_NO_FAIL_FAST,
    catalog::SHARED_VALUES_EMPTY,
    catalog::SHARED_VALUES_IGNORED,
];
pub const NOTIFICATION_SERVICE_TEST_SCHEMA: &str = "notification_service_test";
pub const NOTIFICATION_SERVICE_CREATE_TEST_SCHEMA_SQL: [&str; 2] = [
    "CREATE SCHEMA IF catalog::NOT EXISTS ",
    NOTIFICATION_SERVICE_TEST_SCHEMA,
];
pub const ADMIN_HTML_SAVED_FRAGMENT: &str = "#saved";
pub const WORKSPACE_ADMIN: &str = "Workspace Admin";
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_DOC_ARGS: [&str; 5] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_DOC,
    catalog::SHARED_VALUES_ALL_FEATURES,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TBL_ARGS: [&str; 6] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::P,
    catalog::SHARED_VALUES_GENERATE_PG_TABLE_TEST,
    catalog::SHARED_VALUES_FEATURES,
    catalog::TEST_UTILS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_PG_TYPES_ARGS: [&str; 6] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::P,
    catalog::SHARED_VALUES_GENERATE_PG_TYPES_TEST,
    catalog::SHARED_VALUES_FEATURES,
    catalog::TEST_UTILS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_GEN_WH_FLTS_ARGS: [&str; 6] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::P,
    catalog::SHARED_VALUES_GENERATE_WHERE_FILTERS_TEST,
    catalog::SHARED_VALUES_FEATURES,
    catalog::TEST_UTILS,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_IGNORED_ARGS: [&str; 7] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_NO_FAIL_FAST,
    catalog::SHARED_VALUES_EMPTY,
    catalog::SHARED_VALUES_IGNORED,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_STYLE_ARGS: [&str; 5] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::P,
    catalog::TESTS_ALT,
    catalog::SHARED_VALUES_LIB,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_WORKSPACE_ARGS: [&str; 5] = [
    catalog::TEST_ALT_3,
    catalog::SHARED_VALUES_LOCKED,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_NO_FAIL_FAST,
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_HEAVY_ARGS: [&str; 7] = [
    catalog::NEXTEST,
    catalog::SHARED_VALUES_RUN,
    catalog::SHARED_VALUES_NO_FAIL_FAST,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_P_2,
    catalog::SHARED_VALUES_HEAVY_LOAD,
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_IGNORED_ARGS: [&str; 9] = [
    catalog::NEXTEST,
    catalog::SHARED_VALUES_RUN,
    catalog::SHARED_VALUES_NO_FAIL_FAST,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_P_2,
    catalog::WORKSPACE_TEST_RUNNER_STATIC_WORKSPACE_PROFILE,
    catalog::SHARED_VALUES_RUN_IGNORED,
    catalog::SHARED_VALUES_ONLY,
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_WORKSPACE_ARGS: [&str; 7] = [
    catalog::NEXTEST,
    catalog::SHARED_VALUES_RUN,
    catalog::SHARED_VALUES_NO_FAIL_FAST,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_P_2,
    catalog::WORKSPACE_TEST_RUNNER_STATIC_WORKSPACE_PROFILE,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_AUDIT_ARGS: [&str; 1] =
    [catalog::WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND];
pub const WORKSPACE_TEST_RUNNER_CARGO_DENY_ARGS: [&str; 6] = [
    catalog::WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
    catalog::SHARED_VALUES_CHECK,
    catalog::WORKSPACE_TEST_RUNNER_ADVISORIES_ARG,
    catalog::WORKSPACE_TEST_RUNNER_BANS_ARG,
    catalog::WORKSPACE_TEST_RUNNER_LICENSES_ARG,
    catalog::WORKSPACE_TEST_RUNNER_SOURCES_ARG,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_HACK_ARGS: [&str; 6] = [
    catalog::WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
    catalog::SHARED_VALUES_CHECK,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::WORKSPACE_TEST_RUNNER_FEATURE_POWERSET_ARG,
    catalog::WORKSPACE_TEST_RUNNER_NO_DEV_DEPS_ARG,
    catalog::SHARED_VALUES_LOCKED,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_MACHETE_ARGS: [&str; 1] = [catalog::MACHETE];
pub const WORKSPACE_TEST_RUNNER_CARGO_SEMVER_CHECKS_ARGS: [&str; 1] = [catalog::SEMVER_CHECKS];
pub const WORKSPACE_TEST_RUNNER_CARGO_UDEPS_ARGS: [&str; 6] = [
    catalog::WORKSPACE_TEST_RUNNER_NIGHTLY_ARG,
    catalog::UDEPS,
    catalog::SHARED_VALUES_WORKSPACE,
    catalog::SHARED_VALUES_ALL_TARGETS,
    catalog::SHARED_VALUES_ALL_FEATURES,
    catalog::SHARED_VALUES_LOCKED,
];
pub const WORKSPACE_TEST_RUNNER_STATIC_COMMANDS: [(&str, &[&str]); 3] = [
    (
        catalog::WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_FMT_CHECK_ARGS,
    ),
    (
        catalog::WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS,
    ),
    (
        catalog::WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_STYLE_ARGS,
    ),
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS: [(&str, &[&str]); 2] = [
    (
        catalog::WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_WORKSPACE_ARGS,
    ),
    (
        catalog::WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_DOC_ARGS,
    ),
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS: [(&str, &[&str]); 2] = [
    (
        catalog::WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_NEXTEST_WORKSPACE_ARGS,
    ),
    (
        catalog::WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_DOC_ARGS,
    ),
];
pub const NOTIFICATION_API_TOKEN_REDACTED: &str = "NotificationApiToken([catalog::REDACTED])";
pub const BLOCKING_STD_FS_CALLS: [&str; 13] = [
    "std::fs::canonicalize",
    "std::fs::copy",
    "std::fs::create_dir",
    "std::fs::create_dir_all",
    "std::fs::metadata",
    "std::fs::read",
    "std::fs::read_to_string",
    "std::fs::remove_dir",
    "std::fs::remove_dir_all",
    "std::fs::remove_file",
    "std::fs::rename",
    "std::fs::write",
    "std::fs::File::open",
];
pub const BLOCKING_STD_NET_CALLS: [&str; 3] = [
    "std::net::TcpListener::bind",
    "std::net::TcpStream::connect",
    "std::net::UdpSocket::bind",
];
pub const GIT_PROGRAM: &str = "git";
pub const GIT_LS_FILES_ARGS: [&str; 2] = ["ls-files", "-z"];
pub const FILE_DELETE_STAGING_DIRECTORY: &str = ".delete_staging";
pub const FILE_UPLOAD_STAGING_DIRECTORY: &str = ".upload_staging";
pub const TEST_PATH_TRAVERSAL: &str = "../secret";
pub const HTTP_ACCEPT_QUALITY_PARAMETER: &str = "q";
pub const TEST_SERVICE_USERS_PATH: &str = "/service/users";
pub const TEST_SERVICE_PREFIX: &str = "/service";
pub const TEST_SIGNIN_PATH: &str = "/signin";
pub const TEST_ACCEPT_HTML_JSON_ZERO_QUALITY: &str = "text/html, application/json;q=0";
pub const TEST_TRANSACTION_OPERATION_ERROR: &str = "write";
pub const TEST_TRANSACTION_ROLLBACK_ERROR: &str = "rollback";
pub const TEST_EMPTY_DELIMITED_LIST: &str = " , ";
pub const TEST_NOTIFICATION_API_TOKEN: &str = "secret";
pub const TEST_JWT_SECRET_CHARACTER_A: &str = "a";
pub const TEST_JWT_SECRET_CHARACTER_B: &str = "b";
pub const TEST_MULTIPART_FILE_FIELD: &str = "file";
pub const BEARER: &str = "Bearer";
pub const TEST_COOKIE_NAME: &str = "session";
pub const TEST_DUPLICATE_COOKIE: &str = "session=one; session=two";
pub const TEST_JSON_CONTENT_TYPE_WITH_CHARSET: &str = "application/json; charset=utf-8";
pub const HTTP_SCHEME_PREFIX: &str = "http://";
pub const HTTPS_SCHEME_PREFIX: &str = "https://";
pub const ENCODED_DOT: &str = "%2e";
pub const ENCODED_SLASH: &str = "%2f";
pub const ENCODED_QUERY: &str = "%3f";
