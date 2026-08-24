const DEFAULT_HTTP_METRICS_PATH_CACHE_MAXIMUM: usize = 4_096usize;
const METRICS_RESPONSE_BODY_MAXIMUM_BYTES: usize = 8 * 1_024 * 1_024usize;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::IntoInner,
)]
pub struct MetricsResponseBody(String);
impl axum::response::IntoResponse for MetricsResponseBody {
    fn into_response(self) -> axum::response::Response {
        axum::response::IntoResponse::into_response(self.0)
    }
}

impl TryFrom<String> for MetricsResponseBody {
    type Error = MetricsResponseBodyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > METRICS_RESPONSE_BODY_MAXIMUM_BYTES {
            Err(MetricsResponseBodyError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = str_constants::METRICS_RESPONSE_BODY_EXCEEDS_MAXIMUM_LENGTH)]
pub struct MetricsResponseBodyError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct HttpMetricsPathCacheMaximum(usize);

impl TryFrom<usize> for HttpMetricsPathCacheMaximum {
    type Error = HttpMetricsPathCacheMaximumTryFromUsizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(|non_zero_value| Self(non_zero_value.get()))
            .ok_or(HttpMetricsPathCacheMaximumTryFromUsizeError)
    }
}

impl From<std::num::NonZeroUsize> for HttpMetricsPathCacheMaximum {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(value.get())
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{message}", message = str_constants::HTTP_METRICS_PATH_CACHE_MAXIMUM_MUST_BE_GREATER_THAN_ZERO)]
pub struct HttpMetricsPathCacheMaximumTryFromUsizeError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct HttpMetricsPathCache {
    entries: StdHttpMetricsPathEntries,
    maximum: HttpMetricsPathCacheMaximum,
    unmatched: MetricsSharedString,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct StdHttpMetricsPathEntries(
    std::sync::RwLock<std::collections::HashMap<HttpMetricsPathText, MetricsSharedString>>,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
struct MetricsSharedString(metrics::SharedString);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    newtype::BorrowStr,
)]
struct HttpMetricsPathText(String);

impl TryFrom<String> for HttpMetricsPathText {
    type Error = HttpMetricsPathTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > usize_constants::VALUE_8_192 {
            Err(HttpMetricsPathTextError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
struct HttpMetricsPathTextError;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct HttpMetricsPathTextRef<'path>(&'path str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
struct StdSharedHttpMetricsPathCache(std::sync::Arc<HttpMetricsPathCache>);

impl From<HttpMetricsPathCache> for StdSharedHttpMetricsPathCache {
    fn from(value: HttpMetricsPathCache) -> Self {
        Self(std::sync::Arc::from(value))
    }
}

#[allow(clippy::arbitrary_source_item_ordering)] // constructor precedes cache lookup implementation
impl HttpMetricsPathCache {
    #[allow(clippy::single_call_fn)] // cache construction owns its capacity invariant
    fn new(maximum: HttpMetricsPathCacheMaximum) -> Self {
        Self {
            entries: StdHttpMetricsPathEntries::from(std::sync::RwLock::new(
                std::collections::HashMap::with_capacity(
                    maximum.0.min(DEFAULT_HTTP_METRICS_PATH_CACHE_MAXIMUM),
                ),
            )),
            maximum,
            unmatched: MetricsSharedString::from(metrics::SharedString::const_str(
                str_constants::HTTP_METRICS_UNMATCHED_PATH,
            )),
        }
    }

    fn label(&self, path: HttpMetricsPathTextRef<'_>) -> MetricsSharedString {
        {
            let read_entries = self
                .entries
                .0
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(label) = read_entries.get(path.0) {
                return label.clone();
            }
            if read_entries.len() >= self.maximum.0 {
                return self.unmatched.clone();
            }
        }
        let mut write_entries = self
            .entries
            .0
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(label) = write_entries.get(path.0) {
            return label.clone();
        }
        if write_entries.len() >= self.maximum.0 {
            return self.unmatched.clone();
        }
        let Ok(path_text) = HttpMetricsPathText::try_from(path.0.to_owned()) else {
            return self.unmatched.clone();
        };
        let label = MetricsSharedString::from(metrics::SharedString::from(path_text.0.clone()));
        let _previous = write_entries.insert(path_text, label.clone());
        label
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct HttpMetricsLayer {
    paths: StdSharedHttpMetricsPathCache,
}

impl Default for HttpMetricsLayer {
    fn default() -> Self {
        Self::new(HttpMetricsPathCacheMaximum::from(
            std::num::NonZeroUsize::MIN.saturating_add(DEFAULT_HTTP_METRICS_PATH_CACHE_MAXIMUM - 1),
        ))
    }
}

impl HttpMetricsLayer {
    #[must_use]
    pub fn apply(self, router: crate::AxumRouter) -> crate::AxumRouter {
        crate::AxumRouter(router.0.layer(HttpMetricsTowerLayer { paths: self.paths }))
    }

    #[must_use]
    pub fn new(path_cache_maximum: HttpMetricsPathCacheMaximum) -> Self {
        Self {
            paths: StdSharedHttpMetricsPathCache::from(HttpMetricsPathCache::new(
                path_cache_maximum,
            )),
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
struct HttpMetricsTowerLayer {
    paths: StdSharedHttpMetricsPathCache,
}

impl<Service> tower::Layer<Service> for HttpMetricsTowerLayer {
    type Service = HttpMetricsService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        HttpMetricsService {
            inner,
            paths: self.paths.clone(),
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
struct HttpMetricsService<Service> {
    inner: Service,
    paths: StdSharedHttpMetricsPathCache,
}

impl<Service> tower::Service<axum::extract::Request> for HttpMetricsService<Service>
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
        let method = match *req.method() {
            http::Method::CONNECT => str_constants::HTTP_METHOD_CONNECT_LABEL,
            http::Method::DELETE => str_constants::DELETE,
            http::Method::GET => str_constants::GET,
            http::Method::HEAD => str_constants::HTTP_METHOD_HEAD_LABEL,
            http::Method::OPTIONS => str_constants::HTTP_METHOD_OPTIONS_LABEL,
            http::Method::PATCH => str_constants::PATCH,
            http::Method::POST => str_constants::POST,
            http::Method::PUT => str_constants::HTTP_METHOD_PUT_LABEL,
            http::Method::TRACE => str_constants::HTTP_METHOD_TRACE_LABEL,
            _ => str_constants::HTTP_METHOD_OTHER_LABEL,
        };
        let normalized_path = req
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .is_none()
            .then(|| {
                crate::normalize_identifier_path(crate::HttpRequestPathRef::from(req.uri().path()))
            })
            .flatten();
        let path_text = req
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .map(axum::extract::MatchedPath::as_str)
            .or_else(|| normalized_path.as_ref().map(AsRef::as_ref))
            .unwrap_or(str_constants::HTTP_METRICS_UNMATCHED_PATH);
        let path_label = self.paths.0.label(HttpMetricsPathTextRef::from(path_text));
        let started_at = std::time::Instant::now();
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(async move {
            let response = response_future.await?;
            let status = MetricsSharedString::from(metrics::SharedString::from(
                response.status().as_str().to_owned(),
            ));
            let labels = [
                metrics::Label::new(str_constants::HTTP_METRICS_LABEL_METHOD, method),
                metrics::Label::new(str_constants::PATH_ALT_5, path_label.0),
                metrics::Label::new(str_constants::STATUS_ALT, status.0),
            ];
            metrics::counter!(str_constants::HTTP_METRICS_REQUESTS_TOTAL, labels.iter())
                .increment(1u64);
            metrics::histogram!(
                str_constants::HTTP_METRICS_REQUEST_DURATION_SECONDS,
                labels.iter()
            )
            .record(started_at.elapsed().as_secs_f64());
            if response.status().is_server_error() {
                metrics::counter!(str_constants::HTTP_METRICS_ERRORS_TOTAL, labels.iter())
                    .increment(1u64);
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

#[cfg(test)]
mod tests {
    async fn call_method(router: axum::Router, method: http::Method) -> http::StatusCode {
        tower::ServiceExt::oneshot(
            router,
            axum::extract::Request::builder()
                .method(method)
                .uri("/items/123")
                .body(axum::body::Body::empty())
                .expect("49ef0e86 call_method invariant must hold"),
        )
        .await
        .expect("12a54113 call_method invariant must hold")
        .status()
    }

    #[test]
    fn metrics_response_body_is_bounded() {
        let _empty_body = super::MetricsResponseBody::try_from(String::new())
            .expect("52410ad9 metrics_response_body_is_bounded invariant must hold");
        let exact = String::from_utf8(vec![b'x'; super::METRICS_RESPONSE_BODY_MAXIMUM_BYTES])
            .expect("560d1f1e metrics_response_body_is_bounded invariant must hold");
        let _exact_body = super::MetricsResponseBody::try_from(exact)
            .expect("2701b706 metrics_response_body_is_bounded invariant must hold");
        let _error = super::MetricsResponseBody::try_from(
            String::from_utf8(vec![
                b'x';
                super::METRICS_RESPONSE_BODY_MAXIMUM_BYTES
                    .saturating_add(usize_constants::ONE)
            ])
            .expect("329fb604 metrics_response_body_is_bounded invariant must hold"),
        )
        .expect_err(str_constants::F0FC293DD);
    }

    #[test]
    fn cache_configuration_and_path_text_validate_boundaries() {
        assert_eq!(
            super::HttpMetricsPathCacheMaximum::try_from(usize_constants::ZERO),
            Err(super::HttpMetricsPathCacheMaximumTryFromUsizeError)
        );
        assert_eq!(
            super::HttpMetricsPathText::try_from(String::new()),
            Err(super::HttpMetricsPathTextError)
        );
        let _path = super::HttpMetricsPathText::try_from("a".repeat(usize_constants::VALUE_8_192)).expect(
            "c1b07056 cache_configuration_and_path_text_validate_boundaries invariant must hold",
        );
        assert_eq!(
            super::HttpMetricsPathText::try_from("a".repeat(8_193usize)),
            Err(super::HttpMetricsPathTextError)
        );
    }

    #[test]
    fn cache_is_bounded_and_reuses_labels() {
        let cache = super::HttpMetricsPathCache::new(super::HttpMetricsPathCacheMaximum::from(
            std::num::NonZeroUsize::MIN,
        ));
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(str_constants::ROOT))
                .0
                .as_ref(),
            str_constants::ROOT
        );
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(str_constants::ROOT))
                .0
                .as_ref(),
            str_constants::ROOT
        );
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(str_constants::V1))
                .0
                .as_ref(),
            str_constants::HTTP_METRICS_UNMATCHED_PATH
        );
    }

    #[test]
    fn invalid_path_does_not_consume_cache_capacity() {
        let cache = super::HttpMetricsPathCache::new(super::HttpMetricsPathCacheMaximum::from(
            std::num::NonZeroUsize::MIN,
        ));
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(str_constants::EMPTY))
                .0
                .as_ref(),
            str_constants::HTTP_METRICS_UNMATCHED_PATH
        );
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(str_constants::ROOT))
                .0
                .as_ref(),
            str_constants::ROOT
        );
    }

    #[tokio::test]
    async fn layer_handles_every_standard_and_custom_http_method() {
        let router = axum::Router::from(super::HttpMetricsLayer::default().apply(
            crate::AxumRouter::from(axum::Router::new().route(
                "/items/{id}",
                axum::routing::any(async || http::StatusCode::INTERNAL_SERVER_ERROR),
            )),
        ));
        let custom = http::Method::from_bytes(b"CUSTOM").expect(
            "6e90dca2 layer_handles_every_standard_and_custom_http_method invariant must hold",
        );
        let statuses = tokio::join!(
            call_method(router.clone(), http::Method::CONNECT),
            call_method(router.clone(), http::Method::DELETE),
            call_method(router.clone(), http::Method::GET),
            call_method(router.clone(), http::Method::HEAD),
            call_method(router.clone(), http::Method::OPTIONS),
            call_method(router.clone(), http::Method::PATCH),
            call_method(router.clone(), http::Method::POST),
            call_method(router.clone(), http::Method::PUT),
            call_method(router.clone(), http::Method::TRACE),
            call_method(router, custom),
        );
        assert_eq!(
            statuses,
            (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        );
    }
}
