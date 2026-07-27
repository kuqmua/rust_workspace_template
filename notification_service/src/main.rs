#![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner
#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each

#[derive(Clone, Debug)]
struct NotificationState {
    metrics: MetricsExporterPrometheusHandle,
    pool: app_state::SqlxPgPool,
    project_git_info: git_info::ProjectGitInfo<'static>,
}
#[derive(Clone, Debug, newtype::FromInner)]
struct AxumNotificationState(NotificationState);

#[derive(Debug, newtype::FromInner)]
struct AxumNotificationJson(notification_service_contract::CreateNotificationReq);

#[derive(Debug, newtype::FromInner)]
struct AxumNotificationResponse(axum::response::Response);

#[derive(Debug, newtype::FromInner)]
struct AxumNotificationRouter(axum::Router);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct HttpNotificationStatusCode(http::StatusCode);

#[derive(Debug, thiserror::Error)]
enum CreateNotificationError {
    #[error("notification persistence failed: {0}")]
    Persistence(#[source] server_runtime::ObservedError<SqlxNotificationDatabaseError>),
    #[error("notification request validation failed")]
    Validation,
}
#[derive(Debug, thiserror::Error)]
enum MetricsError {
    #[error("notification metrics response rendering failed: {0}")]
    Render(#[source] server_runtime::ObservedError<server_runtime::MetricsResponseBodyError>),
}
#[derive(Debug, thiserror::Error)]
enum OpenApiError {}

#[derive(Clone, Debug, newtype::FromInner)]
struct MetricsExporterPrometheusHandle(metrics_exporter_prometheus::PrometheusHandle);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct NotificationBodyMaximumBytes(usize);

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct StdNotificationExitCode(std::process::ExitCode);

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
        let error_type =
            server_runtime::HttpErrorType::from(str_constants::NOTIFICATION_API_ERROR_TYPE);
        let optional_diagnostic = match &self {
            Self::Persistence(error) => Some(server_runtime::HttpErrorDiagnostic::from_observed(
                error_type, error,
            )),
            Self::Validation => None,
        };
        let telemetry = server_runtime::HttpErrorTelemetry::new(
            error_type,
            server_runtime::HttpErrorCode::from(NotificationErrorCode::Validation.get()),
        );
        let problem_status = frontend_contract::ApiProblemStatus::try_from(status.as_u16())
            .unwrap_or_else(|_error| {
                frontend_contract::ApiProblemStatus::from(
                    frontend_contract::KnownHttpStatus::InternalServerError,
                )
            });
        let mut response = axum::response::IntoResponse::into_response(
            frontend_contract::ApiProblemError::from_status(problem_status),
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
                let error_type =
                    server_runtime::HttpErrorType::from(str_constants::NOTIFICATION_API_ERROR_TYPE);
                let mut response = axum::response::IntoResponse::into_response(
                    frontend_contract::ApiProblemError::Internal(
                        frontend_contract::ApiProblemStatus::from(
                            frontend_contract::KnownHttpStatus::InternalServerError,
                        ),
                    ),
                );
                let _previous = response.extensions_mut().insert(
                    server_runtime::HttpErrorDiagnostic::from_observed(error_type, &error),
                );
                response
            }
        }
    }
}
impl axum::response::IntoResponse for OpenApiError {
    fn into_response(self) -> axum::response::Response {
        match self {}
    }
}
impl std::process::Termination for StdNotificationExitCode {
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
impl app_state::GetSqlxPgPool for NotificationState {
    fn get_sqlx_pg_pool(&self) -> app_state::SqlxPgPoolRef<'_> {
        app_state::SqlxPgPoolRef::from(self.pool.as_ref())
    }
}
impl AsRef<str> for NotificationState {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
impl common_routes::CommonRoutesParameters for NotificationState {}
impl axum::extract::FromRequest<NotificationState> for AxumNotificationJson {
    type Rejection = CreateNotificationError;
    async fn from_request(
        req: axum::extract::Request,
        state: &NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::CreateNotificationReq> as axum::extract::FromRequest<NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| CreateNotificationError::Validation)
    }
}

#[derive(Debug, thiserror::Error)]
enum NotificationServiceError {
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
    Socket(StdNotificationIoError),
    #[error("notification service timeout configuration is invalid")]
    Timeout,
}
#[derive(Debug, newtype::FromInner, newtype::Display)]
struct NotificationConfigError(notification_service_config::ConfigTryFromEnvError);

#[derive(Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
struct SqlxNotificationDatabaseError(sqlx::Error);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct SqlxNotificationMigrationError(sqlx::migrate::MigrateError);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct StdNotificationIoError(std::io::Error);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct NotificationServeError(server_runtime::ServeWithGracefulShutdownError);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct MetricsExporterPrometheusNotificationBuildError(metrics_exporter_prometheus::BuildError);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct NotificationObservabilityInitError(server_runtime::ObservabilityInitError);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct NotificationObservabilityShutdownError(
    server_runtime::OpentelemetrySdkObservabilityShutdownError,
);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NotificationErrorCode {
    MetricsRender,
    Persistence,
    Validation,
}
impl NotificationErrorCode {
    const fn get(self) -> &'static str {
        match self {
            Self::MetricsRender => str_constants::NOTIFICATION_OBSERVED_ERROR_METRICS_RENDER,
            Self::Persistence => str_constants::NOTIFICATION_OBSERVED_ERROR_PERSISTENCE,
            Self::Validation => str_constants::NOTIFICATION_OBSERVED_ERROR_VALIDATION,
        }
    }
}
#[frontend_contract::route_openapi()]
async fn create_notification(
    state: AxumNotificationState,
    request: AxumNotificationJson,
) -> Result<AxumNotificationResponse, CreateNotificationError> {
    let id = uuid::Uuid::new_v4();
    let message = request.0.into_message();
    let insert_sql = "INSERT INTO notifications (id, message) VALUES ($1, $2)";
    let _created = sqlx::query(insert_sql)
        .bind(id)
        .bind(message.as_ref())
        .execute(state.0.pool.as_ref())
        .await
        .map_err(|error| {
            CreateNotificationError::Persistence(server_runtime::ObservedError::capture(
                SqlxNotificationDatabaseError::from(error),
                server_runtime::ObservedErrorCode::from(NotificationErrorCode::Persistence.get()),
            ))
        })?;
    Ok(AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(notification_service_contract::CreateNotificationRes::new(
                notification_service_contract::UuidNotificationId::from(id),
            )),
        )),
    ))
}

async fn metrics(
    state: AxumNotificationState,
) -> Result<server_runtime::MetricsResponseBody, MetricsError> {
    server_runtime::MetricsResponseBody::try_from(state.0.metrics.0.render()).map_err(|error| {
        MetricsError::Render(server_runtime::ObservedError::capture(
            error,
            server_runtime::ObservedErrorCode::from(NotificationErrorCode::MetricsRender.get()),
        ))
    })
}

async fn open_api() -> Result<AxumNotificationResponse, OpenApiError> {
    let mut document = NotificationApiRouteRegistry::open_api();
    document.merge(utoipa::openapi::OpenApi::from(
        common_routes::CommonRoutesOpenApi::open_api(),
    ));
    Ok(AxumNotificationResponse::from(
        axum::response::IntoResponse::into_response(axum::Json(document)),
    ))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NotificationOperationalRoute {
    Metrics,
    OpenApi,
}
impl frontend_contract::HandlerContract for NotificationOperationalRoute {
    fn method(self) -> frontend_contract::RouteMethod {
        frontend_contract::RouteMethod::Get
    }
    fn path(self) -> frontend_contract::HandlerPath {
        frontend_contract::HandlerPath::from(match self {
            Self::Metrics => str_constants::METRICS,
            Self::OpenApi => str_constants::OPENAPI_JSON,
        })
    }
}

#[frontend_contract::route_registry(
    state = NotificationState,
    family = notification_service_contract::NotificationRouteFamily;
    ("", "");
    schemas(
        notification_service_contract::NotificationMessage,
        notification_service_contract::UuidNotificationId
    );
    (
        notification_service_contract::CreateNotificationRoute,
        create_notification
    ),
)]
#[openapi()]
struct NotificationApiRouteRegistry;

#[frontend_contract::handler_registry(
    state = NotificationState;
    (
        NotificationOperationalRoute::Metrics,
        metrics
    ),
    (
        NotificationOperationalRoute::OpenApi,
        open_api
    ),
)]
struct NotificationRouteRegistry;

fn router(
    state: NotificationState,
    body_maximum_bytes: NotificationBodyMaximumBytes,
) -> AxumNotificationRouter {
    let common_routes = axum::Router::from(common_routes::common_routes(
        common_routes::StdArcCommonRoutesAppState::from(std::sync::Arc::new(state.clone())),
    ));
    AxumNotificationRouter::from(
        NotificationRouteRegistry::router()
            .merge(NotificationApiRouteRegistry::router())
            .layer(axum::extract::DefaultBodyLimit::max(body_maximum_bytes.0))
            .with_state(state)
            .merge(common_routes),
    )
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(error = %error, "notification shutdown signal failed");
    }
}

async fn run(config: notification_service_config::Config) -> Result<(), NotificationServiceError> {
    let metrics = metrics_exporter_prometheus::PrometheusBuilder::new()
        .install_recorder()
        .map(MetricsExporterPrometheusHandle)
        .map_err(|error| {
            NotificationServiceError::Metrics(
                MetricsExporterPrometheusNotificationBuildError::from(error),
            )
        })?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(**config.pg_pool_max_connections())
        .connect(secrecy::ExposeSecret::expose_secret(
            &config.notification_database_url().0,
        ))
        .await
        .map_err(|error| {
            NotificationServiceError::Database(SqlxNotificationDatabaseError::from(error))
        })?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|error| {
            NotificationServiceError::Migration(SqlxNotificationMigrationError::from(error))
        })?;
    let listener = tokio::net::TcpListener::bind(config.notification_service_socket_address().0)
        .await
        .map_err(|error| NotificationServiceError::Socket(StdNotificationIoError::from(error)))?;
    let actual_service_socket_address = listener
        .local_addr()
        .map_err(|error| NotificationServiceError::Socket(StdNotificationIoError::from(error)))?;
    let timeout = server_runtime::StdRequestTimeout::try_from(std::time::Duration::from_secs(
        config.request_timeout_seconds().get(),
    ))
    .map_err(|_error| NotificationServiceError::Timeout)?;
    let service_router = server_runtime::RequestIdLayer::with_span_config(
        server_runtime::HttpRequestSpanConfig::new(
            server_runtime::ServiceName::from(env!("CARGO_PKG_NAME")),
            server_runtime::StdSocketAddr::from(actual_service_socket_address),
            server_runtime::TrustedProxyRanges::default(),
        ),
    )
    .apply(
        server_runtime::SecurityHeadersLayer::from(server_runtime::ForwardedProtoTrust::Ignore)
            .apply(
                server_runtime::RequestTimeoutLayer::from(timeout).apply(
                    server_runtime::AxumRouter::from(
                        router(
                            NotificationState {
                                metrics,
                                pool: app_state::SqlxPgPool::from(pool),
                                project_git_info: git_info::project_git_info(),
                            },
                            NotificationBodyMaximumBytes::from(
                                notification_service_contract::NOTIFICATION_API_BODY_MAX_BYTES,
                            ),
                        )
                        .0,
                    ),
                ),
            ),
    );
    server_runtime::serve_with_graceful_shutdown(
        server_runtime::TokioTcpListener::from(listener),
        service_router,
        shutdown_signal(),
        timeout,
    )
    .await
    .map_err(|error| NotificationServiceError::Serve(NotificationServeError::from(error)))
}

#[tokio::main]
async fn main() -> StdNotificationExitCode {
    let config = match notification_service_config::Config::try_from_env() {
        Ok(value) => value,
        Err(error) => {
            eprintln!(
                "{}",
                NotificationServiceError::Config(NotificationConfigError(error))
            );
            return StdNotificationExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let tracing_format = if *config.tracing_format() == config_lib::types::TracingFormat::Json {
        server_runtime::ServiceTracingFormat::Json
    } else {
        server_runtime::ServiceTracingFormat::Text
    };
    let observability = match server_runtime::initialize_service_observability(
        tracing_format,
        server_runtime::ServiceName::from(env!("CARGO_PKG_NAME")),
    ) {
        Ok(value) => value,
        Err(error) => {
            eprintln!(
                "{}",
                NotificationServiceError::ObservabilityInit(
                    NotificationObservabilityInitError::from(error)
                )
            );
            return StdNotificationExitCode::from(std::process::ExitCode::FAILURE);
        }
    };
    let run_result = run(config).await;
    if let Err(error) = run_result.as_ref() {
        tracing::error!(error = %error, "notification service terminated with an error");
    }
    let shutdown_result = observability.shutdown().map_err(|error| {
        NotificationServiceError::ObservabilityShutdown(
            NotificationObservabilityShutdownError::from(error),
        )
    });
    match run_result.and(shutdown_result) {
        Ok(()) => StdNotificationExitCode::from(std::process::ExitCode::SUCCESS),
        Err(error) => {
            eprintln!("{error}");
            StdNotificationExitCode::from(std::process::ExitCode::FAILURE)
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
            pool: app_state::SqlxPgPool::from(pool),
            project_git_info: git_info::project_git_info(),
        }
    }

    #[tokio::test]
    #[cfg_attr(
        miri,
        ignore = "SQLx account discovery calls getpwuid_r, which Miri does not support"
    )]
    async fn router_contains_service_owned_routes() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(
                str_constants::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
            )
            .expect("52a25be1");
        let router = super::router(
            state(pool),
            super::NotificationBodyMaximumBytes::from(
                notification_service_contract::NOTIFICATION_API_BODY_MAX_BYTES,
            ),
        )
        .0;
        let liveness_response = tower::ServiceExt::oneshot(
            router.clone(),
            http::Request::builder()
                .uri(common_routes::CommonRoute::HealthLive.path().as_ref())
                .body(axum::body::Body::empty())
                .expect("ec467ec0"),
        )
        .await
        .expect("717fb1f4");
        assert_eq!(liveness_response.status(), http::StatusCode::OK);
        let open_api_response = tower::ServiceExt::oneshot(
            router,
            http::Request::builder()
                .uri(str_constants::OPENAPI_JSON)
                .body(axum::body::Body::empty())
                .expect("789db8f3"),
        )
        .await
        .expect("2d37fbd2");
        assert_eq!(open_api_response.status(), http::StatusCode::OK);
    }

    #[test]
    fn open_api_has_no_unresolved_schema_references() {
        frontend_contract::validate_openapi_schema_references(&{
            let mut document = super::NotificationApiRouteRegistry::open_api();
            document.merge(utoipa::openapi::OpenApi::from(
                common_routes::CommonRoutesOpenApi::open_api(),
            ));
            document
        })
        .expect("3e63ebd8");
    }

    #[test]
    fn open_api_operation_and_statuses_come_from_the_typed_route() {
        let metadata = <notification_service_contract::CreateNotificationRoute as frontend_contract::TypedRoute>::metadata();
        let document = serde_json::to_value(super::NotificationApiRouteRegistry::open_api())
            .expect("3d8a056d");
        let operation = document
            .get(str_constants::PATHS)
            .and_then(|paths| paths.get(metadata.path().as_ref()))
            .and_then(|path| path.get(metadata.method().as_ref().to_ascii_lowercase()))
            .expect("fb8bb06a");
        assert_eq!(
            operation
                .get(str_constants::OPERATION_ID_JSON)
                .and_then(serde_json::Value::as_str),
            Some(metadata.openapi_operation_id().as_ref()),
        );
        let observed_statuses = operation
            .get(str_constants::RESPONSES)
            .and_then(serde_json::Value::as_object)
            .expect("251c95e8")
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
            super::CreateNotificationError::Persistence(server_runtime::ObservedError::capture(
                super::SqlxNotificationDatabaseError::from(sqlx::Error::RowNotFound),
                server_runtime::ObservedErrorCode::from(
                    super::NotificationErrorCode::Persistence.get(),
                ),
            )),
        );
        assert_eq!(
            server_response.status(),
            http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(
            server_response
                .extensions()
                .get::<server_runtime::HttpErrorDiagnostic>()
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
                .get::<server_runtime::HttpErrorDiagnostic>()
                .is_none()
        );
        assert!(
            validation_response
                .extensions()
                .get::<server_runtime::HttpErrorTelemetry>()
                .is_some()
        );
    }

    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn create_notification_persists_through_http_route() {
        let database_url = config_lib::parse_required_env_var(
            config_lib::EnvVarNameRef::from(str_constants::ENV_NAMES_DATABASE_URL),
            |error, name| format!("{error} {name}"),
            <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok,
            |error| error.to_string(),
        )
        .expect("b3aacb7e");
        let exposed_database_url = secrecy::ExposeSecret::expose_secret(&database_url.0).as_str();
        let setup_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect(exposed_database_url)
            .await
            .expect("ceff90ad");
        let _schema_result = sqlx::query(
            str_constants::NOTIFICATION_SERVICE_CREATE_TEST_SCHEMA_SQL
                .concat()
                .as_str(),
        )
        .execute(&setup_pool)
        .await
        .expect("59114ac3");
        setup_pool.close().await;
        let connect_options =
            <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(exposed_database_url)
                .expect("2145d54a")
                .options([(
                    str_constants::SEARCH_PATH,
                    str_constants::NOTIFICATION_SERVICE_TEST_SCHEMA,
                )]);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2u32)
            .connect_with(connect_options)
            .await
            .expect("5344bc9e");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("128c46f1");
        let message = notification_service_contract::NotificationMessage::try_from(
            str_constants::INTEGRATION_NOTIFICATION_MESSAGE.to_owned(),
        )
        .expect("f9605432");
        let body = serde_json::to_vec(&notification_service_contract::CreateNotificationReq::new(
            message,
        ))
        .expect("3daa1ab0");
        let request = http::Request::builder()
            .method(http::Method::POST)
            .uri(
                frontend_contract::typed_route_path::<
                    notification_service_contract::CreateNotificationRoute,
                >()
                .as_ref(),
            )
            .header(
                http::header::CONTENT_TYPE,
                str_constants::HTTP_APPLICATION_JSON,
            )
            .body(axum::body::Body::from(body))
            .expect("f8d2ab0b");
        let response = tower::ServiceExt::oneshot(
            super::router(
                state(pool),
                super::NotificationBodyMaximumBytes::from(
                    notification_service_contract::NOTIFICATION_API_BODY_MAX_BYTES,
                ),
            )
            .0,
            request,
        )
        .await
        .expect("c46bf92a");
        assert_eq!(response.status(), http::StatusCode::CREATED);
        let response_body = axum::body::to_bytes(response.into_body(), 16_384usize)
            .await
            .expect("0aace9dd");
        let created: notification_service_contract::CreateNotificationRes =
            serde_json::from_slice(response_body.as_ref()).expect("e5352eef");
        assert_ne!(
            created.id(),
            notification_service_contract::UuidNotificationId::from(uuid::Uuid::nil())
        );
    }
}
