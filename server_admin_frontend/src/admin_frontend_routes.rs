#[must_use]
pub fn admin_frontend_routes() -> crate::axum_admin_frontend_router::AxumAdminFrontendRouter {
    let static_dir = option_env!("ADMIN_FRONTEND_STATIC_DIR")
        .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/static"));
    crate::axum_admin_frontend_router::AxumAdminFrontendRouter::from(axum::Router::new().nest(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Assets.get(),
        axum::Router::new().fallback(async move |request| {
            tower_http::services::ServeDir::new(static_dir)
                .try_call(request)
                .await
                .map(|response| response.map(axum::body::Body::new))
                .map_err(|error| {
                    crate::admin_assets_error::AdminAssetsError::Read(
                        to_err_string::to_err_string::ToErrString::to_err_string(&error),
                    )
                })
        }),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn routes_build_static_asset_router() {
        let _router = crate::admin_frontend_routes::admin_frontend_routes();
    }
}
