#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct RequestTimeoutLayer(super::RequestTimeoutDuration);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
enum RequestTimeoutError {
    #[error("request timeout")]
    TimedOut,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, serde::Serialize)]
#[serde(transparent)]
#[derive(newtype::FromInner)]
struct StdRequestTimeoutMessage(&'static str);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize)]
struct RequestTimeoutBody {
    error: StdRequestTimeoutMessage,
}

impl axum::response::IntoResponse for RequestTimeoutError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::TimedOut => axum::response::IntoResponse::into_response((
                http::StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(RequestTimeoutBody {
                    error: StdRequestTimeoutMessage::from(constants_str::REQUEST_TIMEOUT),
                }),
            )),
        }
    }
}

impl RequestTimeoutLayer {
    #[must_use]
    pub fn apply(self, router: super::AxumRouter) -> super::AxumRouter {
        super::AxumRouter::from(router.0.layer(RequestTimeoutTowerLayer::from(self.0)))
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
struct RequestTimeoutTowerLayer(super::RequestTimeoutDuration);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
struct RequestTimeoutService<Service> {
    inner: Service,
    timeout: super::RequestTimeoutDuration,
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
            match tokio::time::timeout(timeout.get(), response_future).await {
                Ok(response) => response,
                Err(_elapsed) => {
                    let mut response =
                        axum::response::IntoResponse::into_response(RequestTimeoutError::TimedOut);
                    let _previous = response.headers_mut().insert(
                        http::header::RETRY_AFTER,
                        http::HeaderValue::from(timeout.get().as_secs().max(1u64)),
                    );
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

#[cfg(test)]
mod tests {
    #[test]
    fn timeout_layer_preserves_validated_timeout() {
        let timeout =
            super::super::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64))
                .expect("65a8fd30 timeout_layer_preserves_validated_timeout invariant must hold");
        let layer = super::RequestTimeoutLayer::from(timeout);
        assert_eq!(layer.0.get(), std::time::Duration::from_secs(1u64));
    }

    #[tokio::test(start_paused = true)]
    async fn timeout_response_contains_retry_after_without_text_round_trip() {
        let timeout =
            super::super::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(2u64))
                .expect("b140ead4 timeout_response_contains_retry_after_without_text_round_trip invariant must hold");
        let router = axum::Router::from(super::RequestTimeoutLayer::from(timeout).apply(
            super::super::AxumRouter::from(axum::Router::new().route(
                constants_str::VALUE_971BB40E,
                axum::routing::get(async || std::future::pending::<http::StatusCode>().await),
            )),
        ));
        let response = tower::ServiceExt::oneshot(
            router,
            http::Request::builder()
                .uri(constants_str::VALUE_971BB40E)
                .body(axum::body::Body::empty())
                .expect("9a076c51 timeout_response_contains_retry_after_without_text_round_trip invariant must hold"),
        )
        .await
        .expect("57912096 timeout_response_contains_retry_after_without_text_round_trip invariant must hold");
        assert_eq!(response.status(), http::StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            response.headers().get(http::header::RETRY_AFTER),
            Some(&http::HeaderValue::from_static("2"))
        );
    }
}
