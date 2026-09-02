#[must_use]
pub fn common_routes(
    arc_common_routes_app_state: crate::arc_common_routes_app_state::ArcCommonRoutesAppState,
) -> crate::axum_common_routes::AxumCommonRoutes {
    crate::axum_common_routes::AxumCommonRoutes::from(
        super::common_route_registry::router()
            .fallback(
                async |uri: crate::axum_http_uri::AxumHttpUri,
                       axum::extract::State(app_state_19103bd5_raw)| {
                    let app_state_19103bd5: crate::arc_common_routes_app_state::ArcCommonRoutesAppState = app_state_19103bd5_raw;
                    let uri_suffix = uri
                        .get()
                        .path_and_query()
                        .map_or_else(|| uri.get().path(), |value| value.as_str());
                    let capacity = constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX
                        .len()
                        .saturating_add(uri_suffix.len());
                    let mut message = String::with_capacity(capacity);
                    message.push_str(constants_str::COMMON_ROUTES_NO_ROUTE_MSG_PREFIX);
                    message.push_str(uri_suffix);
                    crate::common_not_found_error::CommonNotFoundError::NotFound(
                        crate::not_found_payload::NotFoundPayload::from_parts(
                            git_info::git_commit_link_provider::GitCommitLinkProvider::build_git_commit_link_cow(
                                app_state_19103bd5.get(),
                            ),
                            to_err_string::error_text::ErrorText::try_from(message)
                                .unwrap_or_else(to_err_string::error_text::ErrorText::from),
                            crate::open_api_specification_path::OpenApiSpecificationPath::from(
                                constants_str::COMMON_ROUTES_SWAGGER_UI,
                            ),
                        ),
                    )
                },
            )
            .with_state(arc_common_routes_app_state),
    )
}
