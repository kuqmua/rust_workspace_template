// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks
#![allow(clippy::wildcard_imports)] // split authentication modules share a private facade vocabulary
#[path = "account_change_own_password.rs"]
mod account_change_own_password;
#[path = "account_me.rs"]
mod account_me;
#[path = "account_me_context_view_ref.rs"]
mod account_me_context_view_ref;
#[path = "admin_observed_error_code.rs"]
mod admin_observed_error_code;
#[path = "api_audit_log.rs"]
mod api_audit_log;
#[path = "api_branding.rs"]
mod api_branding;
#[path = "api_change_own_password.rs"]
mod api_change_own_password;
#[path = "api_create_role.rs"]
mod api_create_role;
#[path = "api_create_user.rs"]
mod api_create_user;
#[path = "api_data_table.rs"]
mod api_data_table;
#[path = "api_data_tables.rs"]
mod api_data_tables;
#[path = "api_delete_role.rs"]
mod api_delete_role;
#[path = "api_delete_user.rs"]
mod api_delete_user;
#[path = "api_export_audit_log.rs"]
mod api_export_audit_log;
#[path = "api_list_permissions.rs"]
mod api_list_permissions;
#[path = "api_list_roles.rs"]
mod api_list_roles;
#[path = "api_list_users.rs"]
mod api_list_users;
#[path = "api_me.rs"]
mod api_me;
#[path = "api_refresh.rs"]
mod api_refresh;
#[path = "api_revoke_all_sessions.rs"]
mod api_revoke_all_sessions;
#[path = "api_revoke_session.rs"]
mod api_revoke_session;
#[path = "api_sessions.rs"]
mod api_sessions;
#[path = "api_set_role_permissions.rs"]
mod api_set_role_permissions;
#[path = "api_set_user_ban.rs"]
mod api_set_user_ban;
#[path = "api_set_user_password.rs"]
mod api_set_user_password;
#[path = "api_set_user_roles.rs"]
mod api_set_user_roles;
#[path = "api_settings.rs"]
mod api_settings;
#[path = "api_sign_in.rs"]
mod api_sign_in;
#[path = "api_sign_out.rs"]
mod api_sign_out;
#[path = "api_update_role.rs"]
mod api_update_role;
#[path = "api_update_settings.rs"]
mod api_update_settings;
#[path = "api_update_user.rs"]
mod api_update_user;
#[path = "append_cleared_session_cookies.rs"]
mod append_cleared_session_cookies;
#[path = "append_session_cookies.rs"]
mod append_session_cookies;
#[path = "authn_apply_refresh_failure_delay.rs"]
mod authn_apply_refresh_failure_delay;
#[path = "authn_refresh.rs"]
mod authn_refresh;
#[path = "authn_sign_in.rs"]
mod authn_sign_in;
#[path = "authn_sign_out.rs"]
mod authn_sign_out;
#[path = "authorization_authenticate.rs"]
pub(super) mod authorization_authenticate;
#[path = "authorization_authorize_generated_request.rs"]
pub(crate) mod authorization_authorize_generated_request;
#[path = "authorization_hash_refresh_token_with_context.rs"]
pub(super) mod authorization_hash_refresh_token_with_context;
#[path = "authorization_origin_is_present_and_allowed.rs"]
pub(super) mod authorization_origin_is_present_and_allowed;
#[path = "authorization_session_context_hash.rs"]
pub(super) mod authorization_session_context_hash;
#[path = "authorization_validate_csrf.rs"]
pub(super) mod authorization_validate_csrf;
#[path = "data_tables_get.rs"]
mod data_tables_get;
#[path = "data_tables_list.rs"]
mod data_tables_list;
#[path = "extractors.rs"]
mod extractors;
#[path = "html.rs"]
mod html;
#[path = "jsonwebtoken_admin_encoding_key.rs"]
mod jsonwebtoken_admin_encoding_key;
#[path = "persistence.rs"]
mod persistence;
#[path = "application_roles.rs"]
mod roles;
#[path = "sessions.rs"]
mod sessions;
#[path = "sessions_revoke_all_sessions.rs"]
mod sessions_revoke_all_sessions;
#[path = "sessions_revoke_session.rs"]
mod sessions_revoke_session;
#[path = "settings_branding.rs"]
mod settings_branding;
#[path = "settings_branding_view.rs"]
mod settings_branding_view;
#[path = "settings_branding_view_ref.rs"]
mod settings_branding_view_ref;
#[path = "settings_get.rs"]
mod settings_get;
#[path = "settings_update.rs"]
mod settings_update;
#[path = "shared.rs"]
mod shared;
#[path = "state.rs"]
mod state;
#[path = "application_users.rs"]
mod users;
pub use jsonwebtoken_admin_encoding_key::*;
#[path = "std_admin_access_ttl_seconds.rs"]
mod std_admin_access_ttl_seconds;
pub use std_admin_access_ttl_seconds::*;
#[path = "std_admin_refresh_ttl_seconds.rs"]
mod std_admin_refresh_ttl_seconds;
pub use std_admin_refresh_ttl_seconds::*;
#[path = "std_admin_session_limit.rs"]
mod std_admin_session_limit;
pub use std_admin_session_limit::*;
#[path = "std_admin_failure_threshold.rs"]
mod std_admin_failure_threshold;
pub use std_admin_failure_threshold::*;
#[path = "admin_auth_positive_value_error.rs"]
mod admin_auth_positive_value_error;
pub use admin_auth_positive_value_error::*;
#[path = "std_admin_failure_delay_millis.rs"]
mod std_admin_failure_delay_millis;
pub use std_admin_failure_delay_millis::*;
#[path = "admin_auth_policy.rs"]
mod admin_auth_policy;
pub use admin_auth_policy::*;
#[path = "admin_auth_svc_state.rs"]
mod admin_auth_svc_state;
pub use admin_auth_svc_state::*;
#[path = "shared_admin_auth_svc_state_arc.rs"]
mod shared_admin_auth_svc_state_arc;
pub use shared_admin_auth_svc_state_arc::*;
#[path = "admin_auth_svc_state_build_error.rs"]
mod admin_auth_svc_state_build_error;
pub use admin_auth_svc_state_build_error::*;
#[path = "authenticated_admin.rs"]
mod authenticated_admin;
pub use authenticated_admin::*;
#[path = "admin_audit_query.rs"]
mod admin_audit_query;
pub use admin_audit_query::*;
#[path = "http_admin_header_map.rs"]
mod http_admin_header_map;
pub use http_admin_header_map::*;
#[path = "http_admin_header_value_error.rs"]
mod http_admin_header_value_error;
pub use http_admin_header_value_error::*;
#[path = "axum_admin_response.rs"]
mod axum_admin_response;
pub use axum_admin_response::*;
#[path = "axum_admin_auth_router.rs"]
mod axum_admin_auth_router;
pub use axum_admin_auth_router::*;
#[path = "utoipa_admin_auth_open_api.rs"]
mod utoipa_admin_auth_open_api;
pub use utoipa_admin_auth_open_api::*;
#[path = "admin_auth_html_routes.rs"]
mod admin_auth_html_routes;
pub use admin_auth_html_routes::*;
#[path = "admin_html_swagger_enabled.rs"]
mod admin_html_swagger_enabled;
pub use admin_html_swagger_enabled::*;
#[path = "html_routes_with_swagger.rs"]
mod html_routes_with_swagger;
pub use html_routes_with_swagger::*;
#[path = "admin_session_bundle.rs"]
mod admin_session_bundle;
pub use admin_session_bundle::*;
#[path = "admin_session_error.rs"]
mod admin_session_error;
pub use admin_session_error::*;
#[path = "std_admin_rate_limit_count.rs"]
mod std_admin_rate_limit_count;
pub(crate) use std_admin_rate_limit_count::*;
#[path = "std_admin_rate_limit_window_seconds.rs"]
mod std_admin_rate_limit_window_seconds;
pub(crate) use std_admin_rate_limit_window_seconds::*;
#[path = "admin_audit_query_parts.rs"]
mod admin_audit_query_parts;
pub(crate) use admin_audit_query_parts::*;
#[path = "admin_auth_req.rs"]
mod admin_auth_req;
pub(crate) use admin_auth_req::*;
#[path = "admin_peer_addr.rs"]
mod admin_peer_addr;
pub(crate) use admin_peer_addr::*;
#[path = "admin_sign_in_json.rs"]
mod admin_sign_in_json;
pub(crate) use admin_sign_in_json::*;
#[path = "axum_admin_json.rs"]
mod axum_admin_json;
pub(crate) use axum_admin_json::*;
#[path = "axum_admin_form.rs"]
mod axum_admin_form;
pub(crate) use axum_admin_form::*;
#[path = "axum_admin_path.rs"]
mod axum_admin_path;
pub(crate) use axum_admin_path::*;
#[path = "axum_admin_query.rs"]
mod axum_admin_query;
pub(crate) use axum_admin_query::*;
#[path = "admin_session_path.rs"]
mod admin_session_path;
pub(crate) use admin_session_path::*;
#[path = "admin_error.rs"]
mod admin_error;
pub(crate) use admin_error::*;
#[path = "axum_admin_state_router.rs"]
mod axum_admin_state_router;
pub(crate) use axum_admin_state_router::*;
#[path = "jsonwebtoken_admin_decoding_keys.rs"]
mod jsonwebtoken_admin_decoding_keys;
use jsonwebtoken_admin_decoding_keys::*;
#[path = "std_admin_access_ttl_seconds_non_zero_u64.rs"]
mod std_admin_access_ttl_seconds_non_zero_u64;
use std_admin_access_ttl_seconds_non_zero_u64::*;
#[path = "std_admin_refresh_ttl_seconds_non_zero_u64.rs"]
mod std_admin_refresh_ttl_seconds_non_zero_u64;
use std_admin_refresh_ttl_seconds_non_zero_u64::*;
#[path = "std_admin_session_limit_non_zero_usize.rs"]
mod std_admin_session_limit_non_zero_usize;
use std_admin_session_limit_non_zero_usize::*;
#[path = "admin_password_from_contract.rs"]
mod admin_password_from_contract;
use admin_password_from_contract::*;
#[path = "admin_new_password_from_contract.rs"]
mod admin_new_password_from_contract;
use admin_new_password_from_contract::*;
#[path = "authenticated_admin_contract.rs"]
mod authenticated_admin_contract;
use authenticated_admin_contract::*;
#[path = "admin_error_response_parts.rs"]
mod admin_error_response_parts;
use admin_error_response_parts::*;
pub use routes::{admin_api_open_api, routes};

frontend_contract::domain_types::api_operation_error!(AdminAuditLogError,);
frontend_contract::domain_types::api_operation_error!(AdminAuditExportError,);
frontend_contract::domain_types::api_operation_error!(AdminBrandingError,);
frontend_contract::domain_types::api_operation_error!(AdminChangeOwnPasswordError,);
frontend_contract::domain_types::api_operation_error!(AdminCreateRoleError,);
frontend_contract::domain_types::api_operation_error!(AdminCreateUserError,);
frontend_contract::domain_types::api_operation_error!(AdminDataTableError,);
frontend_contract::domain_types::api_operation_error!(AdminDataTablesError,);
frontend_contract::domain_types::api_operation_error!(AdminDeleteRoleError,);
frontend_contract::domain_types::api_operation_error!(AdminDeleteUserError,);
frontend_contract::domain_types::api_operation_error!(AdminListPermissionsError,);
frontend_contract::domain_types::api_operation_error!(AdminListRolesError,);
frontend_contract::domain_types::api_operation_error!(AdminListUsersError,);
frontend_contract::domain_types::api_operation_error!(AdminMeError,);
frontend_contract::domain_types::api_operation_error!(AdminRefreshError,);
frontend_contract::domain_types::api_operation_error!(AdminRevokeAllSessionsError,);
frontend_contract::domain_types::api_operation_error!(AdminRevokeSessionError,);
frontend_contract::domain_types::api_operation_error!(AdminSessionsError,);
frontend_contract::domain_types::api_operation_error!(AdminSetRolePermissionsError,);
frontend_contract::domain_types::api_operation_error!(AdminSetUserBanError,);
frontend_contract::domain_types::api_operation_error!(AdminSetUserPasswordError,);
frontend_contract::domain_types::api_operation_error!(AdminSetUserRolesError,);
frontend_contract::domain_types::api_operation_error!(AdminSettingsError,);
frontend_contract::domain_types::api_operation_error!(AdminSignInError,);
frontend_contract::domain_types::api_operation_error!(AdminSignOutError,);
frontend_contract::domain_types::api_operation_error!(AdminUpdateRoleError,);
frontend_contract::domain_types::api_operation_error!(AdminUpdateSettingsError,);
frontend_contract::domain_types::api_operation_error!(AdminUpdateUserError,);
#[path = "audit_export_log.rs"]
mod audit_export_log;
#[path = "audit_query_log.rs"]
mod audit_query_log;
#[path = "create_session_in_connection.rs"]
mod create_session_in_connection;
#[path = "rate_limit.rs"]
mod rate_limit;
#[path = "routes.rs"]
mod routes;
#[cfg(test)]
#[path = "application_tests.rs"]
mod tests;
