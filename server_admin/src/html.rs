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

#[path = "action_result_impl.rs"]
mod action_result_impl;
#[path = "assignment_ids_impl.rs"]
mod assignment_ids_impl;
#[path = "authenticated_action_impl.rs"]
mod authenticated_action_impl;
#[path = "authenticated_selected_form_impl.rs"]
mod authenticated_selected_form_impl;
#[path = "form_auth_impl.rs"]
mod form_auth_impl;
#[path = "html_page_error_impl.rs"]
mod html_page_error_impl;
#[path = "html_response_impl.rs"]
mod html_response_impl;
#[path = "html_routes.rs"]
pub(super) mod html_routes;
#[path = "optional_setting_impl.rs"]
mod optional_setting_impl;
#[path = "page_context_impl.rs"]
mod page_context_impl;
#[path = "permission_ids_impl.rs"]
mod permission_ids_impl;
#[path = "role_ids_impl.rs"]
mod role_ids_impl;
#[path = "role_path_impl.rs"]
mod role_path_impl;
#[path = "success_redirect_impl.rs"]
mod success_redirect_impl;
#[path = "user_path_impl.rs"]
mod user_path_impl;

#[cfg(test)]
#[path = "application_html_tests.rs"]
mod tests;
