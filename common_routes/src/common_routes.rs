#[must_use]
pub fn common_routes(
    app_state_b9fc2d94: crate::domain_types::ArcCommonRoutesAppState,
) -> crate::domain_types::AxumCommonRoutes {
    crate::domain_types::AxumCommonRoutes::from(
        super::common_route_registry::CommonRouteRegistry::router()
            .fallback(async |uri, axum::extract::State(app_state_19103bd5_raw)| {
                let app_state_19103bd5: crate::domain_types::ArcCommonRoutesAppState =
                    app_state_19103bd5_raw;
                crate::domain_types::CommonNotFoundError::NotFound(
                    crate::domain_types::make_not_found_payload(
                        crate::domain_types::AxumHttpUriRef::from(&uri),
                        git_info::domain_types::GitCommitLinkProvider::build_git_commit_link_cow(
                            app_state_19103bd5.get(),
                        ),
                    ),
                )
            })
            .with_state(app_state_b9fc2d94),
    )
}
