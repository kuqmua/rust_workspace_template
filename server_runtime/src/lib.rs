const REQUEST_ID_HEADER_NAME: &str = "x-request-id";
const CORRELATION_ID_HEADER_NAME: &str = "x-correlation-id";
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestId(String);
impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl TryFrom<String> for RequestId {
    type Error = RequestIdTryFromStringEr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 128usize || !value.is_ascii() {
            Err(RequestIdTryFromStringEr)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestIdTryFromStringEr;
impl std::fmt::Display for RequestIdTryFromStringEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request id must be non-empty ASCII up to 128 bytes")
    }
}
impl std::error::Error for RequestIdTryFromStringEr {}
#[derive(Debug)]
pub struct HttpHeaderToStrEr(http::header::ToStrError);
impl std::fmt::Display for HttpHeaderToStrEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for HttpHeaderToStrEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum RequestIdTryFromHttpHeaderValueEr {
    Invalid(RequestIdTryFromStringEr),
    ToStr(HttpHeaderToStrEr),
}
impl std::fmt::Display for RequestIdTryFromHttpHeaderValueEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(error) => error.fmt(f),
            Self::ToStr(error) => write!(f, "request id is not a text header: {error}"),
        }
    }
}
impl std::error::Error for RequestIdTryFromHttpHeaderValueEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Invalid(error) => Some(error),
            Self::ToStr(error) => Some(error),
        }
    }
}
impl TryFrom<&http::HeaderValue> for RequestId {
    type Error = RequestIdTryFromHttpHeaderValueEr;
    fn try_from(value: &http::HeaderValue) -> Result<Self, Self::Error> {
        let value_text = value
            .to_str()
            .map_err(|error| RequestIdTryFromHttpHeaderValueEr::ToStr(HttpHeaderToStrEr(error)))?;
        Self::try_from(value_text.to_owned()).map_err(RequestIdTryFromHttpHeaderValueEr::Invalid)
    }
}
impl TryFrom<&RequestId> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: &RequestId) -> Result<Self, Self::Error> {
        Self::from_str(value.0.as_str())
    }
}
#[derive(Debug)]
pub struct AxumRouter(axum::Router);
impl From<axum::Router> for AxumRouter {
    fn from(value: axum::Router) -> Self {
        Self(value)
    }
}
impl From<AxumRouter> for axum::Router {
    fn from(value: AxumRouter) -> Self {
        value.0
    }
}
#[derive(Clone, Debug)]
pub struct ReqwestClient(reqwest::Client);
impl Default for ReqwestClient {
    fn default() -> Self {
        Self(reqwest::Client::new())
    }
}
impl From<ReqwestClient> for reqwest::Client {
    fn from(value: ReqwestClient) -> Self {
        value.0
    }
}
#[derive(Debug)]
pub struct TokioTaskJoinHandle(tokio::task::JoinHandle<()>);
impl TokioTaskJoinHandle {
    pub fn abort(&self) {
        self.0.abort();
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRunInterval(std::time::Duration);
impl TryFrom<std::time::Duration> for StdRunInterval {
    type Error = StdRunIntervalTryFromDurationEr;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdRunIntervalTryFromDurationEr)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRunIntervalTryFromDurationEr;
impl std::fmt::Display for StdRunIntervalTryFromDurationEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("run interval must be greater than zero")
    }
}
impl std::error::Error for StdRunIntervalTryFromDurationEr {}
#[derive(Debug)]
pub struct ServiceRuntime {
    optional_task: Option<TokioTaskJoinHandle>,
    router: AxumRouter,
}
impl ServiceRuntime {
    #[must_use]
    pub fn into_parts(self) -> (AxumRouter, Option<TokioTaskJoinHandle>) {
        (self.router, self.optional_task)
    }
    #[must_use]
    pub const fn new(router: AxumRouter, optional_task: Option<TokioTaskJoinHandle>) -> Self {
        Self {
            optional_task,
            router,
        }
    }
}
#[derive(Debug)]
pub struct TokioTcpListener(tokio::net::TcpListener);
impl From<tokio::net::TcpListener> for TokioTcpListener {
    fn from(value: tokio::net::TcpListener) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct RequestIdLayer;
impl RequestIdLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter(router.0.layer(RequestIdTowerLayer))
    }
}
#[derive(Clone, Copy, Debug)]
struct RequestIdTowerLayer;
#[derive(Clone, Debug)]
struct RequestIdService<Service> {
    inner: Service,
}
impl<Service> tower::Layer<Service> for RequestIdTowerLayer {
    type Service = RequestIdService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        RequestIdService { inner }
    }
}
impl<Service> tower::Service<axum::extract::Request> for RequestIdService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Send
        + 'static,
    Service::Future: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        let request_id = [REQUEST_ID_HEADER_NAME, CORRELATION_ID_HEADER_NAME]
            .into_iter()
            .find_map(|header_name| {
                req.headers()
                    .get(header_name)
                    .and_then(|value| RequestId::try_from(value).ok())
            })
            .unwrap_or_else(|| {
                loop {
                    if let Ok(value) = RequestId::try_from(uuid::Uuid::new_v4().to_string()) {
                        break value;
                    }
                }
            });
        let _previous_extension_request_id = req.extensions_mut().insert(request_id.clone());
        let method = req.method().clone();
        let path = req.uri().path().to_owned();
        let started_at = tokio::time::Instant::now();
        let span = tracing::info_span!("http.request", request_id = %request_id, method = %method, path = %path);
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(tracing::Instrument::instrument(
            async move {
                let mut response = response_future.await?;
                tracing::info!(request_id = %request_id, status = response.status().as_u16(), duration_ms = started_at.elapsed().as_millis(), "http request completed");
                if let Ok(value) = http::HeaderValue::try_from(&request_id) {
                    let _previous_header_request_id = response.headers_mut().insert(
                        http::HeaderName::from_static(REQUEST_ID_HEADER_NAME),
                        value.clone(),
                    );
                    let _previous_correlation_id = response.headers_mut().insert(
                        http::HeaderName::from_static(CORRELATION_ID_HEADER_NAME),
                        value,
                    );
                }
                Ok(response)
            },
            span,
        ))
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRequestTimeout(std::time::Duration);
impl TryFrom<std::time::Duration> for StdRequestTimeout {
    type Error = StdRequestTimeoutTryFromDurationEr;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdRequestTimeoutTryFromDurationEr)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdRequestTimeoutTryFromDurationEr;
impl std::fmt::Display for StdRequestTimeoutTryFromDurationEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("request timeout must be greater than zero")
    }
}
impl std::error::Error for StdRequestTimeoutTryFromDurationEr {}
#[derive(Clone, Copy, Debug)]
pub struct RequestTimeoutLayer(StdRequestTimeout);
impl From<StdRequestTimeout> for RequestTimeoutLayer {
    fn from(value: StdRequestTimeout) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(transparent)]
struct StdRequestTimeoutMsg(&'static str);
#[derive(Debug, serde::Serialize)]
struct RequestTimeoutBody {
    error: StdRequestTimeoutMsg,
}
impl RequestTimeoutLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter(router.0.layer(RequestTimeoutTowerLayer(self.0)))
    }
}
#[derive(Clone, Copy, Debug)]
struct RequestTimeoutTowerLayer(StdRequestTimeout);
#[derive(Clone, Debug)]
struct RequestTimeoutService<Service> {
    inner: Service,
    timeout: StdRequestTimeout,
}
impl<Service> tower::Layer<Service> for RequestTimeoutTowerLayer {
    type Service = RequestTimeoutService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        RequestTimeoutService {
            inner,
            timeout: self.0,
        }
    }
}
impl<Service> tower::Service<axum::extract::Request> for RequestTimeoutService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Send
        + 'static,
    Service::Future: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, req: axum::extract::Request) -> Self::Future {
        let response_future = tower::Service::call(&mut self.inner, req);
        let timeout = self.timeout;
        Box::pin(async move {
            match tokio::time::timeout(timeout.0, response_future).await {
                Ok(response) => response,
                Err(_elapsed) => {
                    let retry_after = timeout.0.as_secs().max(1u64).to_string();
                    let mut response = axum::response::IntoResponse::into_response((
                        http::StatusCode::SERVICE_UNAVAILABLE,
                        axum::Json(RequestTimeoutBody {
                            error: StdRequestTimeoutMsg("request timeout"),
                        }),
                    ));
                    if let Ok(value) = http::HeaderValue::from_str(retry_after.as_str()) {
                        let _previous = response
                            .headers_mut()
                            .insert(http::header::RETRY_AFTER, value);
                    }
                    Ok(response)
                }
            }
        })
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForwardedProtoTrust {
    Ignore,
    Trust,
}
#[derive(Clone, Copy, Debug)]
pub struct SecurityHeadersLayer {
    forwarded_proto_trust: ForwardedProtoTrust,
}
impl From<ForwardedProtoTrust> for SecurityHeadersLayer {
    fn from(value: ForwardedProtoTrust) -> Self {
        Self {
            forwarded_proto_trust: value,
        }
    }
}
impl SecurityHeadersLayer {
    #[must_use]
    pub fn apply(self, router: AxumRouter) -> AxumRouter {
        AxumRouter(
            router
                .0
                .layer(SecurityHeadersTowerLayer(self.forwarded_proto_trust)),
        )
    }
}
#[derive(Clone, Copy, Debug)]
struct SecurityHeadersTowerLayer(ForwardedProtoTrust);
#[derive(Clone, Debug)]
struct SecurityHeadersService<Service> {
    forwarded_proto_trust: ForwardedProtoTrust,
    inner: Service,
}
impl<Service> tower::Layer<Service> for SecurityHeadersTowerLayer {
    type Service = SecurityHeadersService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        SecurityHeadersService {
            forwarded_proto_trust: self.0,
            inner,
        }
    }
}
impl<Service> tower::Service<axum::extract::Request> for SecurityHeadersService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Send
        + 'static,
    Service::Future: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, req: axum::extract::Request) -> Self::Future {
        let is_api_path = req.uri().path().starts_with("/api/");
        let is_forwarded_https = matches!(self.forwarded_proto_trust, ForwardedProtoTrust::Trust)
            && req
                .headers()
                .get("x-forwarded-proto")
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    value
                        .split(',')
                        .next()
                        .is_some_and(|first| first.trim().eq_ignore_ascii_case("https"))
                });
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(async move {
            let mut response = response_future.await?;
            let _content_type_options = response.headers_mut().insert(
                http::HeaderName::from_static("x-content-type-options"),
                http::HeaderValue::from_static("nosniff"),
            );
            let _frame_options = response.headers_mut().insert(
                http::HeaderName::from_static("x-frame-options"),
                http::HeaderValue::from_static("DENY"),
            );
            let _referrer_policy = response.headers_mut().insert(
                http::HeaderName::from_static("referrer-policy"),
                http::HeaderValue::from_static("no-referrer"),
            );
            if is_api_path {
                let _cache_control = response.headers_mut().insert(
                    http::header::CACHE_CONTROL,
                    http::HeaderValue::from_static("no-store"),
                );
            }
            if is_forwarded_https {
                let _strict_transport_security = response.headers_mut().insert(
                    http::HeaderName::from_static("strict-transport-security"),
                    http::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                );
            }
            Ok(response)
        })
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
#[derive(Debug)]
struct StdVecDequeRunReports<RunReport>(std::collections::VecDeque<RunReport>);
#[derive(Debug)]
struct TokioRwLockRunReports<RunReport>(tokio::sync::RwLock<StdVecDequeRunReports<RunReport>>);
#[derive(Debug)]
struct StdArcSharedRunReports<RunReport>(std::sync::Arc<TokioRwLockRunReports<RunReport>>);
impl<RunReport> Clone for StdArcSharedRunReports<RunReport> {
    fn clone(&self) -> Self {
        Self(std::sync::Arc::clone(&self.0))
    }
}
#[derive(Debug)]
pub struct AsyncRunHistory<RunReport> {
    maximum_len: StdAsyncRunHistoryMaximumLen,
    reports: StdArcSharedRunReports<RunReport>,
}
impl<RunReport> Clone for AsyncRunHistory<RunReport> {
    fn clone(&self) -> Self {
        Self {
            maximum_len: self.maximum_len,
            reports: self.reports.clone(),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAsyncRunHistoryMaximumLen(std::num::NonZeroUsize);
impl TryFrom<usize> for StdAsyncRunHistoryMaximumLen {
    type Error = StdAsyncRunHistoryMaximumLenTryFromUsizeEr;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self)
            .ok_or(StdAsyncRunHistoryMaximumLenTryFromUsizeEr)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAsyncRunHistoryMaximumLenTryFromUsizeEr;
impl std::fmt::Display for StdAsyncRunHistoryMaximumLenTryFromUsizeEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("run history maximum length must be greater than zero")
    }
}
impl std::error::Error for StdAsyncRunHistoryMaximumLenTryFromUsizeEr {}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdAsyncRunHistoryReportCount(usize);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncRunHistorySnapshot<RunReport> {
    latest_report: Option<RunReport>,
    report_count: StdAsyncRunHistoryReportCount,
}
impl<RunReport> AsyncRunHistorySnapshot<RunReport> {
    #[must_use]
    pub const fn latest_report(&self) -> Option<&RunReport> {
        self.latest_report.as_ref()
    }
    #[must_use]
    pub const fn report_count(&self) -> StdAsyncRunHistoryReportCount {
        self.report_count
    }
}
impl From<StdAsyncRunHistoryReportCount> for usize {
    fn from(value: StdAsyncRunHistoryReportCount) -> Self {
        value.0
    }
}
impl<RunReport: Send + Sync> AsyncRunHistory<RunReport> {
    #[must_use]
    pub fn new(maximum_len: StdAsyncRunHistoryMaximumLen) -> Self {
        let reports = StdVecDequeRunReports(std::collections::VecDeque::with_capacity(
            maximum_len.0.get(),
        ));
        Self {
            maximum_len,
            reports: StdArcSharedRunReports(std::sync::Arc::from(TokioRwLockRunReports(
                tokio::sync::RwLock::new(reports),
            ))),
        }
    }
    pub async fn push(&self, report: RunReport) {
        let mut reports = self.reports.0.0.write().await;
        if reports.0.len() == self.maximum_len.0.get() {
            let _removed = reports.0.pop_front();
        }
        reports.0.push_back(report);
    }
}
impl<RunReport: Clone + Send + Sync> AsyncRunHistory<RunReport> {
    pub async fn snapshot(&self) -> AsyncRunHistorySnapshot<RunReport> {
        let reports = self.reports.0.0.read().await;
        AsyncRunHistorySnapshot {
            latest_report: reports.0.back().cloned(),
            report_count: StdAsyncRunHistoryReportCount(reports.0.len()),
        }
    }
}
#[derive(Debug)]
pub struct StdServeIoEr(std::io::Error);
impl std::fmt::Display for StdServeIoEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for StdServeIoEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum ServeWithGracefulShutdownEr {
    Serve(StdServeIoEr),
    ShutdownTimeout,
}
impl std::fmt::Display for ServeWithGracefulShutdownEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serve(error) => write!(f, "server failed: {error}"),
            Self::ShutdownTimeout => f.write_str("server graceful shutdown timed out"),
        }
    }
}
impl std::error::Error for ServeWithGracefulShutdownEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serve(error) => Some(error),
            Self::ShutdownTimeout => None,
        }
    }
}
#[must_use]
pub fn add_status_route(router: AxumRouter) -> AxumRouter {
    AxumRouter(
        router
            .0
            .route("/status", axum::routing::get(async || http::StatusCode::OK)),
    )
}
#[must_use]
pub fn spawn_interval_task<Run, RunFuture>(
    optional_interval: Option<StdRunInterval>,
    mut run: Run,
) -> Option<TokioTaskJoinHandle>
where
    Run: FnMut() -> RunFuture + Send + 'static,
    RunFuture: Future<Output = ()> + Send + 'static,
{
    let interval = optional_interval?;
    Some(TokioTaskJoinHandle(tokio::spawn(async move {
        let mut timer = tokio::time::interval(interval.0);
        timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            let _tick = timer.tick().await;
            run().await;
        }
    })))
}
#[allow(clippy::integer_division_remainder_used)] // tokio::select expansion uses internal randomized branch arithmetic
pub async fn serve_with_graceful_shutdown<Shutdown>(
    listener: TokioTcpListener,
    router: AxumRouter,
    shutdown: Shutdown,
    shutdown_timeout: StdRequestTimeout,
) -> Result<(), ServeWithGracefulShutdownEr>
where
    Shutdown: Future<Output = ()> + Send + 'static,
{
    let (shutdown_started_tx, shutdown_started_rx) = tokio::sync::oneshot::channel();
    let server = IntoFuture::into_future(
        axum::serve(listener.0, router.0.into_make_service()).with_graceful_shutdown(async move {
            shutdown.await;
            let _send_result = shutdown_started_tx.send(());
        }),
    );
    tokio::pin!(server);
    tokio::select! {
        result = &mut server => result.map_err(|error| ServeWithGracefulShutdownEr::Serve(StdServeIoEr(error))),
        shutdown_result = shutdown_started_rx => {
            drop(shutdown_result);
            tokio::time::timeout(shutdown_timeout.0, &mut server)
                .await
                .map_err(|_elapsed| ServeWithGracefulShutdownEr::ShutdownTimeout)?
                .map_err(|error| ServeWithGracefulShutdownEr::Serve(StdServeIoEr(error)))
        }
    }
}
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn async_run_history_keeps_latest_reports() {
        let history = super::AsyncRunHistory::new(
            super::StdAsyncRunHistoryMaximumLen::try_from(2usize).expect("8567a9df"),
        );
        history.push(1u8).await;
        history.push(2u8).await;
        history.push(3u8).await;
        let snapshot = history.snapshot().await;
        assert_eq!(usize::from(snapshot.report_count()), 2usize);
        assert_eq!(snapshot.latest_report(), Some(&3u8));
    }
    #[tokio::test]
    async fn service_runtime_status_route_and_parts_are_stable() {
        let runtime = super::ServiceRuntime::new(
            super::add_status_route(super::AxumRouter::from(axum::Router::new())),
            None,
        );
        let (router, optional_task) = runtime.into_parts();
        assert!(optional_task.is_none());
        let response = tower::ServiceExt::oneshot(
            axum::Router::from(router),
            axum::extract::Request::builder()
                .uri("/status")
                .body(axum::body::Body::empty())
                .expect("8e9c3da1"),
        )
        .await
        .expect("1e97ad3b");
        assert_eq!(response.status(), http::StatusCode::OK);
        let optional_interval_task = super::spawn_interval_task(None, async || {});
        assert!(optional_interval_task.is_none());
    }
    #[tokio::test]
    async fn security_headers_only_trust_forwarded_proto_when_configured() {
        let make_request = || {
            axum::extract::Request::builder()
                .uri("/api/v1/test")
                .header("x-forwarded-proto", "https")
                .body(axum::body::Body::empty())
                .expect("94149bdd")
        };
        let make_router = |trust| {
            axum::Router::from(super::SecurityHeadersLayer::from(trust).apply(
                super::AxumRouter::from(axum::Router::new().route(
                    "/api/v1/test",
                    axum::routing::get(async || http::StatusCode::OK),
                )),
            ))
        };
        let ignored_response = tower::ServiceExt::oneshot(
            make_router(super::ForwardedProtoTrust::Ignore),
            make_request(),
        )
        .await
        .expect("8c89e84f");
        assert!(
            ignored_response
                .headers()
                .get("strict-transport-security")
                .is_none()
        );
        let trusted_response = tower::ServiceExt::oneshot(
            make_router(super::ForwardedProtoTrust::Trust),
            make_request(),
        )
        .await
        .expect("db05c4be");
        assert!(
            trusted_response
                .headers()
                .get("strict-transport-security")
                .is_some()
        );
        assert_eq!(
            trusted_response.headers().get(http::header::CACHE_CONTROL),
            Some(&http::HeaderValue::from_static("no-store"))
        );
        assert_eq!(
            trusted_response.headers().get("x-content-type-options"),
            Some(&http::HeaderValue::from_static("nosniff"))
        );
    }
    #[test]
    fn zero_limits_are_rejected() {
        let Err(history_error) = super::StdAsyncRunHistoryMaximumLen::try_from(0usize) else {
            panic!("5500cd77");
        };
        assert_eq!(
            history_error,
            super::StdAsyncRunHistoryMaximumLenTryFromUsizeEr
        );
        let Err(timeout_error) = super::StdRequestTimeout::try_from(std::time::Duration::ZERO)
        else {
            panic!("bca83cb0");
        };
        assert_eq!(timeout_error, super::StdRequestTimeoutTryFromDurationEr);
    }
}
