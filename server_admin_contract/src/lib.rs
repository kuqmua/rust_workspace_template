#![allow(
    unused_imports,
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "root-owned modules retain the vocabulary and DTO/route-metadata grouping previously inherited from the contract owner module"
)]

mod admin_api_body_max_bytes;
pub(crate) use admin_api_body_max_bytes::*;
mod admin_api_route_path;
pub(crate) use admin_api_route_path::*;
mod admin_audit_cursor;
pub use admin_audit_cursor::*;
mod admin_audit_details_bytes;
pub(crate) use admin_audit_details_bytes::*;
mod admin_audit_details_max_bytes;
pub(crate) use admin_audit_details_max_bytes::*;
mod admin_audit_details_too_large;
pub(crate) use admin_audit_details_too_large::*;
mod admin_audit_export;
pub use admin_audit_export::*;
mod admin_audit_export_csv;
pub use admin_audit_export_csv::*;
mod admin_audit_export_route;
mod admin_audit_log_id;
pub(crate) use admin_audit_log_id::*;
mod admin_audit_log_route;
mod admin_audit_page;
pub use admin_audit_page::*;
mod admin_audit_timestamp;
pub(crate) use admin_audit_timestamp::*;
mod admin_audit_view;
pub use admin_audit_view::*;
mod admin_audit_views;
mod admin_bool;
mod admin_bounded_vec;
pub(crate) use admin_bounded_vec::*;
mod admin_branding_route;
mod admin_branding_view;
pub(crate) use admin_branding_view::*;
mod admin_change_own_password_req;
pub use admin_change_own_password_req::*;
mod admin_change_own_password_route;
mod admin_collection_error;
mod admin_collection_max_items;
pub(crate) use admin_collection_max_items::*;
mod admin_create_role_req;
pub use admin_create_role_req::*;
mod admin_create_role_res;
pub use admin_create_role_res::*;
mod admin_create_role_route;
mod admin_create_user_req;
pub use admin_create_user_req::*;
mod admin_create_user_res;
pub use admin_create_user_res::*;
mod admin_create_user_route;
mod admin_data_column;
pub use admin_data_column::*;
mod admin_data_columns;
pub use admin_data_columns::*;
mod admin_data_columns_csv_ref;
pub(crate) use admin_data_columns_csv_ref::*;
mod admin_data_filter;
pub use admin_data_filter::*;
mod admin_data_filters;
pub use admin_data_filters::*;
mod admin_data_order_ref;
pub(crate) use admin_data_order_ref::*;
mod admin_data_row;
pub use admin_data_row::*;
mod admin_data_rows;
mod admin_data_table;
pub(crate) use admin_data_table::*;
mod admin_data_table_catalog;
pub use admin_data_table_catalog::*;
mod admin_data_table_filter_query;
mod admin_data_table_frontend_path;
mod admin_data_table_query;
mod admin_data_table_route;
mod admin_data_table_spec;
pub(crate) use admin_data_table_spec::*;
mod admin_data_table_str_ref;
pub(crate) use admin_data_table_str_ref::*;
mod admin_data_table_view;
pub use admin_data_table_view::*;
mod admin_data_tables;
mod admin_data_tables_route;
mod admin_default_page_limit;
pub(crate) use admin_default_page_limit::*;
mod admin_default_route;
pub(crate) use admin_default_route::*;
mod admin_delete_role_route;
mod admin_delete_user_route;
mod admin_display_name;
pub(crate) use admin_display_name::*;
mod admin_empty_collection;
pub(crate) use admin_empty_collection::*;
mod admin_filter_field;
mod admin_filter_operation_key;
mod admin_filter_value;
mod admin_frontend_path;
mod admin_html_action;
mod admin_id_try_from_i64_error;
pub(crate) use admin_id_try_from_i64_error::*;
mod admin_list_permissions_route;
mod admin_list_roles_route;
mod admin_list_users_route;
mod admin_login;
pub(crate) use admin_login::*;
mod admin_main_logo;
pub(crate) use admin_main_logo::*;
mod admin_me_route;
mod admin_new_password;
pub(crate) use admin_new_password::*;
mod admin_no_body;
pub(crate) use admin_no_body::*;
mod admin_open_api_vec;
pub(crate) use admin_open_api_vec::*;
mod admin_open_api_vec_phantom_data;
pub(crate) use admin_open_api_vec_phantom_data::*;
mod admin_optional_setting;
pub use admin_optional_setting::*;
mod admin_optional_settings;
mod admin_organization_contacts;
pub(crate) use admin_organization_contacts::*;
mod admin_organization_name;
pub(crate) use admin_organization_name::*;
mod admin_page;
mod admin_page_capability;
mod admin_page_client_mode;
mod admin_page_limit;
mod admin_page_limit_error;
mod admin_page_limit_visitor;
pub(crate) use admin_page_limit_visitor::*;
mod admin_page_metadata;
mod admin_page_navigation;
mod admin_page_offset;
mod admin_page_offset_visitor;
pub(crate) use admin_page_offset_visitor::*;
mod admin_page_path_ref;
mod admin_page_spec;
mod admin_page_title;
pub(crate) use admin_page_title::*;
mod admin_page_total;
mod admin_parameterized_route_path;
mod admin_password;
pub(crate) use admin_password::*;
mod admin_path_route_name;
pub(crate) use admin_path_route_name::*;
mod admin_permission;
pub(crate) use admin_permission::*;
mod admin_permission_id;
pub(crate) use admin_permission_id::*;
mod admin_permission_ids;
mod admin_permission_requirement;
pub(crate) use admin_permission_requirement::*;
mod admin_permission_str_ref;
pub(crate) use admin_permission_str_ref::*;
mod admin_permission_summaries;
mod admin_permission_summary;
pub use admin_permission_summary::*;
mod admin_permission_value;
pub(crate) use admin_permission_value::*;
mod admin_permission_values;
mod admin_permissions_page;
pub use admin_permissions_page::*;
mod admin_primary_color;
pub(crate) use admin_primary_color::*;
mod admin_refresh_route;
mod admin_revoke_all_sessions_route;
mod admin_revoke_session_route;
mod admin_role_id;
pub(crate) use admin_role_id::*;
mod admin_role_ids;
mod admin_role_name;
pub(crate) use admin_role_name::*;
mod admin_role_names;
mod admin_role_summaries;
mod admin_role_summary;
pub use admin_role_summary::*;
mod admin_roles_page;
pub use admin_roles_page::*;
mod admin_route;
mod admin_route_path;
mod admin_route_path_error;
mod admin_session_identifier;
pub(crate) use admin_session_identifier::*;
mod admin_session_timestamp;
pub(crate) use admin_session_timestamp::*;
mod admin_session_view;
pub(crate) use admin_session_view::*;
mod admin_session_views;
mod admin_sessions_page;
pub(crate) use admin_sessions_page::*;
mod admin_sessions_route;
mod admin_set_role_permissions_req;
pub use admin_set_role_permissions_req::*;
mod admin_set_role_permissions_route;
mod admin_set_user_ban_req;
pub use admin_set_user_ban_req::*;
mod admin_set_user_ban_route;
mod admin_set_user_password_req;
pub use admin_set_user_password_req::*;
mod admin_set_user_password_route;
mod admin_set_user_roles_req;
pub use admin_set_user_roles_req::*;
mod admin_set_user_roles_route;
mod admin_setting;
pub use admin_setting::*;
mod admin_setting_input_kind;
pub(crate) use admin_setting_input_kind::*;
mod admin_setting_label;
pub(crate) use admin_setting_label::*;
mod admin_setting_name;
pub(crate) use admin_setting_name::*;
mod admin_setting_optionality;
pub(crate) use admin_setting_optionality::*;
mod admin_setting_spec;
pub(crate) use admin_setting_spec::*;
mod admin_settings_route;
mod admin_settings_view;
pub use admin_settings_view::*;
mod admin_sign_in_req;
pub use admin_sign_in_req::*;
mod admin_sign_in_res;
pub use admin_sign_in_res::*;
mod admin_sign_in_route;
mod admin_sign_out_route;
mod admin_site_name;
pub(crate) use admin_site_name::*;
mod admin_sort_direction;
mod admin_support_url;
pub(crate) use admin_support_url::*;
mod admin_tab_title;
pub(crate) use admin_tab_title::*;
mod admin_table_query;
mod admin_table_search;
mod admin_table_sort_field;
pub(crate) use admin_table_sort_field::*;
mod admin_table_sort_field_try_from_key_error;
pub(crate) use admin_table_sort_field_try_from_key_error::*;
mod admin_table_sort_key;
mod admin_table_sort_key_ref;
pub(crate) use admin_table_sort_key_ref::*;
mod admin_table_sort_values;
pub(crate) use admin_table_sort_values::*;
mod admin_text;
pub(crate) use admin_text::*;
mod admin_texts;
mod admin_update_role_req;
pub use admin_update_role_req::*;
mod admin_update_role_route;
mod admin_update_settings_req;
pub(crate) use admin_update_settings_req::*;
mod admin_update_settings_route;
mod admin_update_user_req;
pub use admin_update_user_req::*;
mod admin_update_user_route;
mod admin_user_id;
pub(crate) use admin_user_id::*;
mod admin_user_summaries;
mod admin_user_summary;
pub use admin_user_summary::*;
mod admin_users_page;
pub use admin_audit_details_bytes::AdminAuditDetailsBytes;
pub use admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES;
pub use admin_audit_details_too_large::AdminAuditDetailsTooLarge;
pub use admin_audit_timestamp::AdminAuditTimestamp;
pub use admin_default_route::AdminDefaultRoute;
pub use admin_main_logo::AdminMainLogo;
pub use admin_organization_contacts::AdminOrganizationContacts;
pub use admin_organization_name::AdminOrganizationName;
pub use admin_primary_color::AdminPrimaryColor;
pub use admin_site_name::AdminSiteName;
pub use admin_support_url::AdminSupportUrl;
pub use admin_tab_title::AdminTabTitle;
pub use admin_users_page::*;
pub use serde_json_admin_audit_details::SerdeJsonAdminAuditDetails;
#[cfg(test)]
mod audit_branding_tests {
    #[test]
    fn audit_detail_limit_is_stable() {
        assert_eq!(
            super::AdminAuditDetailsTooLarge::from(super::AdminAuditDetailsBytes::from(
                constants_usize::ONE,
            ))
            .maximum_bytes(),
            super::AdminAuditDetailsBytes::from(super::ADMIN_AUDIT_DETAILS_MAX_BYTES),
        );
    }
}
mod authenticated_admin;
pub use admin_data_columns_csv_ref::AdminDataColumnsCsvRef;
pub use admin_data_order_ref::AdminDataOrderRef;
pub use admin_data_table::{AdminDataTable, AdminDataTableTryFromStrError};
pub use admin_data_table_spec::AdminDataTableSpec;
pub use admin_data_table_str_ref::AdminDataTableStrRef;
pub use admin_permission::{AdminPermission, AdminPermissionTryFromStrError};
pub use admin_permission_str_ref::AdminPermissionStrRef;
pub use admin_permission_value::AdminPermissionValue;
pub use authenticated_admin::*;
#[cfg(test)]
mod authorization_catalog_tests {
    #[test]
    fn user_table_requires_user_read_permission() {
        assert_eq!(
            super::AdminDataTable::Users.permission(),
            super::AdminPermission::UsersRead,
        );
    }
}
use self::admin_empty_collection::AdminEmptyCollection;
use self::admin_open_api_vec_phantom_data::AdminOpenApiVecPhantomData;
pub use admin_audit_views::*;
pub(crate) use admin_bounded_vec::*;
pub use admin_collection_error::*;
pub(crate) use admin_collection_max_items::*;
pub use admin_data_rows::*;
pub use admin_data_tables::*;
pub(crate) use admin_open_api_vec::*;
pub use admin_optional_settings::*;
pub use admin_permission_ids::*;
pub use admin_permission_summaries::*;
pub use admin_permission_values::*;
pub use admin_role_ids::*;
pub use admin_role_names::*;
pub use admin_role_summaries::*;
pub use admin_session_views::*;
pub use admin_texts::*;
pub use admin_user_summaries::*;
mod default_admin_api_body_max_bytes;
pub(crate) use default_admin_api_body_max_bytes::*;
#[cfg(test)]
mod domain_types_dto_tests;
#[cfg(test)]
pub(crate) use domain_types_dto_tests::*;
#[cfg(test)]
mod domain_types_query_tests;
#[cfg(test)]
pub(crate) use domain_types_query_tests::*;
#[cfg(test)]
mod domain_types_routes_tests;
#[cfg(test)]
pub(crate) use domain_types_routes_tests::*;
#[cfg(test)]
mod domain_types_sessions_tests;
#[cfg(test)]
pub(crate) use domain_types_sessions_tests::*;
#[cfg(test)]
mod domain_types_settings_tests;
#[cfg(test)]
pub(crate) use domain_types_settings_tests::*;
#[cfg(test)]
mod domain_types_tests;
pub use admin_audit_log_id::AdminAuditLogId;
pub use admin_id_try_from_i64_error::AdminIdTryFromI64Error;
pub use admin_permission_id::AdminPermissionId;
pub use admin_role_id::AdminRoleId;
pub use admin_user_id::AdminUserId;
#[cfg(test)]
pub(crate) use domain_types_tests::*;
pub use frontend_contract::InputKind;
pub mod identity;
pub(crate) use identity::*;
mod positive_non_zero_i64;
use self::admin_default_page_limit::AdminDefaultPageLimit;
use self::admin_page_limit_visitor::AdminPageLimitVisitor;
use self::admin_page_offset_visitor::AdminPageOffsetVisitor;
pub(crate) use admin_api_route_path::*;
pub use admin_audit_export_route::*;
pub use admin_audit_log_route::*;
pub use admin_bool::*;
pub use admin_branding_route::*;
pub use admin_change_own_password_route::*;
pub use admin_create_role_route::*;
pub use admin_create_user_route::*;
pub use admin_data_table_filter_query::*;
pub use admin_data_table_frontend_path::*;
pub use admin_data_table_query::*;
pub use admin_data_table_route::*;
pub use admin_data_tables_route::*;
pub use admin_delete_role_route::*;
pub use admin_delete_user_route::*;
pub use admin_filter_field::*;
pub use admin_filter_operation_key::*;
pub use admin_filter_value::*;
pub use admin_frontend_path::*;
pub use admin_html_action::*;
pub use admin_list_permissions_route::*;
pub use admin_list_roles_route::*;
pub use admin_list_users_route::*;
pub use admin_me_route::*;
pub use admin_page::*;
pub use admin_page_capability::*;
pub use admin_page_client_mode::*;
pub use admin_page_limit::*;
pub use admin_page_limit_error::*;
pub use admin_page_metadata::*;
pub use admin_page_navigation::*;
pub use admin_page_offset::*;
pub use admin_page_path_ref::*;
pub use admin_page_spec::*;
pub(crate) use admin_page_title::*;
pub use admin_page_total::*;
pub use admin_parameterized_route_path::*;
pub(crate) use admin_path_route_name::*;
pub(crate) use admin_permission_requirement::*;
pub use admin_refresh_route::*;
pub use admin_revoke_all_sessions_route::*;
pub use admin_revoke_session_route::*;
pub use admin_route::*;
pub use admin_route_path::*;
pub use admin_route_path_error::*;
pub use admin_sessions_route::*;
pub use admin_set_role_permissions_route::*;
pub use admin_set_user_ban_route::*;
pub use admin_set_user_password_route::*;
pub use admin_set_user_roles_route::*;
pub use admin_settings_route::*;
pub use admin_sign_in_route::*;
pub use admin_sign_out_route::*;
pub use admin_sort_direction::*;
pub use admin_table_query::*;
pub use admin_table_search::*;
pub use admin_table_sort_key::*;
pub use admin_update_role_route::*;
pub use admin_update_settings_route::*;
pub use admin_update_user_route::*;
pub(crate) use positive_non_zero_i64::*;
mod serde_json_admin_audit_details;
use self::admin_table_sort_values::AdminTableSortValues;
pub use admin_branding_view::AdminBrandingView;
pub use admin_no_body::AdminNoBody;
pub use admin_session_identifier::AdminSessionIdentifier;
pub use admin_session_timestamp::AdminSessionTimestamp;
pub use admin_session_view::AdminSessionView;
pub use admin_sessions_page::AdminSessionsPage;
pub use admin_setting_input_kind::AdminSettingInputKind;
pub use admin_setting_label::AdminSettingLabel;
pub use admin_setting_name::AdminSettingName;
pub use admin_setting_optionality::AdminSettingOptionality;
pub use admin_setting_spec::AdminSettingSpec;
pub use admin_table_sort_field::AdminTableSortField;
pub use admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;
pub use admin_table_sort_key_ref::AdminTableSortKeyRef;
pub use admin_update_settings_req::AdminUpdateSettingsReq;
pub(crate) use serde_json_admin_audit_details::*;
#[cfg(test)]
mod table_sort_tests {
    #[test]
    fn user_login_sort_field_has_login_key() {
        assert_eq!(
            super::AdminTableSortField::UserLogin.key().as_ref(),
            constants_str::LOGIN,
        );
    }
}

pub mod domain_types;
