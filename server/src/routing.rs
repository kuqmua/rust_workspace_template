#![allow(clippy::single_call_fn)] // composition helpers each own one explicit route-building responsibility

pub(super) fn frontend_fallback_routes() -> server_runtime_http::AxumRouter {
    server_runtime_http::AxumRouter::from(axum::Router::new().fallback(async || {
        axum::response::Redirect::to(server_admin_contract::AdminFrontendPath::SignIn.get())
    }))
}

pub(super) fn mount_service_routes(
    operational_routes: server_runtime_http::AxumRouter,
    api_routes: super::AxumApiRoutes,
    body_maximum_bytes: super::HttpBodyMaximumBytes,
) -> server_runtime_http::AxumRouter {
    server_runtime_http::AxumRouter::from(
        axum::Router::new()
            .merge(axum::Router::from(operational_routes).reset_fallback())
            .nest(
                constants_str::V1,
                api_routes
                    .0
                    .layer(axum::extract::DefaultBodyLimit::max(body_maximum_bytes.0)),
            ),
    )
}

pub(super) fn mk_api_routes(
    app_state: &super::StdSharedServerAppState,
    admin_auth_state: server_admin::auth::StdSharedAdminAuthSvcState,
    metrics_handle: super::MetricsExporterPrometheusHandle,
) -> super::AxumApiRoutes {
    let generated_admin_auth_state = admin_auth_state.clone();
    let generated_table_logic_state: std::sync::Arc<
        dyn server_admin::CombinationOfAppStateLogicTraits,
    > = std::sync::Arc::<server_app_state::ServerAppState<'static>>::clone(app_state.get());
    let generated_table_state =
        server_admin::generated_tables::StdSharedAdminGeneratedTableState::from(
            generated_table_logic_state,
        );
    let generated_table_routes = axum::Router::from(
        server_admin::generated_tables::generated_routes(&generated_table_state),
    );
    let open_api_contract = server_admin_contract::AdminRoute::OpenApi.contract();
    let documented_admin_routes = if *app_state.config.admin_swagger_enabled {
        generated_table_routes.route(
            open_api_contract.path().as_ref(),
            axum::routing::on(
                axum::routing::MethodFilter::from(frontend_contract::axum_method_filter(
                    open_api_contract.method(),
                )),
                async || {
                    axum::Json(utoipa::openapi::OpenApi::from(
                        server_admin::generated_tables::generated_open_api(),
                    ))
                },
            ),
        )
    } else {
        generated_table_routes
    }
    .method_not_allowed_fallback(async || frontend_contract::ApiProblemError::MethodNotAllowed);
    let metrics_contract = server_admin_contract::AdminRoute::Metrics.contract();
    let secured_admin_routes = documented_admin_routes
        .route(
            metrics_contract.path().as_ref(),
            axum::routing::on(
                axum::routing::MethodFilter::from(frontend_contract::axum_method_filter(
                    metrics_contract.method(),
                )),
                async move || {
                    server_runtime_http::MetricsResponseBody::try_from(metrics_handle.0.render())
                        .map(|body| {
                            axum::response::IntoResponse::into_response((
                                axum::http::StatusCode::OK,
                                body.into_inner(),
                            ))
                        })
                        .map_err(super::AdminMetricsError::Render)
                },
            ),
        )
        .route_layer(server_admin::AdminGeneratedAuthLayer::from(
            generated_admin_auth_state,
        ));
    super::AxumApiRoutes::from(
        axum::Router::new()
            .nest(
                server_admin_contract::AdminFrontendPath::Root.get(),
                axum::Router::from(server_admin::auth::routes(admin_auth_state)),
            )
            .nest(
                server_admin_contract::AdminFrontendPath::Root.get(),
                secured_admin_routes,
            ),
    )
}
