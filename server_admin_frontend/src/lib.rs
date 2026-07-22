#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(not(target_arch = "wasm32"))]
pub mod ssr;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct AxumAdminFrontendRouter(axum::Router);

#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn routes() -> AxumAdminFrontendRouter {
    let static_dir = option_env!("ADMIN_FRONTEND_STATIC_DIR")
        .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));
    AxumAdminFrontendRouter::from(axum::Router::new().nest_service(
        server_admin_contract::AdminFrontendPath::Assets.get(),
        tower_http::services::ServeDir::new(static_dir),
    ))
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "test modules stay last to satisfy clippy::items_after_test_module"
)]
mod tests {
    #[test]
    fn routes_build_static_asset_router() {
        let _router = super::routes();
    }
}
