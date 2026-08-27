#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub(super) struct RequestTimeoutService<Service> {
    pub(super) inner: Service,
    pub(super) timeout: super::super::RequestTimeoutDuration,
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
                    let mut response = axum::response::IntoResponse::into_response(
                        super::RequestTimeoutError::TimedOut,
                    );
                    let _previous = response.headers_mut().insert(
                        http::header::RETRY_AFTER,
                        http::HeaderValue::from(timeout.get().as_secs().max(constants_u64::ONE)),
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
