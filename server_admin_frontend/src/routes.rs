#[must_use]
pub fn admin_frontend_routes() -> crate::domain_types::AxumAdminFrontendRouter {
    let static_dir = option_env!("ADMIN_FRONTEND_STATIC_DIR")
        .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));
    crate::domain_types::AxumAdminFrontendRouter::from(axum::Router::new().nest(
        server_admin_contract::domain_types::AdminFrontendPath::Assets.get(),
        axum::Router::new().fallback(async move |request| {
            tower_http::services::ServeDir::new(static_dir)
                .try_call(request)
                .await
                .map(|response| response.map(axum::body::Body::new))
                .map_err(|error| {
                    crate::domain_types::admin_assets_error::AdminAssetsError::Read(
                        to_err_string::domain_types::ToErrString::to_err_string(&error),
                    )
                })
        }),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn routes_build_static_asset_router() {
        let _router = super::admin_frontend_routes();
    }
}
