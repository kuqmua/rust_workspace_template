pub mod ssr;

#[derive(Debug, Clone, newtype::IntoInnerFrom)]
pub struct AxumAdminFrontendRouter(axum::Router);

#[must_use]
pub fn routes() -> AxumAdminFrontendRouter {
    static_routes()
}

#[must_use]
pub fn routes_without_swagger() -> AxumAdminFrontendRouter {
    static_routes()
}

fn static_routes() -> AxumAdminFrontendRouter {
    let static_dir = option_env!("ADMIN_FRONTEND_STATIC_DIR")
        .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));
    AxumAdminFrontendRouter(axum::Router::new().nest_service(
        server_admin_contract::AdminFrontendPath::Assets.get(),
        tower_http::services::ServeDir::new(static_dir),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn routes_build_static_asset_router() {
        let _router = super::routes();
        let _router_without_swagger = super::routes_without_swagger();
    }
}
