pub(crate) use admin_html_action_route_registry::AdminHtmlActionRouteRegistry;

#[frontend_contract::route_operation]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::domain_types::AdminFrontendPath::Users.get(),
    ))
}

// Root-owned module compatibility wrappers.
mod admin_html_action_route_registry {
    pub use crate::admin_html_action_route_registry::*;
}
