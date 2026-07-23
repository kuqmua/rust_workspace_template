#![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner
#![allow(clippy::arbitrary_source_item_ordering)] // OpenAPI document stays next to its generated schema and operation marker
#![allow(clippy::needless_for_each)] // utoipa OpenApi derive expands to an internal for_each

#[derive(Clone, Debug)]
struct NotificationState {
    metrics: MetricsExporterPrometheusHandle,
    pool: app_state::SqlxPgPool,
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

#[derive(Debug, newtype::FromInner)]
struct HttpNotificationApiProblem(http::StatusCode);

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
impl axum::response::IntoResponse for HttpNotificationApiProblem {
    fn into_response(self) -> axum::response::Response {
        let status = self.0;
        axum::response::IntoResponse::into_response((
            status,
            axum::Json(frontend_contract::ApiProblem::from_status(
                frontend_contract::ApiProblemStatus::try_from(status.as_u16()).unwrap_or_else(
                    |_error| {
                        frontend_contract::ApiProblemStatus::from(
                            frontend_contract::KnownHttpStatus::InternalServerError,
                        )
                    },
                ),
            )),
        ))
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
impl axum::extract::FromRequest<NotificationState> for AxumNotificationJson {
    type Rejection = HttpNotificationApiProblem;
    async fn from_request(
        req: axum::extract::Request,
        state: &NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::CreateNotificationReq> as axum::extract::FromRequest<NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| HttpNotificationApiProblem::from(http::StatusCode::UNPROCESSABLE_ENTITY))
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

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct SqlxNotificationDatabaseError(sqlx::Error);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct SqlxNotificationMigrationError(sqlx::migrate::MigrateError);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct StdNotificationIoError(std::io::Error);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct NotificationServeError(server_runtime::ServeWithGracefulShutdownError);

#[derive(Debug, newtype::FromInner, newtype::Display)]
struct MetricsExporterPrometheusNotificationBuildError(metrics_exporter_prometheus::BuildError);

async fn create_notification(
    state: AxumNotificationState,
    request: AxumNotificationJson,
) -> Result<AxumNotificationResponse, HttpNotificationApiProblem> {
    let id = uuid::Uuid::new_v4();
    let message = request.0.into_message();
    let _created = sqlx::query(str_constants::NOTIFICATION_INSERT_SQL)
        .bind(id)
        .bind(message.as_ref())
        .execute(state.0.pool.as_ref())
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "notification persistence failed");
            HttpNotificationApiProblem::from(http::StatusCode::INTERNAL_SERVER_ERROR)
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
) -> Result<server_runtime::MetricsResponseBody, HttpNotificationApiProblem> {
    server_runtime::MetricsResponseBody::try_from(state.0.metrics.0.render())
        .map_err(|_error| HttpNotificationApiProblem::from(http::StatusCode::INTERNAL_SERVER_ERROR))
}

async fn readiness(state: AxumNotificationState) -> HttpNotificationStatusCode {
    match sqlx::query(str_constants::COMMON_ROUTES_HEALTH_CHECK_SQL)
        .execute(state.0.pool.as_ref())
        .await
    {
        Ok(_result) => HttpNotificationStatusCode::from(http::StatusCode::OK),
        Err(error) => {
            tracing::error!(error = %error, "notification readiness probe failed");
            HttpNotificationStatusCode::from(http::StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

fn router(
    state: NotificationState,
    body_maximum_bytes: NotificationBodyMaximumBytes,
) -> AxumNotificationRouter {
    AxumNotificationRouter::from(
        axum::Router::new()
            .route(
                str_constants::COMMON_ROUTES_HEALTH_LIVE,
                axum::routing::get(async || http::StatusCode::OK),
            )
            .route(
                str_constants::COMMON_ROUTES_HEALTH_READY,
                axum::routing::get(readiness),
            )
            .route(
                frontend_contract::typed_route_path::<
                    notification_service_contract::CreateNotificationRoute,
                >()
                .as_ref(),
                axum::routing::post(create_notification),
            )
            .route(str_constants::METRICS, axum::routing::get(metrics))
            .route(
                str_constants::OPENAPI_JSON_PATH,
                axum::routing::get(async || {
                    axum::Json(<NotificationOpenApi as utoipa::OpenApi>::openapi())
                }),
            )
            .layer(axum::extract::DefaultBodyLimit::max(body_maximum_bytes.0))
            .with_state(state),
    )
}

#[allow(
    clippy::arbitrary_source_item_ordering,
    clippy::needless_for_each,
    reason = "utoipa derives operation iteration and the document remains next to its schema declaration"
)]
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(create_notification_openapi),
    components(schemas(
        notification_service_contract::CreateNotificationReq,
        notification_service_contract::CreateNotificationRes,
        notification_service_contract::NotificationMessage,
        notification_service_contract::UuidNotificationId,
        frontend_contract::ApiProblem
    ))
)]
struct NotificationOpenApi;

#[utoipa::path(
    post,
    path = "/notifications",
    request_body = notification_service_contract::CreateNotificationReq,
    responses(
        (status = 201, description = "Notification persisted", body = notification_service_contract::CreateNotificationRes),
        (status = 422, description = "Invalid request", body = frontend_contract::ApiProblem),
        (status = 500, description = "Persistence failure", body = frontend_contract::ApiProblem)
    )
)]
#[allow(
    dead_code,
    reason = "utoipa references this operation through generated metadata"
)]
const fn create_notification_openapi() {}

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
    let timeout = server_runtime::StdRequestTimeout::try_from(std::time::Duration::from_secs(
        config.request_timeout_seconds().get(),
    ))
    .map_err(|_error| NotificationServiceError::Timeout)?;
    let service_router = server_runtime::RequestIdLayer.apply(
        server_runtime::SecurityHeadersLayer::from(server_runtime::ForwardedProtoTrust::Ignore)
            .apply(
                server_runtime::RequestTimeoutLayer::from(timeout).apply(
                    server_runtime::AxumRouter::from(
                        router(
                            NotificationState {
                                metrics,
                                pool: app_state::SqlxPgPool::from(pool),
                            },
                            NotificationBodyMaximumBytes::from(
                                (**config.maximum_size_of_http_body_in_bytes()).min(
                                    notification_service_contract::NOTIFICATION_API_BODY_MAX_BYTES,
                                ),
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
    if let Err(error) = server_runtime::initialize_service_tracing(tracing_format) {
        eprintln!("notification service tracing initialization failed: {error}");
        return StdNotificationExitCode::from(std::process::ExitCode::FAILURE);
    }
    match run(config).await {
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
        }
    }

    #[tokio::test]
    async fn router_contains_service_owned_routes() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(
                str_constants::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
            )
            .expect("52a25be1");
        let _router = super::router(
            state(pool),
            super::NotificationBodyMaximumBytes::from(
                notification_service_contract::NOTIFICATION_API_BODY_MAX_BYTES,
            ),
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
