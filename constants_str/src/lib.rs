//! Reusable messages, test text, macro diagnostics, and technical string fragments.
//!
//! Domain values are owned by typed APIs: administrator routes and frontend paths by
//! `server_admin_contract` route/path types, permissions by `AdminPermission`, configuration keys
//! by `server_config::domain_types::Config` fields interpreted by `TryFromEnv`, and table column names by the
//! generated table descriptors. The remaining `ENV_NAMES_*` constants support infrastructure and
//! conformance tests; `SQL_NAMES_ID` is a documented generic SQL-protocol token validated through
//! `pg_crud_common::domain_types::PgSqlIdentifier`, not an application-schema declaration.

mod catalog;
mod integration_fixtures;
mod test_fixtures;

pub use catalog::*;
pub use integration_fixtures::*;
pub use test_fixtures::*;

pub const GIT_INFO_PROJECT_GIT_COMMIT_ID: &str =
    git_version::git_version!(args = ["--always", "--abbrev=40"]);
pub const GIT_INFO_PROJECT_GIT_COMMIT_LINK: &str = git_version::git_version!(
    args = ["--always", "--abbrev=40"],
    prefix = "https://github.com/kuqmua/rust_workspace_template/tree/"
);

pub const MACRO_CLIPPY_CARGO_CHECK_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 4] = [
    SHARED_VALUES_CHECK,
    SHARED_VALUES_ALL_TARGETS,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_OFFLINE,
];
pub const MACRO_CLIPPY_CARGO_CLIPPY_ALL_TARGETS_ALL_FEATURES_ARGS: [&str; 24] = [
    CLIPPY,
    SHARED_VALUES_ALL_TARGETS,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_OFFLINE,
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
pub const MACRO_CLIPPY_CARGO_TEST_LIB_ARGS: [&str; 4] = [
    TEST_ALT_3,
    SHARED_VALUES_LIB,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_OFFLINE,
];

pub const PG_CRUD_SERDE_BETWEEN_FIELDS: &[&str] = &[PG_CRUD_START_FIELD, PG_CRUD_END_FIELD];
pub const PG_CRUD_SERDE_PG_TYPE_WHERE_FIELDS: &[&str] = &[PG_CRUD_OPERATOR_FIELD, PG_CRUD_V_FIELD];

pub const CODE_STYLE_CLIPPY_LINT_EXCEPTIONS: [&str; 0] = [];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_SETS: &[&[&str]] = &[
    &["file", "line", "column"],
    &["secs", "nanos"],
    &["identifier", "type0", "vis"],
    &[
        "bulk_item_budget",
        "config",
        "idempotency_response_budget",
        "pg_pool",
        "project_git_info",
    ],
    &["greater_than", "create", "variant"],
    &["create", "variant", "len_greater_than"],
    &["column", "order"],
    &["v"],
    &[
        "cors_allow_origin",
        "content_security_policy",
        "database_url",
        "admin_jwt_secret",
        "admin_token_audience",
        "admin_token_issuer",
        "trusted_proxy_ranges_text",
        "admin_access_token_ttl_seconds",
        "admin_login_failure_limit",
        "admin_password_hash_concurrency",
        "admin_refresh_token_ttl_seconds",
        "admin_session_limit",
        "admin_sign_in_rate_limit",
        "pg_pool_acquire_timeout_seconds",
        "pg_pool_idle_timeout_seconds",
        "pg_pool_max_lifetime_seconds",
        "request_timeout_seconds",
        "maximum_size_of_http_body_in_bytes",
        "service_socket_address",
        "pg_pool_max_connections",
        "pg_pool_min_connections",
        "timezone",
        "src_place_type",
        "tracing_level",
        "tracing_format",
        "enable_api_git_commit_check",
        "admin_cookie_secure",
        "admin_swagger_enabled",
        "http_gzip_enabled",
        "production_mode",
        "svc_mode",
    ],
    &["id", "user_id", "role_id", "created_at"],
    &["id", "role_id", "permission_id", "created_at"],
    &["id", "name", "is_system", "created_at", "updated_at"],
    &["id", "name", "created_at"],
    &[
        "id",
        "site_name",
        "tab_title",
        "main_logo",
        "primary_color",
        "default_admin_route",
        "organization_name",
        "organization_contacts",
        "support_url",
        "updated_at",
    ],
];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_PATH_SUFFIXES: [&str; 14] = [
    "location_lib/src/domain_types.rs",
    "location_lib/src/domain_types.rs",
    "macro_helpers/src/syn_field.rs",
    "server_app_state/src/domain_types.rs",
    "pg_crud_common/src/domain_types.rs",
    "pg_crud_common/src/domain_types.rs",
    "pg_crud_common/src/query_pagination.rs",
    "pg_crud_common/src/query_collections.rs",
    "server_config/src/domain_types.rs",
    "server_admin/src/generated_tables.rs",
    "server_admin/src/generated_tables.rs",
    "server_admin/src/generated_tables.rs",
    "server_admin/src/generated_tables.rs",
    "server_admin/src/generated_tables.rs",
];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_REASONS: [&str; 14] = [
    "location proc-macro output exposes occurrence coordinates as its public data contract",
    "serialized duration representation is a public wire-format helper",
    "macro generators consume the parsed field descriptor across crate boundaries",
    "Axum state consumers in service crates require direct access to shared immutable state",
    "public generator test descriptor is constructed by downstream generated test crates",
    "public generator test descriptor is constructed by downstream generated test crates",
    "generated query code constructs this public typed ordering contract",
    "generated filter code constructs this public generic value contract",
    "service entry points consume the validated immutable workspace configuration contract",
    "generated database row model is a public serialization and query contract",
    "generated database row model is a public serialization and query contract",
    "generated database row model is a public serialization and query contract",
    "generated database row model is a public serialization and query contract",
    "generated database row model is a public serialization and query contract",
];
pub const CODE_STYLE_REVIEWED_PUBLIC_FIELD_STRUCT_NAMES: [&str; 14] = [
    "Occr",
    "StdTimeDuration",
    "SynField",
    "ServerAppState",
    "PgTypeGreaterThanTest",
    "PgTypeLenGreaterThanTest",
    "OrderBy",
    "V",
    "Config",
    "AdminUserRoles",
    "AdminRolePermissions",
    "AdminRoles",
    "AdminPermissions",
    "AdminSystemSettings",
];
pub const CODE_STYLE_DIRECT_FS_OWNER_SUFFIXES: [&str; 15] = [
    "/config_lib/src/domain_types.rs",
    "/config_lib/src/types.rs",
    "/file_storage/src/adapters.rs",
    "/file_storage/src/domain_types.rs",
    "/init_env_files/src/domain_types.rs",
    "/init_env_files/src/adapters.rs",
    "/init_env_files/src/run.rs",
    CODE_STYLE_MACRO_CLIPPY_FS_OWNER_SUFFIX,
    CODE_STYLE_MACROS_HLP_TEST_FS_OWNER_SUFFIX,
    CODE_STYLE_MACROS_HLP_WRITE_STRING_FS_OWNER_SUFFIX,
    "/macro_helpers/src/write_token_stream_into_file.rs",
    "/administrator_account_initialization_and_password_reset/src/application.rs",
    "/workspace_scaffold/src/main.rs",
    CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX,
    CODE_STYLE_WORKSPACE_SCAFFOLD_TEMPLATE_FS_OWNER_SUFFIX,
];
pub const CODE_STYLE_DIRECT_FS_OWNER_REASONS: [&str; 15] = [
    "configuration loader owns process environment and configuration file access",
    "configuration domain types own environment-backed initialization",
    "file storage adapter owns persisted file lifecycle operations",
    "file storage domain unit tests exercise persistence behavior with temporary directories",
    "environment initializer domain unit tests own temporary filesystem fixtures",
    "environment initializer adapter owns bounded reads and environment file writes",
    "environment initializer application owns command-line mode selection",
    "macro Clippy fixture builder owns temporary crate filesystem operations",
    "macro helper test fixture owns deterministic temporary file assertions",
    "generated string writer owns generated source file comparison and updates",
    "token stream writer owns rustfmt execution for generated source files",
    "initial administrator creation command owns its bounded command-line input",
    "workspace scaffold entry point owns command-line parsing and dispatch",
    "workspace scaffold command owns generated projection and catalog writes",
    "workspace scaffold template filesystem module owns bounded template traversal and copying",
];
pub const CODE_STYLE_DOMAIN_FIXTURE_PATH: &str = "../tests/src/domain_type_policy_fixture.rs";
pub const CODE_STYLE_BOUNDED_TYPES_SRC: &str = "../bounded_types/src";
pub const CODE_STYLE_LOCATION_TEST_SRC: &str = "../location_lib_location_test/src";
pub const CODE_STYLE_LOCATION_TEST_REASON: &str = "location macro fixture deliberately exposes raw Vec fields required by the macro input contract";
pub const CODE_STYLE_PG_CRUD_COMMON_BENCHES: &str = "../pg_crud_common/benches";
pub const CODE_STYLE_PG_CRUD_COMMON_BENCHES_REASON: &str =
    "benchmark-only boundaries are outside the production domain API";
pub const CODE_STYLE_LEPTOS_CRATE: &str = "leptos";
pub const CODE_STYLE_MACRO_CLIPPY_FS_OWNER_SUFFIX: &str = "/macro_clippy_check_common/src/lib.rs";
pub const CODE_STYLE_MACROS_HLP_TEST_FS_OWNER_SUFFIX: &str = "/macro_helpers/src/test_hlp.rs";
pub const CODE_STYLE_MACROS_HLP_WRITE_STRING_FS_OWNER_SUFFIX: &str =
    "/macro_helpers/src/write_string_into_file.rs";
pub const CODE_STYLE_PRELUDE_MODULE: &str = "prelude";
pub const CODE_STYLE_TEST_CRATE_NAMES: [&str; 6] = [
    SHARED_VALUES_GENERATE_PG_TABLE_TEST,
    SHARED_VALUES_GENERATE_PG_TYPES_TEST,
    SHARED_VALUES_GENERATE_WHERE_FILTERS_TEST,
    "location_test",
    "tests",
    "workspace_test_runner",
];
pub const CODE_STYLE_TEST_CRATE_REASONS: [&str; 6] = [
    "generated PG table validation crate is test-only",
    "generated PG type validation crate is test-only",
    "generated where-filter validation crate is test-only",
    "location macro contract fixture is test-only",
    "code-style and integration analyzer crate is test-only",
    "workspace orchestration runner executes tests and measurements only",
];
pub const CODE_STYLE_TESTS_SRC_ROOT: &str = "../tests/src";
pub const CODE_STYLE_UNBOUNDED_READ_OWNER_SUFFIXES: [&str; 0] = [];
pub const CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX: &str =
    "/workspace_scaffold/src/domain_types.rs";
pub const CODE_STYLE_WORKSPACE_SCAFFOLD_TEMPLATE_FS_OWNER_SUFFIX: &str =
    "/workspace_scaffold/src/template_fs.rs";
pub const CODE_STYLE_ROUTE_VALIDATORS_TEST_HLP_SUFFIX: &str = "/route_validators/src/test_hlp.rs";
pub const CODE_STYLE_RUNTIME_TEST_HELPER_SUFFIXES: [&str; 2] = [
    CODE_STYLE_MACROS_HLP_TEST_FS_OWNER_SUFFIX,
    CODE_STYLE_ROUTE_VALIDATORS_TEST_HLP_SUFFIX,
];
pub const CODE_STYLE_RUNTIME_TEST_HELPER_REASONS: [&str; 2] = [
    "macro helper assertions intentionally panic on deterministic test-fixture failures",
    "route validator test fixtures intentionally panic on invalid local test setup",
];
pub const CODE_STYLE_RUNTIME_ARC_OWNER_SUFFIXES: [&str; 7] = [
    "notification_service/src/routes.rs",
    "server/src/mk_pg_pool.rs",
    SERVER_SRC_APPLICATION_RS,
    SERVER_SRC_APPLICATION_ADMIN_API_RS,
    SERVER_ADMIN_SRC_PASSWORD_RS,
    SERVER_RUNTIME_SRC_BOUNDED_READ_RS,
    SERVER_RUNTIME_SRC_LIMITS_RS,
];
pub const CODE_STYLE_RUNTIME_ARC_OWNER_REASONS: [&str; 7] = [
    "notification service composition shares immutable application state across request tasks",
    "server startup shares immutable application state across request tasks",
    "server lifecycle shares immutable shutdown state across tasks",
    "administrator API composition shares immutable application state across request tasks",
    "password hashing shares the cross-thread concurrency limit",
    "bounded reads share a Tokio semaphore across asynchronous readers",
    "runtime limits share immutable concurrency budgets across tasks",
];
pub const CODE_STYLE_FACADE_REEXPORT_SUFFIXES: [&str; 14] = [
    "bounded_types/src/lib.rs",
    "config_lib/src/domain_types.rs",
    "constants_str/src/lib.rs",
    FRONTEND_CONTRACT_SRC_LIB_RS,
    PG_CRUD_PG_CRUD_COMMON_SRC_LIB_RS,
    VALUE_1ACC98BE,
    PG_CRUD_PG_TABLE_GENERATE_PG_TABLE_SRC_SRC_LIB_RS,
    PG_CRUD_PG_TYPES_GENERATE_PG_TYPES_SRC_SRC_LIB_RS,
    PG_CRUD_WHERE_FILTERS_GENERATE_WHERE_FILTERS_SRC_SRC_LIB_RS,
    "server_admin_contract/src/domain_types.rs",
    SERVER_ADMIN_SRC_LIB_RS,
    "server_observability/src/lib.rs",
    "server_runtime_core/src/domain_types.rs",
    "server_runtime_http/src/domain_types.rs",
];
pub const CODE_STYLE_FACADE_REEXPORT_REASONS: [&str; 14] = [
    "bounded types facade exports validated string and collection families",
    "configuration domain facade exports its public typed configuration API",
    "string constants facade preserves the workspace-wide constant paths across responsibility modules",
    "frontend contract facade exports its public transport API",
    "PG CRUD common facade exports shared domain primitives",
    "PG CRUD macro common facade preserves its public token-generation API across responsibility modules",
    "PG table generator facade exports source pipeline entrypoints",
    "PG types generator facade exports source pipeline entrypoints",
    "where-filter generator facade exports source pipeline entrypoints",
    "server administrator contract facade preserves its public typed API",
    "server administrator facade exports its public service API",
    "server observability facade exports tracing and diagnostic primitives",
    "server runtime core facade exports dependency-light runtime primitives",
    "server HTTP runtime facade exports HTTP and integration primitives",
];
pub const CODE_STYLE_LEPTOS_PRELUDE_SUFFIXES: [&str; 58] = [
    SERVER_ADMIN_FRONTEND_SRC_APP_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_DATA_GRID_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_NAVIGATION_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_PAGINATION_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_PERMISSIONS_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_PROFILE_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_PROFILE_ACCOUNT_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_PROFILE_PASSWORD_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_ROLES_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_ROLES_ROW_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_SESSIONS_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_SETTINGS_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_SHELL_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_USERS_RS,
    SERVER_ADMIN_FRONTEND_SRC_APP_USERS_ROW_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_ADMIN_TABLE_CELLS_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_DATA_GRID_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_DATA_GRID_COLUMN_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_DATA_GRID_COLUMN_FILTER_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_DATA_GRID_COLUMN_FILTER_OPTION_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_DATA_GRID_COLUMN_FILTER_OPTION_RANGE_END_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_DATA_GRID_COLUMN_FILTER_OPTION_VALUE_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_DATA_GRID_ROW_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_SETTINGS_INPUT_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_SETTINGS_INPUT_TEXT_RS,
    SERVER_ADMIN_FRONTEND_SRC_SHARED_SETTINGS_INPUT_TEXTAREA_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_DATA_TABLES_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_DATA_TABLES_CSR_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_DATA_TABLES_SSR_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_DOCUMENT_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_DOCUMENT_PAGE_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_DOCUMENT_PAGE_NAVIGATION_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_DOCUMENT_SIGN_IN_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_CRUD_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_PERMISSIONS_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_PROFILE_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_ROLES_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_ROLES_ROW_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_SESSIONS_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_SETTINGS_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_TABLE_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_TEXT_PAGE_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_USERS_RS,
    SERVER_ADMIN_FRONTEND_SRC_SSR_USERS_ROW_RS,
    "server_admin_frontend/src/domain_types_with_owner_alert.rs",
    "server_admin_frontend/src/domain_types_with_owner_admin_alert_dialog.rs",
    "server_admin_frontend/src/domain_types_with_owner_badge.rs",
    "server_admin_frontend/src/domain_types_with_owner_button.rs",
    "server_admin_frontend/src/domain_types_with_owner_card.rs",
    "server_admin_frontend/src/domain_types_with_owner_admin_checkbox.rs",
    "server_admin_frontend/src/domain_types_with_owner_admin_empty.rs",
    "server_admin_frontend/src/domain_types_with_owner_field.rs",
    "server_admin_frontend/src/domain_types_with_owner_input.rs",
    "server_admin_frontend/src/domain_types_with_owner_navigation.rs",
    "server_admin_frontend/src/domain_types_with_owner_admin_spinner.rs",
    "server_admin_frontend/src/domain_types_with_owner_table.rs",
    "server_admin_frontend/src/domain_types_with_owner_admin_textarea.rs",
    SSR_SOURCE_PATH,
];
pub const CODE_STYLE_LEPTOS_PRELUDE_REASONS: [&str; 58] = [
    "Leptos CSR view macro expansion requires attribute traits in lexical scope",
    "Leptos CSR data-grid component requires attribute traits in lexical scope",
    "Leptos CSR navigation component requires attribute traits in lexical scope",
    "Leptos CSR pagination component requires attribute traits in lexical scope",
    "Leptos CSR permissions component requires attribute traits in lexical scope",
    "Leptos CSR profile component requires attribute traits in lexical scope",
    "Leptos CSR profile account card requires attribute traits in lexical scope",
    "Leptos CSR change-password card requires attribute traits in lexical scope",
    "Leptos CSR roles component requires attribute traits in lexical scope",
    "Leptos CSR role-row rendering requires attribute traits in lexical scope",
    "Leptos CSR sessions component requires attribute traits in lexical scope",
    "Leptos CSR settings component requires attribute traits in lexical scope",
    "Leptos CSR shell component requires attribute traits in lexical scope",
    "Leptos CSR users component requires attribute traits in lexical scope",
    "Leptos CSR user-row rendering requires attribute traits in lexical scope",
    "shared administrator table value cells require attribute traits in lexical scope",
    "shared Leptos data-grid rendering requires attribute traits in lexical scope",
    "shared Leptos data-grid column rendering requires attribute traits in lexical scope",
    "shared Leptos data-grid column filter requires attribute traits in lexical scope",
    "shared Leptos data-grid filter option requires attribute traits in lexical scope",
    "shared Leptos data-grid range-end control requires attribute traits in lexical scope",
    "shared Leptos data-grid value control requires attribute traits in lexical scope",
    "shared Leptos data-grid row rendering requires attribute traits in lexical scope",
    "shared Leptos settings input rendering requires attribute traits in lexical scope",
    "shared Leptos settings text input requires attribute traits in lexical scope",
    "shared Leptos settings textarea requires attribute traits in lexical scope",
    "Leptos SSR data-table rendering requires attribute traits in lexical scope",
    "Leptos SSR CSR loading-shell rendering requires attribute traits in lexical scope",
    "Leptos SSR data-table page rendering requires attribute traits in lexical scope",
    "Leptos SSR document rendering requires attribute traits in lexical scope",
    "Leptos SSR administrator page rendering requires attribute traits in lexical scope",
    "Leptos SSR administrator navigation requires attribute traits in lexical scope",
    "Leptos SSR sign-in rendering requires attribute traits in lexical scope",
    "Leptos SSR CRUD rendering requires attribute traits in lexical scope",
    "Leptos SSR permissions rendering requires attribute traits in lexical scope",
    "Leptos SSR profile rendering requires attribute traits in lexical scope",
    "Leptos SSR roles rendering requires attribute traits in lexical scope",
    "Leptos SSR role-row rendering requires attribute traits in lexical scope",
    "Leptos SSR sessions rendering requires attribute traits in lexical scope",
    "Leptos SSR settings rendering requires attribute traits in lexical scope",
    "Leptos SSR table rendering requires attribute traits in lexical scope",
    "Leptos SSR text-page rendering requires attribute traits in lexical scope",
    "Leptos SSR users rendering requires attribute traits in lexical scope",
    "Leptos SSR user-row rendering requires attribute traits in lexical scope",
    "Leptos alert primitive requires attribute traits in lexical scope",
    "Leptos alert-dialog primitive requires attribute traits in lexical scope",
    "Leptos badge primitive requires attribute traits in lexical scope",
    "Leptos button primitive requires attribute traits in lexical scope",
    "Leptos card primitive requires attribute traits in lexical scope",
    "Leptos checkbox primitive requires attribute traits in lexical scope",
    "Leptos empty-state primitive requires attribute traits in lexical scope",
    "Leptos field primitive requires attribute traits in lexical scope",
    "Leptos input primitive requires attribute traits in lexical scope",
    "Leptos navigation primitive requires attribute traits in lexical scope",
    "Leptos spinner primitive requires attribute traits in lexical scope",
    "Leptos table primitives require attribute traits in lexical scope",
    "Leptos textarea primitive requires attribute traits in lexical scope",
    "Leptos SSR view macro expansion requires attribute traits in lexical scope",
];
pub const CODE_STYLE_SINGLE_SOURCE_OWNER_SUFFIXES: [&str; 6] = [
    SERVER_RUNTIME_SRC_BOUNDED_READ_RS,
    PG_CRUD_PG_CRUD_COMMON_SRC_SQL_IDENTIFIER_RS,
    PG_CRUD_COMMON_SRC_PG_ERROR_RS,
    MACRO_HELPERS_SRC_TOOL_COMMAND_RS,
    STR_CONSTANTS_SRC_LIB_RS,
    CODE_STYLE_WORKSPACE_SCAFFOLD_FS_OWNER_SUFFIX,
];
pub const CODE_STYLE_SINGLE_SOURCE_OWNER_REASONS: [&str; 6] = [
    "bounded-read implementation necessarily performs the underlying bounded filesystem read",
    "SQL identifier wrapper owns validation and SQL identifier vocabulary",
    "PostgreSQL error classifier centrally owns SQLSTATE interpretation",
    "tool command wrapper centrally owns process command construction",
    "string constant crate centrally owns reusable production string constants",
    "workspace scaffold owns embedded generated SQL and deployment templates",
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
    SERVICE_MODE_MIGRATE,
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
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_DATABASE_ARGS: [&str; 7] = [
    TEST_ALT_3,
    SHARED_VALUES_LOCKED,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_NO_FAIL_FAST,
    SHARED_VALUES_EMPTY,
    SHARED_VALUES_IGNORED,
];
pub const NOTIFICATION_SERVICE_TEST_SCHEMA: &str = "notification_service_test";
pub const NOTIFICATION_SERVICE_CREATE_TEST_SCHEMA_SQL: [&str; 2] = [
    "CREATE SCHEMA IF NOT EXISTS ",
    NOTIFICATION_SERVICE_TEST_SCHEMA,
];
pub const ADMIN_HTML_SAVED_FRAGMENT: &str = "#saved";
pub const WORKSPACE_ADMIN: &str = "Workspace Admin";
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
pub const WORKSPACE_TEST_RUNNER_CARGO_AUDIT_ARGS: [&str; 1] =
    [WORKSPACE_TEST_RUNNER_AUDIT_SUBCOMMAND];
pub const WORKSPACE_TEST_RUNNER_CARGO_DENY_ARGS: [&str; 6] = [
    WORKSPACE_TEST_RUNNER_DENY_SUBCOMMAND,
    SHARED_VALUES_CHECK,
    WORKSPACE_TEST_RUNNER_ADVISORIES_ARG,
    WORKSPACE_TEST_RUNNER_BANS_ARG,
    WORKSPACE_TEST_RUNNER_LICENSES_ARG,
    WORKSPACE_TEST_RUNNER_SOURCES_ARG,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_HACK_ARGS: [&str; 6] = [
    WORKSPACE_TEST_RUNNER_HACK_SUBCOMMAND,
    SHARED_VALUES_CHECK,
    SHARED_VALUES_WORKSPACE,
    WORKSPACE_TEST_RUNNER_FEATURE_POWERSET_ARG,
    WORKSPACE_TEST_RUNNER_NO_DEV_DEPS_ARG,
    SHARED_VALUES_LOCKED,
];
pub const WORKSPACE_TEST_RUNNER_CARGO_MACHETE_ARGS: [&str; 1] = [MACHETE];
pub const WORKSPACE_TEST_RUNNER_CARGO_SEMVER_CHECKS_ARGS: [&str; 1] = [SEMVER_CHECKS];
pub const WORKSPACE_TEST_RUNNER_CARGO_UDEPS_ARGS: [&str; 6] = [
    WORKSPACE_TEST_RUNNER_NIGHTLY_ARG,
    UDEPS,
    SHARED_VALUES_WORKSPACE,
    SHARED_VALUES_ALL_TARGETS,
    SHARED_VALUES_ALL_FEATURES,
    SHARED_VALUES_LOCKED,
];
pub const WORKSPACE_TEST_RUNNER_STATIC_COMMANDS: [(&str, &[&str]); 3] = [
    (
        WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_FMT_CHECK_ARGS,
    ),
    (
        WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_CLIPPY_ARGS,
    ),
    (
        WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_STYLE_ARGS,
    ),
];
pub const WORKSPACE_TEST_RUNNER_CARGO_TEST_COMMANDS: [(&str, &[&str]); 2] = [
    (
        WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_WORKSPACE_ARGS,
    ),
    (
        WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_DOC_ARGS,
    ),
];
pub const WORKSPACE_TEST_RUNNER_NEXTEST_COMMANDS: [(&str, &[&str]); 2] = [
    (
        WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_NEXTEST_WORKSPACE_ARGS,
    ),
    (
        WORKSPACE_TEST_RUNNER_CARGO,
        &WORKSPACE_TEST_RUNNER_CARGO_TEST_DOC_ARGS,
    ),
];
pub const NOTIFICATION_API_TOKEN_REDACTED: &str = "NotificationApiToken([REDACTED])";
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
