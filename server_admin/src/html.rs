#![allow(clippy::single_call_fn)] // each server-rendered HTML endpoint is registered once in the Axum route inventory
#![allow(
    clippy::shadow_reuse,
    reason = "form adapters deliberately replace unvalidated extractor values with validated domain values"
)]

#[path = "application_html_actions.rs"]
mod actions;
#[path = "application_html_forms.rs"]
mod forms;
#[path = "application_html_pages.rs"]
mod pages;

#[path = "html_action_result_impl.rs"]
mod action_result_impl;
#[path = "html_assignment_ids_impl.rs"]
mod assignment_ids_impl;
#[path = "html_authenticated_action_impl.rs"]
mod authenticated_action_impl;
#[path = "html_authenticated_selected_form_impl.rs"]
mod authenticated_selected_form_impl;
#[path = "html_form_auth_impl.rs"]
mod form_auth_impl;
#[path = "html_html_page_error_impl.rs"]
mod html_page_error_impl;
#[path = "html_html_response_impl.rs"]
mod html_response_impl;
#[path = "html_optional_setting_impl.rs"]
mod optional_setting_impl;
#[path = "html_page_context_impl.rs"]
mod page_context_impl;
#[path = "html_permission_ids_impl.rs"]
mod permission_ids_impl;
#[path = "html_role_ids_impl.rs"]
mod role_ids_impl;
#[path = "html_role_path_impl.rs"]
mod role_path_impl;
#[path = "html_routes.rs"]
pub(super) mod routes;
#[path = "html_success_redirect_impl.rs"]
mod success_redirect_impl;
#[path = "html_user_path_impl.rs"]
mod user_path_impl;

#[cfg(test)]
#[path = "application_html_tests.rs"]
mod tests;
