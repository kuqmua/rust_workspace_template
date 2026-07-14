#![cfg_attr(target_arch = "wasm32", allow(clippy::absolute_paths))]
#[cfg(target_arch = "wasm32")]
pub mod app;
#[cfg(any(target_arch = "wasm32", test))]
mod auth_keep_alive;
#[cfg(any(target_arch = "wasm32", test))]
mod table_state;
#[cfg(target_arch = "wasm32")]
mod transport;
#[cfg(not(target_arch = "wasm32"))]
const ADMIN_PAGE_PATHS: [&str; 10] = [
    server_admin_contract::admin_page_paths::ROOT,
    server_admin_contract::admin_page_paths::SIGN_IN,
    server_admin_contract::admin_page_paths::USERS,
    server_admin_contract::admin_page_paths::ROLES,
    server_admin_contract::admin_page_paths::PERMISSIONS,
    server_admin_contract::admin_page_paths::AUDIT,
    server_admin_contract::admin_page_paths::SETTINGS,
    server_admin_contract::admin_page_paths::METRICS,
    server_admin_contract::admin_page_paths::VERSION,
    server_admin_contract::admin_page_paths::OPEN_API,
];
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct AxumAdminFrontendRouter(axum::Router);
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
struct AdminSwaggerEnabled(bool);
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
        .filter(|path| {
            swagger_enabled.0 || *path != server_admin_contract::admin_page_paths::OPEN_API
        })
        .fold(axum::Router::new(), |router, path| {
            router.route_service(path, tower_http::services::ServeFile::new(index_path))
        })
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            axum::http::header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static("no-cache, no-store, must-revalidate"),
        ));
    AxumAdminFrontendRouter(page_routes.nest_service(
        server_admin_contract::admin_page_paths::ASSETS,
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
        assert!(
            super::ADMIN_PAGE_PATHS.contains(&server_admin_contract::admin_page_paths::SIGN_IN)
        );
        assert!(super::ADMIN_PAGE_PATHS.contains(&server_admin_contract::admin_page_paths::USERS));
        assert!(super::ADMIN_PAGE_PATHS.contains(&server_admin_contract::admin_page_paths::AUDIT));
        assert!(
            super::ADMIN_PAGE_PATHS.contains(&server_admin_contract::admin_page_paths::OPEN_API)
        );
    }
    #[test]
    #[allow(clippy::needless_for_each)] // workspace policy prohibits for loops in test inventories
    fn leptos_client_uses_typed_operational_api_contracts() {
        let script = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/app.rs"))
            .expect("fe89c42a");
        let pages =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/app/pages.rs"))
                .expect("89a2c4de");
        let transport =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/transport.rs"))
                .expect("320c7d1e");
        [
            "AdminRoute::Users",
            "AdminRoute::Roles",
            "AdminRoute::Permissions",
            "AdminRoute::Audit",
            "AdminRoute::Settings",
            "AdminRoute::Metrics",
            "AdminRoute::OpenApi",
            "AdminRoute::SignIn",
            "AdminRoute::Refresh",
            "AdminRoute::SignOut",
        ]
        .into_iter()
        .for_each(|contract| {
            assert!(
                script.contains(contract) || pages.contains(contract),
                "missing SPA contract: {contract}"
            );
        });
        assert!(transport.contains("TransportRequest"));
        assert!(transport.contains("RequestCredentials::Include"));
        assert!(transport.contains("X-CSRF-Token"));
        assert!(!script.contains("/api/v1/admin/users"));
        assert!(!pages.contains("/api/v1/admin/users"));
        assert!(!script.contains("serde_json::json!"));
        assert!(!pages.contains("serde_json::json!"));
        assert!(!script.contains("fetch("));
        assert!(
            !std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/swagger.js"))
                .exists()
        );
    }
    #[test]
    #[allow(clippy::needless_for_each)] // workspace source policy intentionally avoids for loops
    fn header_navigation_and_table_controls_are_part_of_the_spa() {
        let pages =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/app/pages.rs"))
                .expect("64e815ee");
        let tables =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/app/tables.rs"))
                .expect("962197b5");
        let styles =
            std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/static/style.css"))
                .expect("3cc52ac5");
        assert!(pages.contains("<header class=\"topbar\">"));
        assert!(pages.contains("<nav aria-label=\"Admin sections\">"));
        assert!(!pages.contains("<header class=\"sidebar\">"));
        assert_eq!(tables.matches("<TableTools state").count(), 4usize);
        assert_eq!(tables.matches("<TablePager state total").count(), 4usize);
        [
            "aria-label=\"Filter rows\"",
            "aria-label=\"Sort field\"",
            "aria-label=\"Toggle sort direction\"",
            "aria-label=\"Rows per page\"",
            "aria-label=\"Previous page\"",
            "aria-label=\"Next page\"",
        ]
        .into_iter()
        .for_each(|control| assert!(tables.contains(control), "missing table control: {control}"));
        assert!(styles.contains(".topbar nav"));
        assert!(styles.contains(".table-tools"));
        assert!(styles.contains(".table-footer"));
    }
}
