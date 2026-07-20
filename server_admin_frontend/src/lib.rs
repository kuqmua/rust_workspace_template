pub mod ssr;

#[derive(Debug, Clone, newtype::IntoInnerFrom)]
pub struct AxumAdminFrontendRouter(axum::Router);

#[must_use]
pub fn routes() -> AxumAdminFrontendRouter {
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
    }
}
