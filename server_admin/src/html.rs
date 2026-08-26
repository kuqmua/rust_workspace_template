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

#[path = "html_action_result.rs"]
mod action_result_impl;
#[path = "html_assignment_ids.rs"]
mod assignment_ids_impl;
#[path = "html_authenticated_action.rs"]
mod authenticated_action_impl;
#[path = "html_authenticated_selected_form.rs"]
mod authenticated_selected_form_impl;
#[path = "html_form_auth.rs"]
mod form_auth_impl;
#[path = "html_html_page_error.rs"]
mod html_page_error_impl;
#[path = "html_html_response.rs"]
mod html_response_impl;
#[path = "html_optional_setting.rs"]
mod optional_setting_impl;
#[path = "html_page_context.rs"]
mod page_context_impl;
#[path = "html_permission_ids.rs"]
mod permission_ids_impl;
#[path = "html_role_ids.rs"]
mod role_ids_impl;
#[path = "html_role_path.rs"]
mod role_path_impl;
#[path = "html_routes.rs"]
pub(super) mod routes;
#[path = "html_success_redirect.rs"]
mod success_redirect_impl;
#[path = "html_user_path.rs"]
mod user_path_impl;

#[cfg(test)]
#[path = "application_html_tests.rs"]
mod tests;
