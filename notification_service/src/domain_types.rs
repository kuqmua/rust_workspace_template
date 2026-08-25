#![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner
#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each
#![allow(clippy::field_scoped_visibility_modifiers)] // sibling application and adapter modules consume these private binary domain models

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(crate) struct NotificationState {
    pub(crate) metrics: MetricsExporterPrometheusHandle,
    pub(crate) pool: app_state::domain_types::SqlxPgPool,
    pub(crate) project_git_info: git_info::domain_types::ProjectGitInfo<'static>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationState(NotificationState);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationJson(
    notification_service_contract::domain_types::CreateNotificationReq,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationResponse(axum::response::Response);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationRouter(axum::Router);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct HttpNotificationStatusCode(http::StatusCode);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum CreateNotificationError {
    #[error("notification persistence failed: {0}")]
    Persistence(
        #[source] server_runtime_http::domain_types::ObservedError<SqlxNotificationDatabaseError>,
    ),
    #[error("notification request validation failed")]
    Validation,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum MetricsError {
    #[error("notification metrics response rendering failed: {0}")]
    Render(
        #[source]
        server_runtime_http::domain_types::ObservedError<
            server_runtime_http::domain_types::MetricsResponseBodyError,
        >,
    ),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct MetricsExporterPrometheusHandle(metrics_exporter_prometheus::PrometheusHandle);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct NotificationBodyMaximumBytes(usize);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct NotificationExitCode(std::process::ExitCode);

impl AxumNotificationState {
    pub(crate) const fn get(&self) -> &NotificationState {
        &self.0
    }
}
impl AxumNotificationJson {
    pub(crate) fn into_inner(
        self,
    ) -> notification_service_contract::domain_types::CreateNotificationReq {
        self.0
    }
}
impl AxumNotificationRouter {
    pub(crate) fn into_inner(self) -> axum::Router {
        self.0
    }
}
impl MetricsExporterPrometheusHandle {
    pub(crate) fn render(
        &self,
    ) -> Result<
        server_runtime_http::domain_types::MetricsResponseBody,
        server_runtime_http::domain_types::MetricsResponseBodyError,
    > {
        server_runtime_http::domain_types::MetricsResponseBody::try_from(self.0.render())
    }
}
impl NotificationBodyMaximumBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}

impl axum::response::IntoResponse for AxumNotificationResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}
impl axum::response::IntoResponse for HttpNotificationStatusCode {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}
impl axum::response::IntoResponse for CreateNotificationError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::Persistence(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::Validation => http::StatusCode::UNPROCESSABLE_ENTITY,
        };
        let error_type = server_runtime_http::domain_types::HttpErrorType::from(
            constants_str::NOTIFICATION_API_ERROR_TYPE,
        );
        let optional_diagnostic = match &self {
            Self::Persistence(error) => Some(
                server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                    error_type, error,
                ),
            ),
            Self::Validation => None,
        };
        let telemetry = server_runtime_http::domain_types::HttpErrorTelemetry::new(
            error_type,
            server_runtime_http::domain_types::HttpErrorCode::from(
                NotificationErrorCode::Validation.get(),
            ),
        );
        let problem_status =
            frontend_contract::domain_types::ApiProblemStatus::try_from(status.as_u16())
                .unwrap_or_else(|_error| {
                    frontend_contract::domain_types::ApiProblemStatus::from(
                        frontend_contract::domain_types::KnownHttpStatus::InternalServerError,
                    )
                });
        let mut response = axum::response::IntoResponse::into_response(
            frontend_contract::domain_types::ApiProblemError::from_status(problem_status),
        );
        if let Some(diagnostic) = optional_diagnostic {
            let _previous = response.extensions_mut().insert(diagnostic);
        } else {
            let _previous = response.extensions_mut().insert(telemetry);
        }
        response
    }
}
impl axum::response::IntoResponse for MetricsError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Render(error) => {
                let error_type = server_runtime_http::domain_types::HttpErrorType::from(
                    constants_str::NOTIFICATION_API_ERROR_TYPE,
                );
                let mut response = axum::response::IntoResponse::into_response(
                    frontend_contract::domain_types::ApiProblemError::Internal(
                        frontend_contract::domain_types::ApiProblemStatus::from(
                            frontend_contract::domain_types::KnownHttpStatus::InternalServerError,
                        ),
                    ),
                );
                let _previous = response.extensions_mut().insert(
                    server_runtime_http::domain_types::HttpErrorDiagnostic::from_observed(
                        error_type, &error,
                    ),
                );
                response
            }
        }
    }
}
impl std::process::Termination for NotificationExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0
    }
}
impl axum::extract::FromRequestParts<NotificationState> for AxumNotificationState {
    type Rejection = HttpNotificationStatusCode;
    fn from_request_parts(
        _parts: &mut http::request::Parts,
        state: &NotificationState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(state.clone())))
    }
}
impl app_state::domain_types::GetSqlxPgPool for NotificationState {
    fn get_sqlx_pg_pool(&self) -> app_state::domain_types::SqlxPgPoolRef<'_> {
        app_state::domain_types::SqlxPgPoolRef::from(self.pool.as_ref())
    }
}
impl AsRef<str> for NotificationState {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
impl common_routes::domain_types::CommonRoutesParameters for NotificationState {}
impl axum::extract::FromRequest<NotificationState> for AxumNotificationJson {
    type Rejection = CreateNotificationError;
    async fn from_request(
        req: axum::extract::Request,
        state: &NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::domain_types::CreateNotificationReq> as axum::extract::FromRequest<NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| CreateNotificationError::Validation)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum NotificationServiceError {
    #[error("notification service configuration failed: {0}")]
    Config(NotificationConfigError),
    #[error("notification database connection failed: {0}")]
    Database(SqlxNotificationDatabaseError),
    #[error("notification metrics recorder initialization failed: {0}")]
    Metrics(MetricsExporterPrometheusNotificationBuildError),
    #[error("notification observability initialization failed: {0}")]
    ObservabilityInit(NotificationObservabilityInitError),
    #[error("notification observability shutdown failed: {0}")]
    ObservabilityShutdown(NotificationObservabilityShutdownError),
    #[error("notification database migration failed: {0}")]
    Migration(SqlxNotificationMigrationError),
    #[error("notification service failed: {0}")]
    Serve(NotificationServeError),
    #[error("notification service socket bind failed: {0}")]
    Socket(NotificationIoError),
    #[error("notification service timeout configuration is invalid")]
    Timeout,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationConfigError(
    notification_service_config::domain_types::ConfigTryFromEnvError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct SqlxNotificationDatabaseError(sqlx::Error);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct SqlxNotificationMigrationError(sqlx::migrate::MigrateError);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationIoError(std::io::Error);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationServeError(
    server_runtime_http::domain_types::ServeWithGracefulShutdownError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct MetricsExporterPrometheusNotificationBuildError(
    metrics_exporter_prometheus::BuildError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationObservabilityInitError(
    server_runtime_http::domain_types::ObservabilityInitError,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationObservabilityShutdownError(
    server_runtime_http::domain_types::OpentelemetrySdkObservabilityShutdownError,
);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NotificationErrorCode {
    MetricsRender,
    Persistence,
    Validation,
}
impl NotificationErrorCode {
    pub(crate) const fn get(self) -> &'static str {
        match self {
            Self::MetricsRender => constants_str::NOTIFICATION_OBSERVED_ERROR_METRICS_RENDER,
            Self::Persistence => constants_str::NOTIFICATION_OBSERVED_ERROR_PERSISTENCE,
            Self::Validation => constants_str::NOTIFICATION_OBSERVED_ERROR_VALIDATION,
        }
    }
}
#[cfg(test)]
mod tests {
    fn state(pool: sqlx::PgPool) -> super::NotificationState {
        super::NotificationState {
            metrics: super::MetricsExporterPrometheusHandle::from(
                metrics_exporter_prometheus::PrometheusBuilder::new()
                    .build_recorder()
                    .handle(),
            ),
            pool: app_state::domain_types::SqlxPgPool::from(pool),
            project_git_info: git_info::domain_types::project_git_info(),
        }
    }

    #[tokio::test]
    async fn boundary_adapters_preserve_status_state_and_exit_code() {
        let status_response = axum::response::IntoResponse::into_response(
            super::HttpNotificationStatusCode::from(http::StatusCode::IM_A_TEAPOT),
        );
        assert_eq!(status_response.status(), http::StatusCode::IM_A_TEAPOT);
        assert_eq!(
            std::process::Termination::report(super::NotificationExitCode::from(
                std::process::ExitCode::SUCCESS,
            )),
            std::process::ExitCode::SUCCESS
        );

        let state = state(
            sqlx::postgres::PgPoolOptions::new()
                .connect_lazy(
                    constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
                )
                .expect("75b0f8e4 boundary_adapters_preserve_status_state_and_exit_code invariant must hold"),
        );
        let request = http::Request::new(());
        let (mut parts, _body) = request.into_parts();
        let extracted = <super::AxumNotificationState as axum::extract::FromRequestParts<
            super::NotificationState,
        >>::from_request_parts(&mut parts, &state)
        .await
        .expect(
            "c12d49a7 boundary_adapters_preserve_status_state_and_exit_code invariant must hold",
        );
        assert_eq!(
            AsRef::<str>::as_ref(extracted.get()),
            AsRef::<str>::as_ref(&state)
        );
        let _pool = app_state::domain_types::GetSqlxPgPool::get_sqlx_pg_pool(&state);
    }

    #[tokio::test]
    #[cfg_attr(
        miri,
        ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
    )]
    async fn default_service_routes_return_success_statuses() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(
                constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
            )
            .expect("52a25be1 default_service_routes_return_success_statuses invariant must hold");
        let router = crate::adapters::routes::router(
            state(pool),
            super::NotificationBodyMaximumBytes::from(
                notification_service_contract::domain_types::NOTIFICATION_API_BODY_MAX_BYTES,
            ),
        )
        .into_inner();
        let liveness_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .uri(
                    common_routes::domain_types::CommonRoute::HealthLive
                        .path()
                        .as_ref(),
                )
                .body(axum::body::Body::empty())
                .expect(
                    "ec467ec0 default_service_routes_return_success_statuses invariant must hold",
                ),
        )
        .await
        .expect("717fb1f4 default_service_routes_return_success_statuses invariant must hold");
        assert_eq!(liveness_response.status(), http::StatusCode::OK);
        let open_api_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .uri(constants_str::OPENAPI_JSON)
                .body(axum::body::Body::empty())
                .expect(
                    "789db8f3 default_service_routes_return_success_statuses invariant must hold",
                ),
        )
        .await
        .expect("2d37fbd2 default_service_routes_return_success_statuses invariant must hold");
        assert_eq!(open_api_response.status(), http::StatusCode::OK);
        let metrics_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .uri(
                    frontend_contract::domain_types::HandlerContract::path(
                        notification_service_contract::domain_types::NotificationOperationalRoute::Metrics,
                    )
                    .get(),
                )
                .body(axum::body::Body::empty())
                .expect(
                    "f9a73c10 default_service_routes_return_success_statuses invariant must hold",
                ),
        )
        .await
        .expect("81c4e6a2 default_service_routes_return_success_statuses invariant must hold");
        assert_eq!(metrics_response.status(), http::StatusCode::OK);

        let create_metadata = <notification_service_contract::domain_types::CreateNotificationRoute as frontend_contract::domain_types::TypedRoute>::metadata();
        let invalid_request = tower::ServiceExt::oneshot(
            router,
            http::Request::builder()
                .method(create_metadata.method().as_ref())
                .uri(create_metadata.path().as_ref())
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from("{"))
                .expect(
                    "4ac710e9 default_service_routes_return_success_statuses invariant must hold",
                ),
        )
        .await
        .expect("d8326f1b default_service_routes_return_success_statuses invariant must hold");
        assert_eq!(
            invalid_request.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
        assert!(
            invalid_request
                .extensions()
                .get::<server_runtime_http::domain_types::HttpErrorTelemetry>()
                .is_some()
        );
    }

    #[test]
    fn open_api_has_no_unresolved_schema_references() {
        frontend_contract_validation::domain_types::openapi_validation::validate_openapi_schema_references(&{
            let mut document = crate::adapters::routes::open_api_document();
            document.merge(utoipa::openapi::OpenApi::from(
                common_routes::domain_types::CommonRoutesOpenApi::open_api(),
            ));
            document
        })
        .expect("3e63ebd8 open_api_has_no_unresolved_schema_references invariant must hold");
    }

    #[test]
    fn open_api_operation_and_statuses_come_from_the_typed_route() {
        let metadata = <notification_service_contract::domain_types::CreateNotificationRoute as frontend_contract::domain_types::TypedRoute>::metadata();
        let document = serde_json::to_value(crate::adapters::routes::open_api_document()).expect("3d8a056d open_api_operation_and_statuses_come_from_the_typed_route invariant must hold");
        let operation = document
            .get(constants_str::PATHS)
            .and_then(|paths| paths.get(metadata.path().as_ref()))
            .and_then(|path| path.get(metadata.method().as_ref().to_ascii_lowercase()))
            .expect("fb8bb06a open_api_operation_and_statuses_come_from_the_typed_route invariant must hold");
        assert_eq!(
            operation
                .get(constants_str::OPERATION_ID_JSON)
                .and_then(serde_json::Value::as_str),
            Some(metadata.openapi_operation_id().as_ref()),
        );
        let observed_statuses = operation
            .get(constants_str::RESPONSES)
            .and_then(serde_json::Value::as_object)
            .expect("251c95e8 open_api_operation_and_statuses_come_from_the_typed_route invariant must hold")
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        let expected_statuses =
            std::iter::once(metadata.success_status().transport_status().to_string())
                .chain(
                    metadata
                        .error_statuses()
                        .iter()
                        .map(|status| status.transport_status().to_string()),
                )
                .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(observed_statuses, expected_statuses);
    }

    #[test]
    fn api_problem_preserves_server_diagnostic_but_keeps_validation_expected() {
        let server_response = axum::response::IntoResponse::into_response(
            super::CreateNotificationError::Persistence(
                server_runtime_http::domain_types::ObservedError::capture(
                    super::SqlxNotificationDatabaseError::from(sqlx::Error::RowNotFound),
                    server_runtime_http::domain_types::ObservedErrorCode::from(
                        super::NotificationErrorCode::Persistence.get(),
                    ),
                ),
            ),
        );
        assert_eq!(
            server_response.status(),
            http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(
            server_response
                .extensions()
                .get::<server_runtime_http::domain_types::HttpErrorDiagnostic>()
                .is_some()
        );
        let validation_response =
            axum::response::IntoResponse::into_response(super::CreateNotificationError::Validation);
        assert_eq!(
            validation_response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
        assert!(
            validation_response
                .extensions()
                .get::<server_runtime_http::domain_types::HttpErrorDiagnostic>()
                .is_none()
        );
        assert!(
            validation_response
                .extensions()
                .get::<server_runtime_http::domain_types::HttpErrorTelemetry>()
                .is_some()
        );

        let metrics_response = axum::response::IntoResponse::into_response(
            super::MetricsError::Render(server_runtime_http::domain_types::ObservedError::capture(
                server_runtime_http::domain_types::MetricsResponseBodyError,
                server_runtime_http::domain_types::ObservedErrorCode::from(
                    super::NotificationErrorCode::MetricsRender.get(),
                ),
            )),
        );
        assert_eq!(
            metrics_response.status(),
            http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(
            metrics_response
                .extensions()
                .get::<server_runtime_http::domain_types::HttpErrorDiagnostic>()
                .is_some()
        );
    }

    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn create_notification_persists_through_http_route() {
        let database_url = config_lib::domain_types::parse_required_env_var(
            config_lib::domain_types::EnvVarNameRef::from(constants_str::ENV_NAMES_DATABASE_URL),
            |error, name| format!("{error} {name}"),
            <config_lib::domain_types::DatabaseUrl as config_lib::domain_types::TryFromStdEnvVarOk>::try_from_std_env_var_ok,
            |error| error.to_string(),
        )
        .expect("b3aacb7e create_notification_persists_through_http_route invariant must hold");
        let exposed_database_url = secrecy::ExposeSecret::expose_secret(&database_url.0).as_str();
        let setup_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect(exposed_database_url)
            .await
            .expect("ceff90ad create_notification_persists_through_http_route invariant must hold");
        let _schema_result = sqlx::query(sqlx::AssertSqlSafe(
            constants_str::NOTIFICATION_SERVICE_CREATE_TEST_SCHEMA_SQL.concat(),
        ))
        .execute(&setup_pool)
        .await
        .expect("59114ac3 create_notification_persists_through_http_route invariant must hold");
        setup_pool.close().await;
        let connect_options =
            <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(exposed_database_url)
                .expect(
                    "2145d54a create_notification_persists_through_http_route invariant must hold",
                )
                .options([(
                    constants_str::SEARCH_PATH,
                    constants_str::NOTIFICATION_SERVICE_TEST_SCHEMA,
                )]);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2u32)
            .connect_with(connect_options)
            .await
            .expect("5344bc9e create_notification_persists_through_http_route invariant must hold");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("128c46f1 create_notification_persists_through_http_route invariant must hold");
        let message = notification_service_contract::domain_types::NotificationMessage::try_from(
            constants_str::INTEGRATION_NOTIFICATION_MESSAGE.to_owned(),
        )
        .expect("f9605432 create_notification_persists_through_http_route invariant must hold");
        let body = serde_json::to_vec(
            &notification_service_contract::domain_types::CreateNotificationReq::new(message),
        )
        .expect("3daa1ab0 create_notification_persists_through_http_route invariant must hold");
        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri(
                frontend_contract::domain_types::typed_route_path::<
                    notification_service_contract::domain_types::CreateNotificationRoute,
                >()
                .as_ref(),
            )
            .header(
                http::header::CONTENT_TYPE,
                constants_str::HTTP_APPLICATION_JSON,
            )
            .body(axum::body::Body::from(body))
            .expect("f8d2ab0b create_notification_persists_through_http_route invariant must hold");
        let response = tower::ServiceExt::oneshot(
            crate::adapters::routes::router(
                state(pool),
                super::NotificationBodyMaximumBytes::from(
                    notification_service_contract::domain_types::NOTIFICATION_API_BODY_MAX_BYTES,
                ),
            )
            .into_inner(),
            request,
        )
        .await
        .expect("c46bf92a create_notification_persists_through_http_route invariant must hold");
        assert_eq!(response.status(), http::StatusCode::CREATED);
        let response_body = axum::body::to_bytes(response.into_body(), 16_384usize)
            .await
            .expect("0aace9dd create_notification_persists_through_http_route invariant must hold");
        let created: notification_service_contract::domain_types::CreateNotificationRes =
            serde_json::from_slice(response_body.as_ref()).expect(
                "e5352eef create_notification_persists_through_http_route invariant must hold",
            );
        assert_ne!(
            created.id(),
            notification_service_contract::domain_types::UuidNotificationId::from(uuid::Uuid::nil())
        );
    }
}
