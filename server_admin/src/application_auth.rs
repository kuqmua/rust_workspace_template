use admin_error_response_parts::*;
use admin_new_password_from_contract::*;
use admin_password_from_contract::*;
use authenticated_admin_contract::*;
use jsonwebtoken_admin_decoding_keys::*;
frontend_contract_macros::api_operation_error!(AdminAuditLogError,);
frontend_contract_macros::api_operation_error!(AdminAuditExportError,);
frontend_contract_macros::api_operation_error!(AdminBrandingError,);
frontend_contract_macros::api_operation_error!(AdminChangeOwnPasswordError,);
frontend_contract_macros::api_operation_error!(AdminCreateRoleError,);
frontend_contract_macros::api_operation_error!(AdminCreateUserError,);
frontend_contract_macros::api_operation_error!(AdminDataTableError,);
frontend_contract_macros::api_operation_error!(AdminDataTablesError,);
frontend_contract_macros::api_operation_error!(AdminDeleteRoleError,);
frontend_contract_macros::api_operation_error!(AdminDeleteUserError,);
frontend_contract_macros::api_operation_error!(AdminListPermissionsError,);
frontend_contract_macros::api_operation_error!(AdminListRolesError,);
frontend_contract_macros::api_operation_error!(AdminListUsersError,);
frontend_contract_macros::api_operation_error!(AdminMeError,);
frontend_contract_macros::api_operation_error!(AdminRefreshError,);
frontend_contract_macros::api_operation_error!(AdminRevokeAllSessionsError,);
frontend_contract_macros::api_operation_error!(AdminRevokeSessionError,);
frontend_contract_macros::api_operation_error!(AdminSessionsError,);
frontend_contract_macros::api_operation_error!(AdminSetRolePermissionsError,);
frontend_contract_macros::api_operation_error!(AdminSetUserBanError,);
frontend_contract_macros::api_operation_error!(AdminSetUserPasswordError,);
frontend_contract_macros::api_operation_error!(AdminSetUserRolesError,);
frontend_contract_macros::api_operation_error!(AdminSettingsError,);
frontend_contract_macros::api_operation_error!(AdminSignInError,);
frontend_contract_macros::api_operation_error!(AdminSignOutError,);
frontend_contract_macros::api_operation_error!(AdminUpdateRoleError,);
frontend_contract_macros::api_operation_error!(AdminUpdateSettingsError,);
frontend_contract_macros::api_operation_error!(AdminUpdateUserError,);

// Root-owned module compatibility wrappers.
mod account_change_own_password {}
mod account_me {}
mod account_me_context_view_ref {}
mod admin_observed_error_code {}
mod api_audit_log {}
mod api_branding {}
mod api_change_own_password {}
mod api_create_role {}
mod api_create_user {}
mod api_data_table {}
mod api_data_tables {}
mod api_delete_role {}
mod api_delete_user {}
mod api_export_audit_log {}
mod api_list_permissions {}
mod api_list_roles {}
mod api_list_users {}
mod api_me {}
mod api_refresh {}
mod api_revoke_all_sessions {}
mod api_revoke_session {}
mod api_sessions {}
mod api_set_role_permissions {}
mod api_set_user_ban {}
mod api_set_user_password {}
mod api_set_user_roles {}
mod api_settings {}
mod api_sign_in {}
mod api_sign_out {}
mod api_update_role {}
mod api_update_settings {}
mod api_update_user {}
mod append_cleared_session_cookies {}
mod append_session_cookies {}
mod authn_apply_refresh_failure_delay {}
mod authn_refresh {}
mod authn_sign_in {}
mod authn_sign_out {}
pub(crate) mod authorization_authenticate {}
pub(crate) mod authorization_authorize_generated_request {}
pub(crate) mod authorization_hash_refresh_token_with_context {}
pub(crate) mod authorization_origin_is_present_and_allowed {}
pub(crate) mod authorization_session_context_hash {}
pub(crate) mod authorization_validate_csrf {}
mod data_tables_get {}
mod data_tables_list {}
mod extractors {}
mod html {}
mod jsonwebtoken_admin_encoding_key {}
mod persistence {}
mod roles {}
mod sessions {}
mod sessions_revoke_all_sessions {}
mod sessions_revoke_session {}
mod settings_branding {}
mod settings_branding_view {}
mod settings_branding_view_ref {}
mod settings_get {}
mod settings_update {}
mod shared {}
mod state {}
mod users {}
mod std_admin_access_ttl_seconds {}
mod std_admin_refresh_ttl_seconds {}
mod std_admin_session_limit {}
mod std_admin_failure_threshold {}
mod admin_auth_positive_value_error {}
mod std_admin_failure_delay_millis {}
mod admin_auth_policy {}
mod admin_auth_svc_state {}
mod shared_admin_auth_svc_state_arc {}
mod admin_auth_svc_state_build_error {}
mod authenticated_admin {}
mod admin_audit_query {}
mod http_admin_header_map {}
mod http_admin_header_value_error {}
mod axum_admin_response {}
mod axum_admin_auth_router {}
mod utoipa_admin_auth_open_api {}
mod admin_auth_html_routes {}
mod admin_html_swagger_enabled {}
mod html_routes_with_swagger {}
mod admin_session_bundle {}
mod admin_session_error {}
mod std_admin_rate_limit_count {}
mod std_admin_rate_limit_window_seconds {}
mod admin_audit_query_parts {}
mod admin_auth_req {}
mod admin_peer_addr {}
mod admin_sign_in_json {}
mod axum_admin_json {}
mod axum_admin_form {}
mod axum_admin_path {}
mod axum_admin_query {}
mod admin_session_path {}
mod admin_error {}
mod axum_admin_state_router {}
mod jsonwebtoken_admin_decoding_keys {}
mod admin_password_from_contract {}
mod admin_new_password_from_contract {}
mod authenticated_admin_contract {}
mod admin_error_response_parts {}
mod audit_export_log {}
mod audit_query_log {}
mod create_session_in_connection {}
mod rate_limit {}
mod routes {}
