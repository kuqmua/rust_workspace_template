#![allow(clippy::single_call_fn)] // binary composition functions intentionally have one startup or route registration owner

#[derive(Clone, Debug)]
struct NotificationState {
    pool: app_state::SqlxPgPool,
}
#[derive(Clone, Debug)]
struct AxumNotificationState(NotificationState);
#[derive(Debug)]
struct AxumNotificationJson(notification_service_contract::CreateNotificationReq);
#[derive(Debug)]
struct AxumNotificationResponse(axum::response::Response);
#[derive(Debug)]
struct AxumNotificationRouter(axum::Router);
#[derive(Clone, Copy, Debug)]
struct HttpNotificationStatusCode(http::StatusCode);
#[derive(Clone, Copy, Debug)]
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
        std::future::ready(Ok(Self(state.clone())))
    }
}
impl axum::extract::FromRequest<NotificationState> for AxumNotificationJson {
    type Rejection = HttpNotificationStatusCode;
    async fn from_request(
        req: axum::extract::Request,
        state: &NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::CreateNotificationReq> as axum::extract::FromRequest<NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self(value))
            .map_err(|_error| HttpNotificationStatusCode(http::StatusCode::UNPROCESSABLE_ENTITY))
    }
}

#[derive(Debug, thiserror::Error)]
enum NotificationServiceError {
    #[error("notification service configuration failed: {0}")]
    Config(NotificationConfigError),
    #[error("notification database connection failed: {0}")]
    Database(SqlxNotificationDatabaseError),
    #[error("notification database migration failed: {0}")]
    Migration(SqlxNotificationMigrationError),
    #[error("notification service failed: {0}")]
    Serve(NotificationServeError),
    #[error("notification service socket bind failed: {0}")]
    Socket(StdNotificationIoError),
    #[error("notification service timeout configuration is invalid")]
    Timeout,
}
#[derive(Debug)]
struct NotificationConfigError(notification_service_config::ConfigTryFromEnvError);
#[derive(Debug)]
struct SqlxNotificationDatabaseError(sqlx::Error);
#[derive(Debug)]
struct SqlxNotificationMigrationError(sqlx::migrate::MigrateError);
#[derive(Debug)]
struct StdNotificationIoError(std::io::Error);
#[derive(Debug)]
struct NotificationServeError(server_runtime::ServeWithGracefulShutdownError);

impl std::fmt::Display for NotificationConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Display for SqlxNotificationDatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Display for SqlxNotificationMigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Display for StdNotificationIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::fmt::Display for NotificationServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

async fn create_notification(
    state: AxumNotificationState,
    request: AxumNotificationJson,
) -> Result<AxumNotificationResponse, HttpNotificationStatusCode> {
    let id = uuid::Uuid::new_v4();
    let message = request.0.into_message();
    let _created = sqlx::query(str_constants::NOTIFICATION_INSERT_SQL)
        .bind(id)
        .bind(message.as_ref())
        .execute(state.0.pool.as_ref())
        .await
        .map_err(|error| {
            tracing::error!(error = %error, "notification persistence failed");
            HttpNotificationStatusCode(http::StatusCode::INTERNAL_SERVER_ERROR)
        })?;
    Ok(AxumNotificationResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(notification_service_contract::CreateNotificationRes::new(
                notification_service_contract::UuidNotificationId::from(id),
            )),
        )),
    ))
}

async fn readiness(state: AxumNotificationState) -> HttpNotificationStatusCode {
    match sqlx::query(str_constants::COMMON_ROUTES_HEALTH_CHECK_SQL)
        .execute(state.0.pool.as_ref())
        .await
    {
        Ok(_result) => HttpNotificationStatusCode(http::StatusCode::OK),
        Err(error) => {
            tracing::error!(error = %error, "notification readiness probe failed");
            HttpNotificationStatusCode(http::StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

fn router(state: NotificationState) -> AxumNotificationRouter {
    AxumNotificationRouter(
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
                str_constants::NOTIFICATION_ROUTE_PATH,
                axum::routing::post(create_notification),
            )
            .with_state(state),
    )
}

async fn shutdown_signal() {
    if let Err(error) = tokio::signal::ctrl_c().await {
        tracing::error!(error = %error, "notification shutdown signal failed");
    }
}

async fn run(config: notification_service_config::Config) -> Result<(), NotificationServiceError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10u32)
        .connect(secrecy::ExposeSecret::expose_secret(
            &config.notification_database_url().0,
        ))
        .await
        .map_err(|error| {
            NotificationServiceError::Database(SqlxNotificationDatabaseError(error))
        })?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|error| {
            NotificationServiceError::Migration(SqlxNotificationMigrationError(error))
        })?;
    let listener = tokio::net::TcpListener::bind(config.notification_service_socket_address().0)
        .await
        .map_err(|error| NotificationServiceError::Socket(StdNotificationIoError(error)))?;
    let timeout =
        server_runtime::StdRequestTimeout::try_from(std::time::Duration::from_secs(30u64))
            .map_err(|_error| NotificationServiceError::Timeout)?;
    let service_router = server_runtime::RequestIdLayer.apply(
        server_runtime::SecurityHeadersLayer::from(server_runtime::ForwardedProtoTrust::Ignore)
            .apply(
                server_runtime::RequestTimeoutLayer::from(timeout).apply(
                    server_runtime::AxumRouter::from(
                        router(NotificationState {
                            pool: app_state::SqlxPgPool::from(pool),
                        })
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
    .map_err(|error| NotificationServiceError::Serve(NotificationServeError(error)))
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
            return StdNotificationExitCode(std::process::ExitCode::FAILURE);
        }
    };
    tracing_subscriber::util::SubscriberInitExt::init(
        tracing_subscriber::layer::SubscriberExt::with(
            tracing_subscriber::registry(),
            tracing_subscriber::fmt::layer().json(),
        ),
    );
    match run(config).await {
        Ok(()) => StdNotificationExitCode(std::process::ExitCode::SUCCESS),
        Err(error) => {
            eprintln!("{error}");
            StdNotificationExitCode(std::process::ExitCode::FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn router_contains_service_owned_routes() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(
                str_constants::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
            )
            .expect("52a25be1");
        let _router = super::router(super::NotificationState {
            pool: app_state::SqlxPgPool::from(pool),
        });
    }
}
