#[must_use]
pub fn common_routes(
    app_state_b9fc2d94: crate::ArcCommonRoutesAppState,
) -> crate::AxumCommonRoutes {
    crate::AxumCommonRoutes::from(
        super::common_route_registry::CommonRouteRegistry::router()
            .fallback(
                async |uri: axum::http::Uri, axum::extract::State(app_state_19103bd5_raw)| {
                    let app_state_19103bd5: crate::ArcCommonRoutesAppState = app_state_19103bd5_raw;
                    let uri_suffix = uri
                        .path_and_query()
                        .map_or_else(|| uri.path(), |value| value.as_str());
                    let capacity = constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX
                        .len()
                        .saturating_add(uri_suffix.len());
                    let mut message = String::with_capacity(capacity);
                    message.push_str(constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX);
                    message.push_str(uri_suffix);
                    crate::CommonNotFoundError::NotFound(crate::NotFoundPayload {
                        commit: git_info::GitCommitLinkProvider::build_git_commit_link_cow(
                            app_state_19103bd5.get(),
                        ),
                        message: to_err_string::domain_types::ErrorText::try_from(message)
                            .unwrap_or_else(to_err_string::domain_types::ErrorText::from),
                        open_api_specification: crate::OpenApiSpecificationPath::from(
                            constants_str::COMMON_ROUTES_SWAGGER_UI,
                        ),
                    })
                },
            )
            .with_state(app_state_b9fc2d94),
    )
}
