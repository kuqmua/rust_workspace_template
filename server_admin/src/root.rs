#[path = "admin_html_action_route_registry.rs"]
mod admin_html_action_route_registry;

pub(in crate::domain_types::auth::html) use admin_html_action_route_registry::AdminHtmlActionRouteRegistry;

#[frontend_contract::domain_types::route_operation]
pub(in crate::domain_types::auth::html) async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::domain_types::AdminFrontendPath::Users.get(),
    ))
}
