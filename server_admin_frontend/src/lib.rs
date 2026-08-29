#![allow(
    unused_imports,
    unreachable_pub,
    clippy::arbitrary_source_item_ordering,
    clippy::shadow_reuse,
    clippy::tests_outside_test_module,
    clippy::unused_trait_names,
    clippy::wildcard_imports,
    reason = "root-owned frontend modules retain the component, signal, generated-view, test-leaf, and compatibility lint contracts previously scoped across their nested owner modules"
)]

mod admin_alert;
pub(crate) use crate::admin_alert::*;
#[cfg(any(target_arch = "wasm32", test))]
mod admin_alert_dialog;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) use crate::admin_alert_dialog::*;
mod admin_alert_variant;
pub(crate) use crate::admin_alert_variant::*;
#[cfg(target_arch = "wasm32")]
mod admin_api_url;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_api_url::*;
#[cfg(target_arch = "wasm32")]
mod admin_api_url_with_suffix;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_api_url_with_suffix::*;
#[cfg(target_arch = "wasm32")]
mod admin_app;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_app::*;
mod admin_assets_error;
pub(crate) use crate::admin_assets_error::*;
mod admin_badge;
pub(crate) use crate::admin_badge::*;
mod admin_badge_variant;
pub(crate) use crate::admin_badge_variant::*;
mod admin_button;
pub(crate) use crate::admin_button::*;
mod admin_button_kind;
pub(crate) use crate::admin_button_kind::*;
mod admin_button_link;
pub(crate) use crate::admin_button_link::*;
mod admin_button_variant;
pub(crate) use crate::admin_button_variant::*;
mod admin_card;
pub(crate) use crate::admin_card::*;
mod admin_card_description;
pub(crate) use crate::admin_card_description::*;
mod admin_card_footer;
pub(crate) use crate::admin_card_footer::*;
mod admin_card_header;
pub(crate) use crate::admin_card_header::*;
mod admin_card_title;
pub(crate) use crate::admin_card_title::*;
mod admin_card_variant;
pub(crate) use crate::admin_card_variant::*;
#[cfg(target_arch = "wasm32")]
mod admin_change_password;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_change_password::*;
mod admin_checkbox;
pub(crate) use crate::admin_checkbox::*;
#[cfg(target_arch = "wasm32")]
mod admin_csr_api_url;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_csr_api_url::*;
#[cfg(target_arch = "wasm32")]
mod admin_csr_api_url_suffix_ref;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_csr_api_url_suffix_ref::*;
#[cfg(target_arch = "wasm32")]
mod admin_csr_query;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_csr_query::*;
#[cfg(target_arch = "wasm32")]
mod admin_csrf_token;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_csrf_token::*;
#[cfg(target_arch = "wasm32")]
mod admin_data_grid;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_data_grid::*;
mod admin_data_grid_input_type;
pub(crate) use crate::admin_data_grid_input_type::*;
mod admin_data_table_grid;
pub(crate) use crate::admin_data_table_grid::*;
#[cfg(any(target_arch = "wasm32", test))]
mod admin_empty;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) use crate::admin_empty::*;
mod admin_field;
pub(crate) use crate::admin_field::*;
mod admin_field_label;
pub(crate) use crate::admin_field_label::*;
mod admin_filter_hidden_inputs;
pub(crate) use crate::admin_filter_hidden_inputs::*;
#[cfg(target_arch = "wasm32")]
mod admin_http_status;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_http_status::*;
mod admin_input;
pub(crate) use crate::admin_input::*;
mod admin_input_group;
pub(crate) use crate::admin_input_group::*;
mod admin_input_kind;
pub(crate) use crate::admin_input_kind::*;
mod admin_input_name;
pub(crate) use crate::admin_input_name::*;
mod admin_joined_text;
pub(crate) use crate::admin_joined_text::*;
mod admin_joined_text_try_from_string_error;
pub(crate) use crate::admin_joined_text_try_from_string_error::*;
#[cfg(target_arch = "wasm32")]
mod admin_load_state;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_load_state::*;
#[cfg(target_arch = "wasm32")]
mod admin_mutation_method;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_mutation_method::*;
mod admin_navigation_link;
pub(crate) use crate::admin_navigation_link::*;
mod admin_page_nav_disabled;
pub(crate) use crate::admin_page_nav_disabled::*;
mod admin_page_range;
pub(crate) use crate::admin_page_range::*;
#[cfg(target_arch = "wasm32")]
mod admin_pagination;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_pagination::*;
#[cfg(target_arch = "wasm32")]
mod admin_permissions_view;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_permissions_view::*;
#[cfg(target_arch = "wasm32")]
mod admin_profile_account;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_profile_account::*;
#[cfg(target_arch = "wasm32")]
mod admin_profile_view;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_profile_view::*;
mod admin_role_permissions;
pub(crate) use crate::admin_role_permissions::*;
#[cfg(target_arch = "wasm32")]
mod admin_roles_view;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_roles_view::*;
#[cfg(target_arch = "wasm32")]
mod admin_route_path_url;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_route_path_url::*;
#[cfg(target_arch = "wasm32")]
mod admin_sessions_view;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_sessions_view::*;
mod admin_setting_disabled;
pub(crate) use crate::admin_setting_disabled::*;
mod admin_setting_input_value;
pub(crate) use crate::admin_setting_input_value::*;
mod admin_setting_inputs;
pub(crate) use crate::admin_setting_inputs::*;
mod admin_setting_required;
pub(crate) use crate::admin_setting_required::*;
mod admin_settings_form_signals;
pub(crate) use crate::admin_settings_form_signals::*;
mod admin_settings_form_values;
pub(crate) use crate::admin_settings_form_values::*;
#[cfg(target_arch = "wasm32")]
mod admin_settings_view;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_settings_view::*;
mod admin_sidebar;
pub(crate) use crate::admin_sidebar::*;
mod admin_sidebar_item;
pub(crate) use crate::admin_sidebar_item::*;
mod admin_spinner;
pub(crate) use crate::admin_spinner::*;
#[cfg(not(target_arch = "wasm32"))]
mod admin_ssr_error_message;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::admin_ssr_error_message::*;
#[cfg(not(target_arch = "wasm32"))]
mod admin_ssr_html;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::admin_ssr_html::*;
#[cfg(not(target_arch = "wasm32"))]
mod admin_ssr_html_try_from_string_error;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::admin_ssr_html_try_from_string_error::*;
#[cfg(not(target_arch = "wasm32"))]
mod admin_ssr_text;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::admin_ssr_text::*;
#[cfg(not(target_arch = "wasm32"))]
mod admin_ssr_text_try_from_string_error;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::admin_ssr_text_try_from_string_error::*;
#[cfg(test)]
mod admin_ssr_view_ext;
#[cfg(test)]
pub(crate) use crate::admin_ssr_view_ext::*;
#[cfg(target_arch = "wasm32")]
mod admin_table_load_error;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_table_load_error::*;
mod admin_table_query_direction;
pub(crate) use crate::admin_table_query_direction::*;
mod admin_table_query_hidden_inputs;
pub(crate) use crate::admin_table_query_hidden_inputs::*;
mod admin_textarea;
pub(crate) use crate::admin_textarea::*;
mod admin_user_roles;
pub(crate) use crate::admin_user_roles::*;
#[cfg(target_arch = "wasm32")]
mod admin_users_view;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::admin_users_view::*;
mod axum_admin_frontend_router;
pub(crate) use crate::axum_admin_frontend_router::*;
#[cfg(all(not(target_arch = "wasm32"), test))]
mod crud_tests;
#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) use crate::crud_tests::*;
#[cfg(not(target_arch = "wasm32"))]
mod crud_render_role_create;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::crud_render_role_create::*;
#[cfg(not(target_arch = "wasm32"))]
mod crud_render_role_manage;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::crud_render_role_manage::*;
#[cfg(not(target_arch = "wasm32"))]
mod crud_render_shell;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::crud_render_shell::*;
#[cfg(not(target_arch = "wasm32"))]
mod crud_render_user_create;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::crud_render_user_create::*;
#[cfg(not(target_arch = "wasm32"))]
mod crud_render_user_manage;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::crud_render_user_manage::*;
#[cfg(target_arch = "wasm32")]
mod csr_admin_nav;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::csr_admin_nav::*;
#[cfg(target_arch = "wasm32")]
mod csr_admin_role_row;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::csr_admin_role_row::*;
#[cfg(target_arch = "wasm32")]
mod csr_admin_user_row;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::csr_admin_user_row::*;
#[cfg(target_arch = "wasm32")]
mod csr_page_from_location;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::csr_page_from_location::*;
#[cfg(target_arch = "wasm32")]
mod csrf_token;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::csrf_token::*;
#[cfg(all(not(target_arch = "wasm32"), test))]
mod data_grid_tests;
#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) use crate::data_grid_tests::*;
#[cfg(not(target_arch = "wasm32"))]
mod data_table_grid;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::data_table_grid::*;
#[cfg(not(target_arch = "wasm32"))]
mod data_tables;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::data_tables::*;
mod domain_types_shared_settings;
pub(crate) use crate::domain_types_shared_settings::*;
mod domain_types_shared_settings_input;
pub(crate) use crate::domain_types_shared_settings_input::*;
#[cfg(not(target_arch = "wasm32"))]
mod domain_types_ssr_document;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::domain_types_ssr_document::*;
#[cfg(test)]
mod domain_types_ssr_tests;
#[cfg(test)]
pub(crate) use crate::domain_types_ssr_tests::*;
#[cfg(test)]
mod domain_types_ssr_tests_document;
#[cfg(test)]
pub(crate) use crate::domain_types_ssr_tests_document::*;
#[cfg(test)]
mod domain_types_ssr_tests_navigation;
#[cfg(test)]
pub(crate) use crate::domain_types_ssr_tests_navigation::*;
#[cfg(test)]
mod domain_types_ssr_tests_settings;
#[cfg(test)]
pub(crate) use crate::domain_types_ssr_tests_settings::*;
#[cfg(target_arch = "wasm32")]
mod domain_types_start_http_mutation;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::domain_types_start_http_mutation::*;
#[cfg(target_arch = "wasm32")]
mod domain_types_start_mutation;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::domain_types_start_mutation::*;
mod domain_types_with_owner_alert;
pub(crate) use crate::domain_types_with_owner_alert::*;
mod domain_types_with_owner_badge;
pub(crate) use crate::domain_types_with_owner_badge::*;
mod domain_types_with_owner_button;
pub(crate) use crate::domain_types_with_owner_button::*;
mod domain_types_with_owner_card;
pub(crate) use crate::domain_types_with_owner_card::*;
mod domain_types_with_owner_field;
pub(crate) use crate::domain_types_with_owner_field::*;
mod domain_types_with_owner_input;
pub(crate) use crate::domain_types_with_owner_input::*;
mod domain_types_with_owner_navigation;
pub(crate) use crate::domain_types_with_owner_navigation::*;
mod domain_types_with_owner_table;
pub(crate) use crate::domain_types_with_owner_table::*;
#[cfg(test)]
mod domain_types_with_owner_tests;
#[cfg(test)]
pub(crate) use crate::domain_types_with_owner_tests::*;
#[cfg(target_arch = "wasm32")]
mod fetch_json;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::fetch_json::*;
#[cfg(target_arch = "wasm32")]
mod fetch_page;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::fetch_page::*;
#[cfg(target_arch = "wasm32")]
mod http;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::http::*;
mod join_text;
pub(crate) use crate::join_text::*;
mod leptos_admin_filter_operation_signal;
pub(crate) use crate::leptos_admin_filter_operation_signal::*;
mod leptos_admin_input_signal;
pub(crate) use crate::leptos_admin_input_signal::*;
#[cfg(target_arch = "wasm32")]
mod location;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::location::*;
#[cfg(not(target_arch = "wasm32"))]
mod page_render_with_access;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::page_render_with_access::*;
#[cfg(not(target_arch = "wasm32"))]
mod page_render_with_table_access;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::page_render_with_table_access::*;
mod pagination;
pub(crate) use crate::pagination::*;
#[cfg(target_arch = "wasm32")]
mod reload_after;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::reload_after::*;
#[cfg(not(target_arch = "wasm32"))]
mod render;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_admin_csr;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_admin_csr::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_admin_page;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_admin_page::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_admin_page_with_access;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_admin_page_with_access::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_admin_page_with_table_access;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_admin_page_with_table_access::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_data_tables;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_data_tables::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_data_tables_csr;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_data_tables_csr::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_document;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_document::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_permissions;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_permissions::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_profile;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_profile::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_roles;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_roles::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_sessions;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_sessions::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_settings;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_settings::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_text_page;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_text_page::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_text_page_with_access;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_text_page_with_access::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_users;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_users::*;
#[cfg(not(target_arch = "wasm32"))]
mod render_view;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::render_view::*;
#[cfg(target_arch = "wasm32")]
mod reset;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::reset::*;
mod routes;
pub(crate) use crate::routes::*;
#[cfg(target_arch = "wasm32")]
mod save;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::save::*;
#[cfg(target_arch = "wasm32")]
mod send_json;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::send_json::*;
mod shared;
pub(crate) use crate::shared::*;
#[cfg(target_arch = "wasm32")]
mod show_mutation_error;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::show_mutation_error::*;
#[cfg(not(target_arch = "wasm32"))]
pub mod ssr;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::ssr::*;
#[cfg(target_arch = "wasm32")]
mod start;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::start::*;
#[cfg(target_arch = "wasm32")]
mod state;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::state::*;
#[cfg(all(not(target_arch = "wasm32"), test))]
mod static_pages_tests;
#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) use crate::static_pages_tests::*;
mod table;
pub(crate) use crate::table::*;
mod table_body;
pub(crate) use crate::table_body::*;
mod table_caption;
pub(crate) use crate::table_caption::*;
mod table_cell;
pub(crate) use crate::table_cell::*;
mod table_filters;
pub(crate) use crate::table_filters::*;
mod table_footer;
pub(crate) use crate::table_footer::*;
mod table_head;
pub(crate) use crate::table_head::*;
mod table_header;
pub(crate) use crate::table_header::*;
#[cfg(not(target_arch = "wasm32"))]
mod table_pagination;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::table_pagination::*;
mod table_row;
pub(crate) use crate::table_row::*;
mod table_wrapper;
pub(crate) use crate::table_wrapper::*;
mod text;
pub(crate) use crate::text::*;
#[cfg(not(target_arch = "wasm32"))]
mod text_page;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use crate::text_page::*;
#[cfg(target_arch = "wasm32")]
mod url;
#[cfg(target_arch = "wasm32")]
pub(crate) use crate::url::*;
mod values;
pub(crate) use crate::values::*;
mod with_owner;
pub(crate) use crate::with_owner::*;

pub mod domain_types;
