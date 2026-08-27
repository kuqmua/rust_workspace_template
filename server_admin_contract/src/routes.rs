#[path = "routes/admin_sign_in_route.rs"]
mod admin_sign_in_route;
pub use admin_sign_in_route::*;
#[path = "routes/admin_refresh_route.rs"]
mod admin_refresh_route;
pub use admin_refresh_route::*;
#[path = "routes/admin_me_route.rs"]
mod admin_me_route;
pub use admin_me_route::*;
#[path = "routes/admin_change_own_password_route.rs"]
mod admin_change_own_password_route;
pub use admin_change_own_password_route::*;
#[path = "routes/admin_sign_out_route.rs"]
mod admin_sign_out_route;
pub use admin_sign_out_route::*;
#[path = "routes/admin_sessions_route.rs"]
mod admin_sessions_route;
pub use admin_sessions_route::*;
#[path = "routes/admin_revoke_session_route.rs"]
mod admin_revoke_session_route;
pub use admin_revoke_session_route::*;
#[path = "routes/admin_revoke_all_sessions_route.rs"]
mod admin_revoke_all_sessions_route;
pub use admin_revoke_all_sessions_route::*;
#[path = "routes/admin_list_users_route.rs"]
mod admin_list_users_route;
pub use admin_list_users_route::*;
#[path = "routes/admin_create_user_route.rs"]
mod admin_create_user_route;
pub use admin_create_user_route::*;
#[path = "routes/admin_update_user_route.rs"]
mod admin_update_user_route;
pub use admin_update_user_route::*;
#[path = "routes/admin_delete_user_route.rs"]
mod admin_delete_user_route;
pub use admin_delete_user_route::*;
#[path = "routes/admin_set_user_password_route.rs"]
mod admin_set_user_password_route;
pub use admin_set_user_password_route::*;
#[path = "routes/admin_set_user_ban_route.rs"]
mod admin_set_user_ban_route;
pub use admin_set_user_ban_route::*;
#[path = "routes/admin_set_user_roles_route.rs"]
mod admin_set_user_roles_route;
pub use admin_set_user_roles_route::*;
#[path = "routes/admin_list_roles_route.rs"]
mod admin_list_roles_route;
pub use admin_list_roles_route::*;
#[path = "routes/admin_create_role_route.rs"]
mod admin_create_role_route;
pub use admin_create_role_route::*;
#[path = "routes/admin_update_role_route.rs"]
mod admin_update_role_route;
pub use admin_update_role_route::*;
#[path = "routes/admin_delete_role_route.rs"]
mod admin_delete_role_route;
pub use admin_delete_role_route::*;
#[path = "routes/admin_set_role_permissions_route.rs"]
mod admin_set_role_permissions_route;
pub use admin_set_role_permissions_route::*;
#[path = "routes/admin_list_permissions_route.rs"]
mod admin_list_permissions_route;
pub use admin_list_permissions_route::*;
#[path = "routes/admin_audit_log_route.rs"]
mod admin_audit_log_route;
pub use admin_audit_log_route::*;
#[path = "routes/admin_audit_export_route.rs"]
mod admin_audit_export_route;
pub use admin_audit_export_route::*;
#[path = "routes/admin_branding_route.rs"]
mod admin_branding_route;
pub use admin_branding_route::*;
#[path = "routes/admin_data_tables_route.rs"]
mod admin_data_tables_route;
pub use admin_data_tables_route::*;
#[path = "routes/admin_data_table_route.rs"]
mod admin_data_table_route;
pub use admin_data_table_route::*;
#[path = "routes/admin_settings_route.rs"]
mod admin_settings_route;
pub use admin_settings_route::*;
#[path = "routes/admin_update_settings_route.rs"]
mod admin_update_settings_route;
pub use admin_update_settings_route::*;
#[path = "routes/admin_route.rs"]
mod admin_route;
pub use admin_route::*;
#[path = "routes/admin_data_table_frontend_path.rs"]
mod admin_data_table_frontend_path;
pub use admin_data_table_frontend_path::*;
#[path = "routes/admin_route_path.rs"]
mod admin_route_path;
pub use admin_route_path::*;
#[path = "routes/admin_route_path_error.rs"]
mod admin_route_path_error;
pub use admin_route_path_error::*;
#[path = "routes/admin_page_path_ref.rs"]
mod admin_page_path_ref;
pub use admin_page_path_ref::*;
#[path = "routes/admin_frontend_path.rs"]
mod admin_frontend_path;
pub use admin_frontend_path::*;
#[path = "routes/admin_html_action.rs"]
mod admin_html_action;
pub use admin_html_action::*;
#[path = "routes/admin_page.rs"]
mod admin_page;
pub use admin_page::*;
#[path = "routes/admin_page_capability.rs"]
mod admin_page_capability;
pub use admin_page_capability::*;
#[path = "routes/admin_page_client_mode.rs"]
mod admin_page_client_mode;
pub use admin_page_client_mode::*;
#[path = "routes/admin_page_navigation.rs"]
mod admin_page_navigation;
pub use admin_page_navigation::*;
#[path = "routes/admin_page_metadata.rs"]
mod admin_page_metadata;
pub use admin_page_metadata::*;
#[path = "routes/admin_page_spec.rs"]
mod admin_page_spec;
pub use admin_page_spec::*;
#[path = "routes/admin_parameterized_route_path.rs"]
mod admin_parameterized_route_path;
pub use admin_parameterized_route_path::*;
#[path = "routes/admin_permission_requirement.rs"]
mod admin_permission_requirement;
pub(super) use admin_permission_requirement::*;
#[path = "routes/admin_page_title.rs"]
mod admin_page_title;
pub(super) use admin_page_title::*;
#[path = "routes/admin_path_route_name.rs"]
mod admin_path_route_name;
pub(super) use admin_path_route_name::*;
#[path = "routes/admin_api_route_path.rs"]
mod admin_api_route_path;
pub(super) use admin_api_route_path::*;

#[cfg(test)]
#[path = "domain_types_routes_tests.rs"]
mod tests;
