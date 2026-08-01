#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct RequestTimeoutLayer(super::StdRequestTimeout);

#[derive(Debug, thiserror::Error)]
enum RequestTimeoutError {
    #[error("request timeout")]
    TimedOut,
}

#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(transparent)]
#[derive(newtype::FromInner)]
struct StdRequestTimeoutMessage(&'static str);

#[derive(Debug, serde::Serialize)]
struct RequestTimeoutBody {
    error: StdRequestTimeoutMessage,
}

impl axum::response::IntoResponse for RequestTimeoutError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::TimedOut => axum::response::IntoResponse::into_response((
                http::StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(RequestTimeoutBody {
                    error: StdRequestTimeoutMessage::from(str_constants::REQUEST_TIMEOUT),
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

#[derive(Clone, Copy, Debug, newtype::FromInner)]
struct RequestTimeoutTowerLayer(super::StdRequestTimeout);

#[derive(Clone, Debug)]
struct RequestTimeoutService<Service> {
    inner: Service,
    timeout: super::StdRequestTimeout,
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
                    let retry_after = timeout.get().as_secs().max(1u64).to_string();
                    let mut response =
                        axum::response::IntoResponse::into_response(RequestTimeoutError::TimedOut);
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

#[cfg(test)]
mod tests {
    #[test]
    fn timeout_layer_preserves_validated_timeout() {
        let timeout =
            super::super::StdRequestTimeout::try_from(std::time::Duration::from_secs(1u64))
                .expect("65a8fd30");
        let layer = super::RequestTimeoutLayer::from(timeout);
        assert_eq!(layer.0.get(), std::time::Duration::from_secs(1u64));
    }
}
