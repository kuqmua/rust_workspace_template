#![cfg_attr(target_arch = "wasm32", allow(clippy::absolute_paths))]
#[cfg(target_arch = "wasm32")]
pub mod app;
#[cfg(not(target_arch = "wasm32"))]
const ADMIN_OPEN_API_HTML: &str = "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Admin API</title><link rel=\"stylesheet\" href=\"/admin/assets/style.css\"></head><body><main><h1>Admin API</h1><pre id=\"openapi\">Loading OpenAPI document...</pre></main><script src=\"/admin/assets/swagger.js\" defer></script></body></html>";
#[cfg(not(target_arch = "wasm32"))]
const ADMIN_PAGE_PATHS: [&str; 10] = [
    "/admin",
    "/admin/sign-in",
    "/admin/users",
    "/admin/roles",
    "/admin/permissions",
    "/admin/audit-log",
    "/admin/system-settings",
    "/admin/metrics",
    "/admin/version",
    "/admin/swagger-ui",
];
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct AxumAdminFrontendRouter(axum::Router);
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug)]
struct AxumAdminOpenApiHtml(axum::response::Html<&'static str>);
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
struct AdminSwaggerEnabled(bool);
#[cfg(not(target_arch = "wasm32"))]
impl axum::response::IntoResponse for AxumAdminOpenApiHtml {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::single_call_fn)] // dedicated documentation handler is registered once
async fn open_api() -> AxumAdminOpenApiHtml {
    AxumAdminOpenApiHtml(axum::response::Html(ADMIN_OPEN_API_HTML))
}
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn routes() -> AxumAdminFrontendRouter {
    routes_with_swagger(AdminSwaggerEnabled::from(true))
}
#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn routes_without_swagger() -> AxumAdminFrontendRouter {
    routes_with_swagger(AdminSwaggerEnabled::from(false))
}
#[cfg(not(target_arch = "wasm32"))]
fn routes_with_swagger(swagger_enabled: AdminSwaggerEnabled) -> AxumAdminFrontendRouter {
    let index_path = concat!(env!("CARGO_MANIFEST_DIR"), "/dist/index.html");
    let page_routes = ADMIN_PAGE_PATHS
        .into_iter()
        .filter(|path| *path != "/admin/swagger-ui")
        .fold(axum::Router::new(), |router, path| {
            router.route_service(path, tower_http::services::ServeFile::new(index_path))
        });
    let enabled_page_routes = if swagger_enabled.0 {
        page_routes.route("/admin/swagger-ui", axum::routing::get(open_api))
    } else {
        page_routes
    };
    AxumAdminFrontendRouter(enabled_page_routes.nest_service(
        "/admin/assets",
        tower_http::services::ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/dist")).fallback(
            tower_http::services::ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        ),
    ))
}
#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::arbitrary_source_item_ordering)] // tests stay after the production route builder
mod tests {
    #[test]
    fn routes_builds_router() {
        let _router = super::routes();
        let _router_without_swagger = super::routes_without_swagger();
    }
    #[test]
    fn page_inventory_contains_auth_and_operations() {
        assert!(super::ADMIN_PAGE_PATHS.contains(&"/admin/sign-in"));
        assert!(super::ADMIN_PAGE_PATHS.contains(&"/admin/users"));
        assert!(super::ADMIN_PAGE_PATHS.contains(&"/admin/audit-log"));
        assert!(super::ADMIN_PAGE_PATHS.contains(&"/admin/swagger-ui"));
    }
    #[test]
    #[allow(clippy::needless_for_each)] // workspace policy prohibits for loops in test inventories
    fn leptos_client_uses_typed_operational_api_contracts() {
        let script = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/app.rs"))
            .expect("fe89c42a");
        [
            "AdminRoute::Users",
            "AdminRoute::Roles",
            "AdminRoute::Permissions",
            "AdminRoute::Audit",
            "AdminRoute::Settings",
            "AdminRoute::Metrics",
            "AdminRoute::SignIn",
            "AdminRoute::SignOut",
            "TransportRequest::new",
            "RequestCredentials::Include",
        ]
        .into_iter()
        .for_each(|contract| {
            assert!(
                script.contains(contract),
                "missing SPA contract: {contract}"
            );
        });
        assert!(script.contains("X-CSRF-Token"));
        assert!(!script.contains("/api/v1/admin/users"));
        assert!(!script.contains("serde_json::json!"));
    }
}
