#![allow(
    unused_imports,
    unreachable_pub,
    clippy::arbitrary_source_item_ordering,
    clippy::shadow_reuse,
    clippy::tests_outside_test_module,
    clippy::unused_trait_names,
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    reason = "Leptos component names intentionally mirror their owner modules and generated view functions are consumed by view macros; root-owned frontend modules retain the signal, generated-view, test-leaf, and compatibility lint contracts previously scoped across their nested owner modules"
)]

pub mod admin_alert;
#[cfg(any(target_arch = "wasm32", test))]
pub mod admin_alert_dialog;
pub mod admin_alert_variant;
#[cfg(target_arch = "wasm32")]
pub mod admin_api_url;
#[cfg(target_arch = "wasm32")]
pub mod admin_api_url_with_suffix;
#[cfg(target_arch = "wasm32")]
pub mod admin_app;
pub mod admin_assets_error;
pub mod admin_badge;
pub mod admin_badge_variant;
pub mod admin_button;
pub mod admin_button_kind;
pub mod admin_button_link;
pub mod admin_button_variant;
pub mod admin_card;
pub mod admin_card_description;
pub mod admin_card_footer;
pub mod admin_card_header;
pub mod admin_card_title;
pub mod admin_card_variant;
#[cfg(target_arch = "wasm32")]
pub mod admin_change_password;
pub mod admin_checkbox;
#[cfg(target_arch = "wasm32")]
pub mod admin_csr_api_url;
#[cfg(target_arch = "wasm32")]
pub mod admin_csr_api_url_suffix_ref;
#[cfg(target_arch = "wasm32")]
pub mod admin_csr_query;
#[cfg(target_arch = "wasm32")]
pub mod admin_csrf_token;
#[cfg(target_arch = "wasm32")]
pub mod admin_data_grid;
pub mod admin_data_grid_input_type;
pub mod admin_data_table_grid;
#[cfg(any(target_arch = "wasm32", test))]
pub mod admin_empty;
pub mod admin_field;
pub mod admin_field_label;
pub mod admin_filter_hidden_inputs;
pub mod admin_frontend_routes;
#[cfg(target_arch = "wasm32")]
pub mod admin_http_status;
pub mod admin_input;
pub mod admin_input_group;
pub mod admin_input_kind;
pub mod admin_input_name;
pub mod admin_joined_text;
pub mod admin_joined_text_try_from_string_error;
#[cfg(target_arch = "wasm32")]
pub mod admin_load_state;
#[cfg(target_arch = "wasm32")]
pub mod admin_mutation_method;
pub mod admin_navigation_link;
pub mod admin_page_nav_disabled;
pub mod admin_page_range;
#[cfg(target_arch = "wasm32")]
pub mod admin_pagination;
#[cfg(target_arch = "wasm32")]
pub mod admin_permissions_view;
#[cfg(target_arch = "wasm32")]
pub mod admin_profile_account;
#[cfg(target_arch = "wasm32")]
pub mod admin_profile_view;
pub mod admin_role_permissions;
#[cfg(target_arch = "wasm32")]
pub mod admin_roles_view;
#[cfg(target_arch = "wasm32")]
pub mod admin_route_path_url;
#[cfg(target_arch = "wasm32")]
pub mod admin_sessions_view;
pub mod admin_setting_disabled;
pub mod admin_setting_input_value;
pub mod admin_setting_inputs;
pub mod admin_setting_required;
pub mod admin_settings_form_signals;
pub mod admin_settings_form_values;
#[cfg(target_arch = "wasm32")]
pub mod admin_settings_view;
pub mod admin_sidebar;
pub mod admin_sidebar_item;
pub mod admin_spinner;
#[cfg(not(target_arch = "wasm32"))]
pub mod admin_ssr_error_message;
#[cfg(not(target_arch = "wasm32"))]
pub mod admin_ssr_html;
#[cfg(not(target_arch = "wasm32"))]
pub mod admin_ssr_html_try_from_string_error;
#[cfg(not(target_arch = "wasm32"))]
pub mod admin_ssr_text;
#[cfg(not(target_arch = "wasm32"))]
pub mod admin_ssr_text_try_from_string_error;
#[cfg(test)]
pub mod admin_ssr_view_ext_tests;
#[cfg(target_arch = "wasm32")]
pub mod admin_table_load_error;
pub mod admin_table_query_direction;
pub mod admin_table_query_hidden_inputs;
pub mod admin_textarea;
pub mod admin_user_roles;
#[cfg(target_arch = "wasm32")]
pub mod admin_users_view;
pub mod axum_admin_frontend_router;
#[cfg(not(target_arch = "wasm32"))]
pub mod crud_render_shell;
#[cfg(target_arch = "wasm32")]
pub mod csr_admin_nav;
#[cfg(target_arch = "wasm32")]
pub mod csr_admin_role_row;
#[cfg(target_arch = "wasm32")]
pub mod csr_admin_user_row;
#[cfg(target_arch = "wasm32")]
pub mod csr_page_from_location;
#[cfg(target_arch = "wasm32")]
pub mod csrf_token;
#[cfg(not(target_arch = "wasm32"))]
pub mod data_table_grid;
pub mod domain_types_shared_settings_input;
#[cfg(test)]
pub mod domain_types_ssr_tests;
#[cfg(target_arch = "wasm32")]
pub mod fetch_json;
#[cfg(target_arch = "wasm32")]
pub mod fetch_page;
pub mod join_text;
pub mod leptos_admin_filter_operation_signal;
pub mod leptos_admin_input_signal;
#[cfg(target_arch = "wasm32")]
pub mod location;
#[cfg(not(target_arch = "wasm32"))]
pub mod page_render_with_access;
#[cfg(not(target_arch = "wasm32"))]
pub mod page_render_with_table_access;
#[cfg(target_arch = "wasm32")]
pub mod reload_after;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_csr;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_page;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_page_with_access;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_page_with_table_access;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_permissions_page;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_profile_page;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_sessions_page;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_admin_settings_page;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_data_tables;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_data_tables_csr;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_document;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_role_create;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_role_manage;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_roles;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_sign_in;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_text_page;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_text_page_with_access;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_user_create;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_user_manage;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_users;
#[cfg(not(target_arch = "wasm32"))]
pub mod render_view;
#[cfg(target_arch = "wasm32")]
pub mod reset;
#[cfg(target_arch = "wasm32")]
pub mod save;
#[cfg(target_arch = "wasm32")]
pub mod send_json;
#[cfg(target_arch = "wasm32")]
pub mod show_mutation_error;
#[cfg(target_arch = "wasm32")]
pub mod start;
pub mod table;
pub mod table_body;
pub mod table_caption;
pub mod table_cell;
pub mod table_footer;
pub mod table_head;
pub mod table_header;
#[cfg(not(target_arch = "wasm32"))]
pub mod table_pagination;
pub mod table_row;
pub mod table_wrapper;
#[cfg(all(not(target_arch = "wasm32"), test))]
pub mod test_crud_tests;
#[cfg(all(not(target_arch = "wasm32"), test))]
pub mod test_data_grid_tests;
#[cfg(test)]
pub mod test_domain_types_ssr_tests_document;
#[cfg(test)]
pub mod test_domain_types_ssr_tests_navigation;
#[cfg(test)]
pub mod test_domain_types_ssr_tests_settings;
#[cfg(test)]
pub mod test_domain_types_with_owner_tests;
#[cfg(all(not(target_arch = "wasm32"), test))]
pub mod test_static_pages_tests;
pub mod with_owner;

const _: fn(&str) -> Result<(), bounded_types::bounded_string_error::BoundedStringError> =
    bounded_types::bounded_string::BoundedString::<0, 0>::validate_str;
