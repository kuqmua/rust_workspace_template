// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks
#![allow(clippy::wildcard_imports)] // split authentication modules share a private facade vocabulary
pub use admin_audit_query::*;
pub(crate) use admin_audit_query_parts::*;
pub use admin_auth_html_routes::*;
pub use admin_auth_policy::*;
pub use admin_auth_positive_value_error::*;
pub(crate) use admin_auth_req::*;
pub use admin_auth_svc_state::*;
pub use admin_auth_svc_state_build_error::*;
pub(crate) use admin_error::*;
use admin_error_response_parts::*;
pub use admin_html_swagger_enabled::*;
use admin_new_password_from_contract::*;
use admin_password_from_contract::*;
pub(crate) use admin_peer_addr::*;
pub use admin_session_bundle::*;
pub use admin_session_error::*;
pub(crate) use admin_session_path::*;
pub(crate) use admin_sign_in_json::*;
pub use authenticated_admin::*;
use authenticated_admin_contract::*;
pub use axum_admin_auth_router::*;
pub(crate) use axum_admin_form::*;
pub(crate) use axum_admin_json::*;
pub(crate) use axum_admin_path::*;
pub(crate) use axum_admin_query::*;
pub use axum_admin_response::*;
pub(crate) use axum_admin_state_router::*;
pub use html_routes_with_swagger::*;
pub use http_admin_header_map::*;
pub use http_admin_header_value_error::*;
use jsonwebtoken_admin_decoding_keys::*;
pub use jsonwebtoken_admin_encoding_key::*;
pub use routes::{admin_api_open_api, admin_auth_routes};
pub use shared_admin_auth_svc_state_arc::*;
pub use std_admin_access_ttl_seconds::*;
pub use std_admin_failure_delay_millis::*;
pub use std_admin_failure_threshold::*;
pub(crate) use std_admin_rate_limit_count::*;
pub(crate) use std_admin_rate_limit_window_seconds::*;
pub use std_admin_refresh_ttl_seconds::*;
pub use std_admin_session_limit::*;
pub use utoipa_admin_auth_open_api::*;

frontend_contract::api_operation_error!(AdminAuditLogError,);
frontend_contract::api_operation_error!(AdminAuditExportError,);
frontend_contract::api_operation_error!(AdminBrandingError,);
frontend_contract::api_operation_error!(AdminChangeOwnPasswordError,);
frontend_contract::api_operation_error!(AdminCreateRoleError,);
frontend_contract::api_operation_error!(AdminCreateUserError,);
frontend_contract::api_operation_error!(AdminDataTableError,);
frontend_contract::api_operation_error!(AdminDataTablesError,);
frontend_contract::api_operation_error!(AdminDeleteRoleError,);
frontend_contract::api_operation_error!(AdminDeleteUserError,);
frontend_contract::api_operation_error!(AdminListPermissionsError,);
frontend_contract::api_operation_error!(AdminListRolesError,);
frontend_contract::api_operation_error!(AdminListUsersError,);
frontend_contract::api_operation_error!(AdminMeError,);
frontend_contract::api_operation_error!(AdminRefreshError,);
frontend_contract::api_operation_error!(AdminRevokeAllSessionsError,);
frontend_contract::api_operation_error!(AdminRevokeSessionError,);
frontend_contract::api_operation_error!(AdminSessionsError,);
frontend_contract::api_operation_error!(AdminSetRolePermissionsError,);
frontend_contract::api_operation_error!(AdminSetUserBanError,);
frontend_contract::api_operation_error!(AdminSetUserPasswordError,);
frontend_contract::api_operation_error!(AdminSetUserRolesError,);
frontend_contract::api_operation_error!(AdminSettingsError,);
frontend_contract::api_operation_error!(AdminSignInError,);
frontend_contract::api_operation_error!(AdminSignOutError,);
frontend_contract::api_operation_error!(AdminUpdateRoleError,);
frontend_contract::api_operation_error!(AdminUpdateSettingsError,);
frontend_contract::api_operation_error!(AdminUpdateUserError,);

// Root-owned module compatibility wrappers.
mod account_change_own_password {
    pub use crate::account_change_own_password::*;
}
mod account_me {
    pub use crate::account_me::*;
}
mod account_me_context_view_ref {
    pub use crate::account_me_context_view_ref::*;
}
mod admin_observed_error_code {
    pub use crate::admin_observed_error_code::*;
}
mod api_audit_log {
    pub use crate::api_audit_log::*;
}
mod api_branding {
    pub use crate::api_branding::*;
}
mod api_change_own_password {
    pub use crate::api_change_own_password::*;
}
mod api_create_role {
    pub use crate::api_create_role::*;
}
mod api_create_user {
    pub use crate::api_create_user::*;
}
mod api_data_table {
    pub use crate::api_data_table::*;
}
mod api_data_tables {
    pub use crate::api_data_tables::*;
}
mod api_delete_role {
    pub use crate::api_delete_role::*;
}
mod api_delete_user {
    pub use crate::api_delete_user::*;
}
mod api_export_audit_log {
    pub use crate::api_export_audit_log::*;
}
mod api_list_permissions {
    pub use crate::api_list_permissions::*;
}
mod api_list_roles {
    pub use crate::api_list_roles::*;
}
mod api_list_users {
    pub use crate::api_list_users::*;
}
mod api_me {
    pub use crate::api_me::*;
}
mod api_refresh {
    pub use crate::api_refresh::*;
}
mod api_revoke_all_sessions {
    pub use crate::api_revoke_all_sessions::*;
}
mod api_revoke_session {
    pub use crate::api_revoke_session::*;
}
mod api_sessions {
    pub use crate::api_sessions::*;
}
mod api_set_role_permissions {
    pub use crate::api_set_role_permissions::*;
}
mod api_set_user_ban {
    pub use crate::api_set_user_ban::*;
}
mod api_set_user_password {
    pub use crate::api_set_user_password::*;
}
mod api_set_user_roles {
    pub use crate::api_set_user_roles::*;
}
mod api_settings {
    pub use crate::api_settings::*;
}
mod api_sign_in {
    pub use crate::api_sign_in::*;
}
mod api_sign_out {
    pub use crate::api_sign_out::*;
}
mod api_update_role {
    pub use crate::api_update_role::*;
}
mod api_update_settings {
    pub use crate::api_update_settings::*;
}
mod api_update_user {
    pub use crate::api_update_user::*;
}
mod append_cleared_session_cookies {
    pub use crate::append_cleared_session_cookies::*;
}
mod append_session_cookies {
    pub use crate::append_session_cookies::*;
}
mod authn_apply_refresh_failure_delay {
    pub use crate::authn_apply_refresh_failure_delay::*;
}
mod authn_refresh {
    pub use crate::authn_refresh::*;
}
mod authn_sign_in {
    pub use crate::authn_sign_in::*;
}
mod authn_sign_out {
    pub use crate::authn_sign_out::*;
}
pub(crate) mod authorization_authenticate {
    pub use crate::authorization_authenticate::*;
}
pub(crate) mod authorization_authorize_generated_request {
    pub use crate::authorization_authorize_generated_request::*;
}
pub(crate) mod authorization_hash_refresh_token_with_context {
    pub use crate::authorization_hash_refresh_token_with_context::*;
}
pub(crate) mod authorization_origin_is_present_and_allowed {
    pub use crate::authorization_origin_is_present_and_allowed::*;
}
pub(crate) mod authorization_session_context_hash {
    pub use crate::authorization_session_context_hash::*;
}
pub(crate) mod authorization_validate_csrf {
    pub use crate::authorization_validate_csrf::*;
}
mod data_tables_get {
    pub use crate::data_tables_get::*;
}
mod data_tables_list {
    pub use crate::data_tables_list::*;
}
mod extractors {
    pub use crate::extractors::*;
}
mod html {
    pub use crate::html::*;
}
mod jsonwebtoken_admin_encoding_key {
    pub use crate::jsonwebtoken_admin_encoding_key::*;
}
mod persistence {
    pub use crate::persistence::*;
}
mod roles {
    pub use crate::application_roles::*;
}
mod sessions {
    pub use crate::sessions::*;
}
mod sessions_revoke_all_sessions {
    pub use crate::sessions_revoke_all_sessions::*;
}
mod sessions_revoke_session {
    pub use crate::sessions_revoke_session::*;
}
mod settings_branding {
    pub use crate::settings_branding::*;
}
mod settings_branding_view {
    pub use crate::settings_branding_view::*;
}
mod settings_branding_view_ref {
    pub use crate::settings_branding_view_ref::*;
}
mod settings_get {
    pub use crate::settings_get::*;
}
mod settings_update {
    pub use crate::settings_update::*;
}
mod shared {
    pub use crate::shared::*;
}
mod state {
    pub use crate::state::*;
}
mod users {
    pub use crate::application_users::*;
}
mod std_admin_access_ttl_seconds {
    pub use crate::std_admin_access_ttl_seconds::*;
}
mod std_admin_refresh_ttl_seconds {
    pub use crate::std_admin_refresh_ttl_seconds::*;
}
mod std_admin_session_limit {
    pub use crate::std_admin_session_limit::*;
}
mod std_admin_failure_threshold {
    pub use crate::std_admin_failure_threshold::*;
}
mod admin_auth_positive_value_error {
    pub use crate::admin_auth_positive_value_error::*;
}
mod std_admin_failure_delay_millis {
    pub use crate::std_admin_failure_delay_millis::*;
}
mod admin_auth_policy {
    pub use crate::admin_auth_policy::*;
}
mod admin_auth_svc_state {
    pub use crate::admin_auth_svc_state::*;
}
mod shared_admin_auth_svc_state_arc {
    pub use crate::shared_admin_auth_svc_state_arc::*;
}
mod admin_auth_svc_state_build_error {
    pub use crate::admin_auth_svc_state_build_error::*;
}
mod authenticated_admin {
    pub use crate::authenticated_admin::*;
}
mod admin_audit_query {
    pub use crate::admin_audit_query::*;
}
mod http_admin_header_map {
    pub use crate::http_admin_header_map::*;
}
mod http_admin_header_value_error {
    pub use crate::http_admin_header_value_error::*;
}
mod axum_admin_response {
    pub use crate::axum_admin_response::*;
}
mod axum_admin_auth_router {
    pub use crate::axum_admin_auth_router::*;
}
mod utoipa_admin_auth_open_api {
    pub use crate::utoipa_admin_auth_open_api::*;
}
mod admin_auth_html_routes {
    pub use crate::admin_auth_html_routes::*;
}
mod admin_html_swagger_enabled {
    pub use crate::admin_html_swagger_enabled::*;
}
mod html_routes_with_swagger {
    pub use crate::html_routes_with_swagger::*;
}
mod admin_session_bundle {
    pub use crate::admin_session_bundle::*;
}
mod admin_session_error {
    pub use crate::admin_session_error::*;
}
mod std_admin_rate_limit_count {
    pub use crate::std_admin_rate_limit_count::*;
}
mod std_admin_rate_limit_window_seconds {
    pub use crate::std_admin_rate_limit_window_seconds::*;
}
mod admin_audit_query_parts {
    pub use crate::admin_audit_query_parts::*;
}
mod admin_auth_req {
    pub use crate::admin_auth_req::*;
}
mod admin_peer_addr {
    pub use crate::admin_peer_addr::*;
}
mod admin_sign_in_json {
    pub use crate::admin_sign_in_json::*;
}
mod axum_admin_json {
    pub use crate::axum_admin_json::*;
}
mod axum_admin_form {
    pub use crate::axum_admin_form::*;
}
mod axum_admin_path {
    pub use crate::axum_admin_path::*;
}
mod axum_admin_query {
    pub use crate::axum_admin_query::*;
}
mod admin_session_path {
    pub use crate::admin_session_path::*;
}
mod admin_error {
    pub use crate::admin_error::*;
}
mod axum_admin_state_router {
    pub use crate::axum_admin_state_router::*;
}
mod jsonwebtoken_admin_decoding_keys {
    pub use crate::jsonwebtoken_admin_decoding_keys::*;
}
mod admin_password_from_contract {
    pub use crate::admin_password_from_contract::*;
}
mod admin_new_password_from_contract {
    pub use crate::admin_new_password_from_contract::*;
}
mod authenticated_admin_contract {
    pub use crate::authenticated_admin_contract::*;
}
mod admin_error_response_parts {
    pub use crate::admin_error_response_parts::*;
}
mod audit_export_log {
    pub use crate::audit_export_log::*;
}
mod audit_query_log {
    pub use crate::audit_query_log::*;
}
mod create_session_in_connection {
    pub use crate::create_session_in_connection::*;
}
mod rate_limit {
    pub use crate::rate_limit::*;
}
mod routes {
    pub use crate::routes::*;
}
