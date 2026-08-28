#![allow(
    clippy::shadow_reuse,
    reason = "form adapters deliberately replace unvalidated extractor values with validated domain values"
)]

// Root-owned module compatibility wrappers.
mod actions {
    pub use crate::application_html_actions::*;
}
mod forms {
    pub use crate::application_html_forms::*;
}
mod pages {
    pub use crate::application_html_pages::*;
}
mod action_result_impl {
    pub use crate::action_result_impl::*;
}
mod assignment_ids_impl {
    pub use crate::assignment_ids_impl::*;
}
mod authenticated_action_impl {
    pub use crate::authenticated_action_impl::*;
}
mod authenticated_selected_form_impl {
    pub use crate::authenticated_selected_form_impl::*;
}
mod form_auth_impl {
    pub use crate::form_auth_impl::*;
}
mod html_page_error_impl {
    pub use crate::html_page_error_impl::*;
}
mod html_response_impl {
    pub use crate::html_response_impl::*;
}
pub(crate) mod html_routes {
    pub use crate::html_routes::*;
}
mod optional_setting_impl {
    pub use crate::optional_setting_impl::*;
}
mod page_context_impl {
    pub use crate::page_context_impl::*;
}
mod permission_ids_impl {
    pub use crate::permission_ids_impl::*;
}
mod role_ids_impl {
    pub use crate::role_ids_impl::*;
}
mod role_path_impl {
    pub use crate::role_path_impl::*;
}
mod success_redirect_impl {
    pub use crate::success_redirect_impl::*;
}
mod user_path_impl {
    pub use crate::user_path_impl::*;
}
