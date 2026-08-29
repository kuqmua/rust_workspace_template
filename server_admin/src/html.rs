// Root-owned module compatibility wrappers.
mod actions {
    pub use super::super::application_html_actions::*;
}
mod forms {
    pub use super::super::application_html_forms::*;
}
mod pages {
    pub use super::super::application_html_pages::*;
}
mod action_result_impl {
    pub use super::super::action_result_impl::*;
}
mod assignment_ids_impl {
    pub use super::super::assignment_ids_impl::*;
}
mod authenticated_action_impl {
    pub use super::super::authenticated_action_impl::*;
}
mod form_auth_impl {
    pub use super::super::form_auth_impl::*;
}
mod html_page_error_impl {
    pub use super::super::html_page_error_impl::*;
}
mod html_response_impl {
    pub use super::super::html_response_impl::*;
}
pub(crate) mod html_routes {
    pub use super::super::html_routes::*;
}
mod optional_setting_impl {
    pub use super::super::optional_setting_impl::*;
}
mod page_context_impl {
    pub use super::super::page_context_impl::*;
}
mod permission_ids_impl {
    pub use super::super::permission_ids_impl::*;
}
mod role_ids_impl {
    pub use super::super::role_ids_impl::*;
}
mod role_path_impl {
    pub use super::super::role_path_impl::*;
}
mod success_redirect_impl {
    pub use super::super::success_redirect_impl::*;
}
mod user_path_impl {
    pub use super::super::user_path_impl::*;
}
