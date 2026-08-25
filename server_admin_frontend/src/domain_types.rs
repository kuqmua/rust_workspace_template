mod shared;
#[cfg(not(target_arch = "wasm32"))]
pub mod ssr;
#[cfg(target_arch = "wasm32")]
mod start;
mod with_owner;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct AxumAdminFrontendRouter(axum::Router);

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, thiserror::Error)]
enum AdminAssetsError {
    #[error("administrator asset read failed: {0}")]
    Read(to_err_string::domain_types::ErrorText),
}
#[cfg(not(target_arch = "wasm32"))]
impl axum::response::IntoResponse for AdminAssetsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Read(_error) => axum::response::IntoResponse::into_response(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[must_use]
pub fn routes() -> AxumAdminFrontendRouter {
    let static_dir = option_env!("ADMIN_FRONTEND_STATIC_DIR")
        .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));
    AxumAdminFrontendRouter::from(axum::Router::new().nest(
        server_admin_contract::domain_types::AdminFrontendPath::Assets.get(),
        axum::Router::new().fallback(async move |request| {
            tower_http::services::ServeDir::new(static_dir)
                .try_call(request)
                .await
                .map(|response| response.map(axum::body::Body::new))
                .map_err(|error| {
                    AdminAssetsError::Read(to_err_string::domain_types::ToErrString::to_err_string(
                        &error,
                    ))
                })
        }),
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
