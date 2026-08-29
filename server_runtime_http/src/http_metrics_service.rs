#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct HttpMetricsService<Service> {
    pub(super) inner: Service,
    pub(super) paths: crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc,
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
            http::Method::CONNECT => constants_str::catalog::HTTP_METHOD_CONNECT_LABEL,
            http::Method::DELETE => constants_str::integration_fixtures::DELETE,
            http::Method::GET => constants_str::catalog::GET,
            http::Method::HEAD => constants_str::catalog::HTTP_METHOD_HEAD_LABEL,
            http::Method::OPTIONS => constants_str::catalog::HTTP_METHOD_OPTIONS_LABEL,
            http::Method::PATCH => constants_str::catalog::PATCH,
            http::Method::POST => constants_str::catalog::POST,
            http::Method::PUT => constants_str::catalog::HTTP_METHOD_PUT_LABEL,
            http::Method::TRACE => constants_str::catalog::HTTP_METHOD_TRACE_LABEL,
            _ => constants_str::catalog::HTTP_METHOD_OTHER_LABEL,
        };
        let normalized_path = req
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .is_none()
            .then(|| {
                crate::normalize_identifier_path::normalize_identifier_path(
                    crate::http_request_path_ref::HttpRequestPathRef::from(req.uri().path()),
                )
            })
            .flatten();
        let path_text = req
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .map(axum::extract::MatchedPath::as_str)
            .or_else(|| normalized_path.as_ref().map(AsRef::as_ref))
            .unwrap_or(constants_str::catalog::HTTP_METRICS_UNMATCHED_PATH);
        let path_label = self
            .paths
            .0
            .label(crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(path_text));
        let started_at = std::time::Instant::now();
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(async move {
            let response = response_future.await?;
            let status = crate::metrics_shared_string::MetricsSharedString::from(
                metrics::SharedString::from(response.status().as_str().to_owned()),
            );
            let labels = [
                metrics::Label::new(constants_str::catalog::HTTP_METRICS_LABEL_METHOD, method),
                metrics::Label::new(constants_str::catalog::PATH_ALT_5, path_label.0),
                metrics::Label::new(constants_str::catalog::STATUS_ALT, status.0),
            ];
            metrics::counter!(
                constants_str::catalog::HTTP_METRICS_REQUESTS_TOTAL,
                labels.iter()
            )
            .increment(constants_u64::ONE);
            metrics::histogram!(
                constants_str::catalog::HTTP_METRICS_REQUEST_DURATION_SECONDS,
                labels.iter()
            )
            .record(started_at.elapsed().as_secs_f64());
            if response.status().is_server_error() {
                metrics::counter!(
                    constants_str::catalog::HTTP_METRICS_ERRORS_TOTAL,
                    labels.iter()
                )
                .increment(constants_u64::ONE);
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
