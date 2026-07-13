const REQUEST_ID_HEADER_NAME: &str = "x-request-id";
const CORRELATION_ID_HEADER_NAME: &str = "x-correlation-id";
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetMaximum(usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetAmount(usize);
impl From<usize> for ResourceBudgetAmount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
struct StdAtomicUsize(std::sync::atomic::AtomicUsize);
#[derive(Clone, Debug)]
struct StdSharedAtomicUsize(std::sync::Arc<StdAtomicUsize>);
impl TryFrom<usize> for ResourceBudgetMaximum {
    type Error = ResourceBudgetConfigEr;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0usize {
            Err(ResourceBudgetConfigEr)
        } else {
            Ok(Self(value))
        }
    }
}
impl From<std::num::NonZeroUsize> for ResourceBudgetMaximum {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(value.get())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetConfigEr;
impl std::fmt::Display for ResourceBudgetConfigEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("resource budget maximum must be greater than zero")
    }
}
impl std::error::Error for ResourceBudgetConfigEr {}
#[derive(Clone, Debug)]
pub struct ResourceBudget {
    maximum: ResourceBudgetMaximum,
    reserved: StdSharedAtomicUsize,
}
pub trait GetBulkItemResourceBudget {
    fn get_bulk_item_resource_budget(&self) -> &ResourceBudget;
}
pub trait GetIdempotencyResponseResourceBudget {
    fn get_idempotency_response_resource_budget(&self) -> &ResourceBudget;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceBudgetReserveEr {
    Exhausted,
    Overflow,
}
impl std::fmt::Display for ResourceBudgetReserveEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exhausted => f.write_str("resource budget exhausted"),
            Self::Overflow => f.write_str("resource budget reservation overflow"),
        }
    }
}
impl std::error::Error for ResourceBudgetReserveEr {}
#[derive(Debug)]
#[must_use]
pub struct ResourceBudgetReservation {
    amount: ResourceBudgetAmount,
    reserved: StdSharedAtomicUsize,
}
impl ResourceBudget {
    #[must_use]
    pub fn new(maximum: ResourceBudgetMaximum) -> Self {
        Self {
            maximum,
            reserved: StdSharedAtomicUsize(std::sync::Arc::from(StdAtomicUsize(
                std::sync::atomic::AtomicUsize::new(0usize),
            ))),
        }
    }
    pub fn reserve(
        &self,
        amount: ResourceBudgetAmount,
    ) -> Result<ResourceBudgetReservation, ResourceBudgetReserveEr> {
        let result = self.reserved.0.0.try_update(
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            |current| {
                current
                    .checked_add(amount.0)
                    .filter(|next| *next <= self.maximum.0)
            },
        );
        match result {
            Ok(_previous) => Ok(ResourceBudgetReservation {
                amount,
                reserved: self.reserved.clone(),
            }),
            Err(current) if current.checked_add(amount.0).is_none() => {
                Err(ResourceBudgetReserveEr::Overflow)
            }
            Err(_current) => Err(ResourceBudgetReserveEr::Exhausted),
        }
    }
    #[must_use]
    pub fn reserved(&self) -> ResourceBudgetAmount {
        ResourceBudgetAmount::from(self.reserved.0.0.load(std::sync::atomic::Ordering::Acquire))
    }
}
impl Drop for ResourceBudgetReservation {
    fn drop(&mut self) {
        let _previous = self
            .reserved
            .0
            .0
            .fetch_sub(self.amount.0, std::sync::atomic::Ordering::AcqRel);
    }
}
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackgroundTaskOutcome {
    Completed,
    ShutdownRequested,
}
#[derive(Debug)]
pub struct TokioTaskJoinEr(tokio::task::JoinError);
impl std::fmt::Display for TokioTaskJoinEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for TokioTaskJoinEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum BackgroundTaskShutdownEr {
    Join(TokioTaskJoinEr),
    Timeout,
}
impl std::fmt::Display for BackgroundTaskShutdownEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Join(error) => write!(f, "background task failed: {error}"),
            Self::Timeout => f.write_str("background task shutdown timed out"),
        }
    }
}
impl std::error::Error for BackgroundTaskShutdownEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Join(error) => Some(error),
            Self::Timeout => None,
        }
    }
}
#[derive(Debug)]
#[must_use]
pub struct BackgroundTask {
    handle: Option<TokioBackgroundTaskJoinHandle>,
    shutdown_tx: Option<TokioBackgroundTaskShutdownSender>,
}
#[derive(Debug)]
struct TokioBackgroundTaskJoinHandle(tokio::task::JoinHandle<BackgroundTaskOutcome>);
impl From<tokio::task::JoinHandle<BackgroundTaskOutcome>> for TokioBackgroundTaskJoinHandle {
    fn from(value: tokio::task::JoinHandle<BackgroundTaskOutcome>) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
struct TokioBackgroundTaskShutdownSender(tokio::sync::oneshot::Sender<()>);
impl From<tokio::sync::oneshot::Sender<()>> for TokioBackgroundTaskShutdownSender {
    fn from(value: tokio::sync::oneshot::Sender<()>) -> Self {
        Self(value)
    }
}
impl BackgroundTask {
    pub async fn join(mut self) -> Result<BackgroundTaskOutcome, BackgroundTaskShutdownEr> {
        let shutdown_tx = self.shutdown_tx.take();
        let result = match self.handle.take() {
            Some(handle) => handle
                .0
                .await
                .map_err(|error| BackgroundTaskShutdownEr::Join(TokioTaskJoinEr(error))),
            None => Ok(BackgroundTaskOutcome::Completed),
        };
        drop(shutdown_tx);
        result
    }
    pub async fn shutdown(
        mut self,
        timeout: StdRequestTimeout,
    ) -> Result<BackgroundTaskOutcome, BackgroundTaskShutdownEr> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = shutdown_tx.0.send(());
        }
        let Some(mut handle) = self.handle.take().map(|value| value.0) else {
            return Ok(BackgroundTaskOutcome::ShutdownRequested);
        };
        match tokio::time::timeout(timeout.0, &mut handle).await {
            Ok(result) => {
                result.map_err(|error| BackgroundTaskShutdownEr::Join(TokioTaskJoinEr(error)))
            }
            Err(_elapsed) => {
                handle.abort();
                match handle.await {
                    Ok(_) | Err(_) => Err(BackgroundTaskShutdownEr::Timeout),
                }
            }
        }
    }
}
impl Drop for BackgroundTask {
    fn drop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _send_result = shutdown_tx.0.send(());
        }
        if let Some(handle) = self.handle.take() {
            handle.0.abort();
        }
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
    optional_task: Option<BackgroundTask>,
    router: AxumRouter,
}
impl ServiceRuntime {
    #[must_use]
    pub fn into_parts(self) -> (AxumRouter, Option<BackgroundTask>) {
        (self.router, self.optional_task)
    }
    #[must_use]
    pub const fn new(router: AxumRouter, optional_task: Option<BackgroundTask>) -> Self {
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
        let request_id_and_header_value = [REQUEST_ID_HEADER_NAME, CORRELATION_ID_HEADER_NAME]
            .into_iter()
            .find_map(|header_name| {
                req.headers().get(header_name).and_then(|value| {
                    RequestId::try_from(value)
                        .ok()
                        .map(|request_id| (request_id, value.clone()))
                })
            })
            .unwrap_or_else(|| {
                loop {
                    if let Ok(value) = RequestId::try_from(uuid::Uuid::new_v4().to_string())
                        && let Ok(header_value) = http::HeaderValue::try_from(&value)
                    {
                        break (value, header_value);
                    }
                }
            });
        let started_at = tokio::time::Instant::now();
        let span = tracing::info_span!("http.request", request_id = %request_id_and_header_value.0, method = %req.method(), path = %req.uri().path());
        let _previous_extension_request_id =
            req.extensions_mut().insert(request_id_and_header_value.0);
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(tracing::Instrument::instrument(
            async move {
                let mut response = response_future.await?;
                tracing::info!(
                    status = response.status().as_u16(),
                    duration_ms = started_at.elapsed().as_millis(),
                    "http request completed"
                );
                let _previous_header_request_id = response.headers_mut().insert(
                    http::HeaderName::from_static(REQUEST_ID_HEADER_NAME),
                    request_id_and_header_value.1.clone(),
                );
                let _previous_correlation_id = response.headers_mut().insert(
                    http::HeaderName::from_static(CORRELATION_ID_HEADER_NAME),
                    request_id_and_header_value.1,
                );
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StdPermitWaitTimeout(std::time::Duration);
impl From<std::time::Duration> for StdPermitWaitTimeout {
    fn from(value: std::time::Duration) -> Self {
        Self(value)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryAfterSecs(u64);
impl TryFrom<u64> for RetryAfterSecs {
    type Error = RetryAfterSecsTryFromU64Er;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0u64 {
            Err(RetryAfterSecsTryFromU64Er)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryAfterSecsTryFromU64Er;
impl std::fmt::Display for RetryAfterSecsTryFromU64Er {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("retry-after seconds must be greater than zero")
    }
}
impl std::error::Error for RetryAfterSecsTryFromU64Er {}
impl TryFrom<RetryAfterSecs> for http::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: RetryAfterSecs) -> Result<Self, Self::Error> {
        Self::from_str(value.0.to_string().as_str())
    }
}
#[derive(Clone, Debug)]
pub struct StdArcTokioSemaphore(std::sync::Arc<tokio::sync::Semaphore>);
impl From<std::sync::Arc<tokio::sync::Semaphore>> for StdArcTokioSemaphore {
    fn from(value: std::sync::Arc<tokio::sync::Semaphore>) -> Self {
        Self(value)
    }
}
#[derive(Debug)]
pub struct TokioAcquireEr(tokio::sync::AcquireError);
impl std::fmt::Display for TokioAcquireEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for TokioAcquireEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
#[derive(Debug)]
pub enum AcquirePermitEr {
    Closed(TokioAcquireEr),
    Timeout(RetryAfterSecs),
}
#[derive(Debug)]
pub struct TokioOwnedSemaphorePermit(tokio::sync::OwnedSemaphorePermit);
impl From<tokio::sync::OwnedSemaphorePermit> for TokioOwnedSemaphorePermit {
    fn from(value: tokio::sync::OwnedSemaphorePermit) -> Self {
        Self(value)
    }
}
impl TokioOwnedSemaphorePermit {
    pub fn forget(self) {
        self.0.forget();
    }
}
impl std::fmt::Display for AcquirePermitEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed(error) => write!(f, "concurrency limiter is closed: {error}"),
            Self::Timeout(retry_after) => {
                write!(
                    f,
                    "concurrency limit reached; retry after {} seconds",
                    retry_after.0
                )
            }
        }
    }
}
impl std::error::Error for AcquirePermitEr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Closed(error) => Some(error),
            Self::Timeout(_) => None,
        }
    }
}
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
pub async fn acquire_permit(
    semaphore: StdArcTokioSemaphore,
    wait_timeout: StdPermitWaitTimeout,
    retry_after: RetryAfterSecs,
) -> Result<TokioOwnedSemaphorePermit, AcquirePermitEr> {
    match tokio::time::timeout(wait_timeout.0, semaphore.0.acquire_owned()).await {
        Ok(Ok(permit)) => Ok(TokioOwnedSemaphorePermit::from(permit)),
        Ok(Err(error)) => Err(AcquirePermitEr::Closed(TokioAcquireEr(error))),
        Err(_elapsed) => Err(AcquirePermitEr::Timeout(retry_after)),
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
#[allow(clippy::integer_division_remainder_used)]
pub fn spawn_interval_task<Run, RunFuture>(
    optional_interval: Option<StdRunInterval>,
    mut run: Run,
) -> Option<BackgroundTask>
where
    Run: FnMut() -> RunFuture + Send + 'static,
    RunFuture: Future<Output = ()> + Send + 'static,
{
    let interval = optional_interval?;
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel();
    let handle = tokio::spawn(async move {
        let mut timer = tokio::time::interval(interval.0);
        timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                shutdown_result = &mut shutdown_rx => {
                    drop(shutdown_result);
                    return BackgroundTaskOutcome::ShutdownRequested;
                }
                _tick = timer.tick() => run().await,
            }
        }
    });
    Some(BackgroundTask {
        handle: Some(TokioBackgroundTaskJoinHandle::from(handle)),
        shutdown_tx: Some(TokioBackgroundTaskShutdownSender::from(shutdown_tx)),
    })
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
        axum::serve(
            listener.0,
            router
                .0
                .into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .with_graceful_shutdown(async move {
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
    #[test]
    fn resource_budget_reservations_are_bounded_and_released() {
        let budget = super::ResourceBudget::new(
            super::ResourceBudgetMaximum::try_from(5usize).expect("0c6362a4"),
        );
        let first = budget
            .reserve(super::ResourceBudgetAmount::from(3usize))
            .expect("3bfeb37c");
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(3usize));
        assert_eq!(
            budget
                .reserve(super::ResourceBudgetAmount::from(3usize))
                .expect_err("3c31187b"),
            super::ResourceBudgetReserveEr::Exhausted
        );
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(3usize));
        let second = budget
            .reserve(super::ResourceBudgetAmount::from(2usize))
            .expect("d86085db");
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(5usize));
        drop(first);
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(2usize));
        drop(second);
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(0usize));
    }
    #[test]
    fn resource_budget_reports_overflow_without_changing_count() {
        let budget = super::ResourceBudget::new(
            super::ResourceBudgetMaximum::try_from(usize::MAX).expect("65f2f229"),
        );
        let reservation = budget
            .reserve(super::ResourceBudgetAmount::from(1usize))
            .expect("1a2bb321");
        assert_eq!(
            budget
                .reserve(super::ResourceBudgetAmount::from(usize::MAX))
                .expect_err("e317c775"),
            super::ResourceBudgetReserveEr::Overflow
        );
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(1usize));
        drop(reservation);
    }
    #[test]
    fn concurrent_resource_budget_reservations_never_exceed_maximum() {
        let budget = super::ResourceBudget::new(
            super::ResourceBudgetMaximum::try_from(5usize).expect("57a61ca4"),
        );
        let start = std::sync::Arc::new(std::sync::Barrier::new(3usize));
        let finish = std::sync::Arc::new(std::sync::Barrier::new(3usize));
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::scope(|scope| {
            let left_budget = budget.clone();
            let left_start = std::sync::Arc::clone(&start);
            let left_finish = std::sync::Arc::clone(&finish);
            let left_tx = tx.clone();
            let _left_handle = scope.spawn(move || {
                let _start_result = left_start.wait();
                let reservation = left_budget.reserve(super::ResourceBudgetAmount::from(3usize));
                left_tx.send(reservation.is_ok()).expect("b048535e");
                let _finish_result = left_finish.wait();
                drop(reservation);
            });
            let right_budget = budget.clone();
            let right_start = std::sync::Arc::clone(&start);
            let right_finish = std::sync::Arc::clone(&finish);
            let right_tx = tx.clone();
            let _right_handle = scope.spawn(move || {
                let _start_result = right_start.wait();
                let reservation = right_budget.reserve(super::ResourceBudgetAmount::from(3usize));
                right_tx.send(reservation.is_ok()).expect("cd734995");
                let _finish_result = right_finish.wait();
                drop(reservation);
            });
            let _start_result = start.wait();
            let outcomes = [rx.recv().expect("7393afca"), rx.recv().expect("67824b65")];
            assert_eq!(outcomes.into_iter().filter(|value| *value).count(), 1usize);
            assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(3usize));
            let _finish_result = finish.wait();
        });
        assert_eq!(budget.reserved(), super::ResourceBudgetAmount::from(0usize));
    }
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
    async fn background_task_shutdown_is_observable() {
        let interval = super::StdRunInterval::try_from(std::time::Duration::from_secs(1u64))
            .expect("e76640c4");
        let task = super::spawn_interval_task(Some(interval), async || {}).expect("32858863");
        let timeout = super::StdRequestTimeout::try_from(std::time::Duration::from_secs(1u64))
            .expect("728b52b3");
        assert_eq!(
            task.shutdown(timeout).await.expect("0d71d1b8"),
            super::BackgroundTaskOutcome::ShutdownRequested
        );
    }
    #[tokio::test]
    async fn background_task_panic_is_observable() {
        let interval = super::StdRunInterval::try_from(std::time::Duration::from_secs(1u64))
            .expect("c9d73cab");
        let task = super::spawn_interval_task(Some(interval), async || panic!("62839854"))
            .expect("7a86a253");
        assert!(matches!(
            task.join().await,
            Err(super::BackgroundTaskShutdownEr::Join(_))
        ));
    }
    #[tokio::test(start_paused = true)]
    async fn stuck_background_task_reaches_shutdown_timeout() {
        let interval = super::StdRunInterval::try_from(std::time::Duration::from_secs(1u64))
            .expect("f797718f");
        let task = super::spawn_interval_task(Some(interval), async || {
            std::future::pending::<()>().await;
        })
        .expect("a58f09dc");
        tokio::task::yield_now().await;
        let timeout = super::StdRequestTimeout::try_from(std::time::Duration::from_secs(1u64))
            .expect("ae1262bb");
        let shutdown = tokio::spawn(task.shutdown(timeout));
        tokio::task::yield_now().await;
        tokio::time::advance(std::time::Duration::from_secs(1u64)).await;
        assert!(matches!(
            shutdown.await.expect("9e76a810"),
            Err(super::BackgroundTaskShutdownEr::Timeout)
        ));
    }
    #[tokio::test]
    async fn acquire_permit_distinguishes_available_timeout_and_closed() {
        let retry_after = super::RetryAfterSecs::try_from(3u64).expect("c52d0e93");
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(1usize));
        let permit = super::acquire_permit(
            super::StdArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
            super::StdPermitWaitTimeout::from(std::time::Duration::ZERO),
            retry_after,
        )
        .await
        .expect("e1394cd0");
        let timeout = super::acquire_permit(
            super::StdArcTokioSemaphore::from(std::sync::Arc::clone(&semaphore)),
            super::StdPermitWaitTimeout::from(std::time::Duration::ZERO),
            retry_after,
        )
        .await;
        assert!(matches!(
            timeout,
            Err(super::AcquirePermitEr::Timeout(value)) if value == retry_after
        ));
        drop(timeout);
        drop(permit);
        semaphore.close();
        let closed = super::acquire_permit(
            super::StdArcTokioSemaphore::from(semaphore),
            super::StdPermitWaitTimeout::from(std::time::Duration::ZERO),
            retry_after,
        )
        .await;
        assert!(matches!(closed, Err(super::AcquirePermitEr::Closed(_))));
        drop(closed);
        assert_eq!(
            http::HeaderValue::try_from(retry_after).expect("cb2a239c"),
            http::HeaderValue::from_static("3")
        );
    }
    #[tokio::test]
    async fn request_id_layer_propagates_existing_and_generated_values() {
        let make_router = || {
            axum::Router::from(super::RequestIdLayer.apply(super::AxumRouter::from(
                axum::Router::new().route("/", axum::routing::get(async || http::StatusCode::OK)),
            )))
        };
        let existing = http::HeaderValue::from_static("existing-request-id");
        let existing_response = tower::ServiceExt::oneshot(
            make_router(),
            axum::extract::Request::builder()
                .uri("/")
                .header(super::REQUEST_ID_HEADER_NAME, existing.clone())
                .body(axum::body::Body::empty())
                .expect("319b3cb4"),
        )
        .await
        .expect("d5a0693b");
        assert_eq!(
            existing_response
                .headers()
                .get(super::REQUEST_ID_HEADER_NAME),
            Some(&existing)
        );
        assert_eq!(
            existing_response
                .headers()
                .get(super::CORRELATION_ID_HEADER_NAME),
            Some(&existing)
        );
        let generated_response = tower::ServiceExt::oneshot(
            make_router(),
            axum::extract::Request::builder()
                .uri("/")
                .body(axum::body::Body::empty())
                .expect("27ce5fbd"),
        )
        .await
        .expect("4cd32371");
        let generated = generated_response
            .headers()
            .get(super::REQUEST_ID_HEADER_NAME)
            .expect("12ed6f85");
        assert_eq!(generated.as_bytes().len(), 36usize);
        assert_eq!(
            generated_response
                .headers()
                .get(super::CORRELATION_ID_HEADER_NAME),
            Some(generated)
        );
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
