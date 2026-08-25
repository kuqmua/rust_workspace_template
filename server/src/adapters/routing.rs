#![allow(clippy::single_call_fn)] // composition helpers each own one explicit route-building responsibility

pub(crate) fn frontend_fallback_routes() -> server_runtime_http::domain_types::AxumRouter {
    server_runtime_http::domain_types::AxumRouter::from(axum::Router::new().fallback(async || {
        axum::response::Redirect::to(
            server_admin_contract::domain_types::AdminFrontendPath::SignIn.get(),
        )
    }))
}

pub(crate) fn mount_service_routes(
    operational_routes: server_runtime_http::domain_types::AxumRouter,
    api_routes: crate::domain_types::AxumApiRoutes,
    body_maximum_bytes: crate::domain_types::HttpBodyMaximumBytes,
) -> server_runtime_http::domain_types::AxumRouter {
    server_runtime_http::domain_types::AxumRouter::from(
        axum::Router::new()
            .merge(axum::Router::from(operational_routes).reset_fallback())
            .nest(
                constants_str::V1,
                axum::Router::from(api_routes).layer(axum::extract::DefaultBodyLimit::max(
                    body_maximum_bytes.get(),
                )),
            ),
    )
}

pub(crate) fn mk_api_routes(
    app_state: &crate::domain_types::SharedServerAppStateArc,
    admin_auth_state: server_admin::domain_types::auth::SharedAdminAuthSvcStateArc,
    metrics_handle: crate::domain_types::MetricsExporterPrometheusHandle,
) -> crate::domain_types::AxumApiRoutes {
    let generated_admin_auth_state = admin_auth_state.clone();
    let generated_table_logic_state: std::sync::Arc<
        dyn server_admin::domain_types::CombinationOfAppStateLogicTraits,
    > = std::sync::Arc::<server_app_state::domain_types::ServerAppState<'static>>::clone(
        app_state.get(),
    );
    let generated_table_state =
        server_admin::domain_types::generated_tables::SharedAdminGeneratedTableStateArc::from(
            generated_table_logic_state,
        );
    let generated_table_routes = axum::Router::from(
        server_admin::domain_types::generated_tables::generated_routes(&generated_table_state),
    );
    let open_api_contract = server_admin_contract::domain_types::AdminRoute::OpenApi.contract();
    let documented_admin_routes = if *app_state.config.admin_swagger_enabled {
        generated_table_routes.route(
            open_api_contract.path().as_ref(),
            axum::routing::on(
                axum::routing::MethodFilter::from(
                    frontend_contract::domain_types::axum_method_filter(open_api_contract.method()),
                ),
                async || {
                    axum::Json(utoipa::openapi::OpenApi::from(
                        server_admin::domain_types::generated_tables::generated_open_api(),
                    ))
                },
            ),
        )
    } else {
        generated_table_routes
    }
    .method_not_allowed_fallback(async || {
        frontend_contract::domain_types::ApiProblemError::MethodNotAllowed
    });
    let metrics_contract = server_admin_contract::domain_types::AdminRoute::Metrics.contract();
    let secured_admin_routes = documented_admin_routes
        .route(
            metrics_contract.path().as_ref(),
            axum::routing::on(
                axum::routing::MethodFilter::from(
                    frontend_contract::domain_types::axum_method_filter(metrics_contract.method()),
                ),
                async move || {
                    server_runtime_http::domain_types::MetricsResponseBody::try_from(
                        metrics_exporter_prometheus::PrometheusHandle::from(metrics_handle)
                            .render(),
                    )
                    .map(|body| {
                        axum::response::IntoResponse::into_response((
                            axum::http::StatusCode::OK,
                            body.into_inner(),
                        ))
                    })
                    .map_err(crate::domain_types::AdminMetricsError::Render)
                },
            ),
        )
        .route_layer(server_admin::domain_types::AdminGeneratedAuthLayer::from(
            generated_admin_auth_state,
        ));
    crate::domain_types::AxumApiRoutes::from(
        axum::Router::new()
            .nest(
                server_admin_contract::domain_types::AdminFrontendPath::Root.get(),
                axum::Router::from(server_admin::domain_types::auth::routes(admin_auth_state)),
            )
            .nest(
                server_admin_contract::domain_types::AdminFrontendPath::Root.get(),
                secured_admin_routes,
            ),
    )
}
