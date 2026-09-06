#![allow(
    unused_imports,
    clippy::arbitrary_source_item_ordering,
    clippy::wildcard_imports,
    reason = "root-owned modules retain the vocabulary and DTO/route-metadata grouping previously inherited from the contract owner module"
)]

pub mod admin_api_body_max_bytes;
pub mod admin_api_route_path;
pub mod admin_audit_cursor;
pub mod admin_audit_details_bytes;
pub mod admin_audit_details_max_bytes;
pub mod admin_audit_details_too_large;
pub mod admin_audit_export;
pub mod admin_audit_export_csv;
pub mod admin_audit_export_route;
pub mod admin_audit_log_id;
pub mod admin_audit_log_route;
pub mod admin_audit_page;
pub mod admin_audit_timestamp;
pub mod admin_audit_view;
pub mod admin_audit_views;
pub mod admin_bool;
pub mod admin_bounded_vec;
pub mod admin_branding_route;
pub mod admin_branding_view;
pub mod admin_change_own_password_request;
pub mod admin_change_own_password_route;
pub mod admin_collection_error;
pub mod admin_collection_max_items;
pub mod admin_create_role_request;
pub mod admin_create_role_response;
pub mod admin_create_role_route;
pub mod admin_create_user_request;
pub mod admin_create_user_response;
pub mod admin_create_user_route;
pub mod admin_data_column;
pub mod admin_data_columns;
pub mod admin_data_columns_csv_ref;
pub mod admin_data_filter;
pub mod admin_data_filters;
pub mod admin_data_order_ref;
pub mod admin_data_row;
pub mod admin_data_rows;
pub mod admin_data_table;
pub mod admin_data_table_catalog;
pub mod admin_data_table_filter_query;
pub mod admin_data_table_frontend_path;
pub mod admin_data_table_query;
pub mod admin_data_table_route;
pub mod admin_data_table_spec;
pub mod admin_data_table_str_ref;
pub mod admin_data_table_view;
pub mod admin_data_tables;
pub mod admin_data_tables_route;
pub mod admin_default_page_limit;
pub mod admin_default_route;
pub mod admin_delete_role_route;
pub mod admin_delete_user_route;
pub mod admin_display_name;
pub mod admin_empty_collection;
pub mod admin_filter_field;
pub mod admin_filter_operation_key;
pub mod admin_filter_value;
pub mod admin_frontend_path;
pub mod admin_html_action;
pub mod admin_id_try_from_i64_error;
pub mod admin_list_permissions_route;
pub mod admin_list_roles_route;
pub mod admin_list_users_route;
pub mod admin_login;
pub mod admin_main_logo;
pub mod admin_me_route;
pub mod admin_new_password;
pub mod admin_no_body;
pub mod admin_open_api_vec;
pub mod admin_open_api_vec_phantom_data;
pub mod admin_optional_setting;
pub mod admin_optional_settings;
pub mod admin_organization_contacts;
pub mod admin_organization_name;
pub mod admin_page;
pub mod admin_page_capability;
pub mod admin_page_client_mode;
pub mod admin_page_limit;
pub mod admin_page_limit_error;
pub mod admin_page_limit_visitor;
pub mod admin_page_metadata;
pub mod admin_page_navigation;
pub mod admin_page_offset;
pub mod admin_page_offset_visitor;
pub mod admin_page_path_ref;
pub mod admin_page_spec;
pub mod admin_page_title;
pub mod admin_page_total;
pub mod admin_parameterized_route_path;
pub mod admin_password;
pub mod admin_password_entropy;
pub mod admin_path_route_name;
pub mod admin_permission;
pub mod admin_permission_id;
pub mod admin_permission_ids;
pub mod admin_permission_requirement;
pub mod admin_permission_str_ref;
pub mod admin_permission_summaries;
pub mod admin_permission_summary;
pub mod admin_permission_value;
pub mod admin_permission_values;
pub mod admin_permissions_page;
pub mod admin_primary_color;
pub mod admin_refresh_route;
pub mod admin_revoke_all_sessions_route;
pub mod admin_revoke_session_route;
pub mod admin_role_id;
pub mod admin_role_ids;
pub mod admin_role_name;
pub mod admin_role_names;
pub mod admin_role_summaries;
pub mod admin_role_summary;
pub mod admin_roles_page;
pub mod admin_route;
pub mod admin_route_path;
pub mod admin_route_path_error;
pub mod admin_session_identifier;
pub mod admin_session_timestamp;
pub mod admin_session_view;
pub mod admin_session_views;
pub mod admin_sessions_page;
pub mod admin_sessions_route;
pub mod admin_set_role_permissions_request;
pub mod admin_set_role_permissions_route;
pub mod admin_set_user_ban_request;
pub mod admin_set_user_ban_route;
pub mod admin_set_user_password_request;
pub mod admin_set_user_password_route;
pub mod admin_set_user_roles_request;
pub mod admin_set_user_roles_route;
pub mod admin_setting;
pub mod admin_setting_input_kind;
pub mod admin_setting_label;
pub mod admin_setting_name;
pub mod admin_setting_optionality;
pub mod admin_setting_spec;
pub mod admin_settings_route;
pub mod admin_settings_view;
pub mod admin_sign_in_request;
pub mod admin_sign_in_response;
pub mod admin_sign_in_route;
pub mod admin_sign_out_route;
pub mod admin_site_name;
pub mod admin_sort_direction;
pub mod admin_support_url;
pub mod admin_tab_title;
pub mod admin_table_query;
pub mod admin_table_search;
pub mod admin_table_sort_field;
pub mod admin_table_sort_field_try_from_key_error;
pub mod admin_table_sort_key;
pub mod admin_table_sort_key_ref;
pub mod admin_table_sort_values;
pub mod admin_text;
pub mod admin_texts;
pub mod admin_update_role_request;
pub mod admin_update_role_route;
pub mod admin_update_settings_request;
pub mod admin_update_settings_route;
pub mod admin_update_user_request;
pub mod admin_update_user_route;
pub mod admin_user_id;
pub mod admin_user_summaries;
pub mod admin_user_summary;
pub mod admin_users_page;
#[cfg(test)]
mod test_audit_branding_tests {
    #[test]
    fn test_audit_detail_limit_is_stable() {
        assert_eq!(
            crate::admin_audit_details_too_large::AdminAuditDetailsTooLarge::from(
                crate::admin_audit_details_bytes::AdminAuditDetailsBytes::from(
                    constants_usize::ONE,
                )
            )
            .maximum_bytes(),
            crate::admin_audit_details_bytes::AdminAuditDetailsBytes::from(
                crate::admin_audit_details_max_bytes::ADMIN_AUDIT_DETAILS_MAX_BYTES
            ),
        );
    }
}
pub mod authenticated_admin;
#[cfg(test)]
mod test_authorization_catalog_tests {
    #[test]
    fn test_user_table_requires_user_read_permission() {
        assert_eq!(
            crate::admin_data_table::AdminDataTable::Users.permission(),
            crate::admin_permission::AdminPermission::UsersRead,
        );
    }
}
pub mod default_admin_api_body_max_bytes;
pub mod identity;
pub mod positive_non_zero_i64;
pub mod serde_json_admin_audit_details;
#[cfg(test)]
pub mod test_domain_types_dto_tests;
#[cfg(test)]
pub mod test_domain_types_query_tests;
#[cfg(test)]
pub mod test_domain_types_routes_tests;
#[cfg(test)]
pub mod test_domain_types_sessions_tests;
#[cfg(test)]
pub mod test_domain_types_settings_tests;
#[cfg(test)]
pub mod test_tests_domain_types;
#[cfg(test)]
mod test_table_sort_tests {
    #[test]
    fn test_user_login_sort_field_has_login_key() {
        assert_eq!(
            crate::admin_table_sort_field::AdminTableSortField::UserLogin
                .key()
                .as_ref(),
            constants_str::LOGIN,
        );
    }
}
