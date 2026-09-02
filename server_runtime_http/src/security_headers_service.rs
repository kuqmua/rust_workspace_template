#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
#[constructor(pub(crate))]
pub(super) struct SecurityHeadersService<Service> {
    content_security_policy: Option<crate::http_content_security_policy::HttpContentSecurityPolicy>,
    forwarded_proto_trust: crate::forwarded_proto_trust::ForwardedProtoTrust,
    inner: Service,
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

    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        let is_api_path = req.uri().path().starts_with(constants_str::V1_SLASH);
        let is_forwarded_https = matches!(
            self.forwarded_proto_trust,
            crate::forwarded_proto_trust::ForwardedProtoTrust::Trust
        ) && req
            .headers()
            .get(constants_str::X_FORWARDED_PROTO)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| {
                value
                    .split(',')
                    .next()
                    .is_some_and(|first| first.trim().eq_ignore_ascii_case(constants_str::HTTPS))
            });
        req.headers_mut().iter_mut().for_each(|(name, value)| {
            if name == http::header::AUTHORIZATION
                || name == http::header::COOKIE
                || name.as_str() == constants_str::X_CSRF_TOKEN_ALT
            {
                value.set_sensitive(true);
            }
        });
        let content_security_policy = self.content_security_policy.clone();
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(async move {
            let mut response = response_future.await?;
            let _content_type_options = response.headers_mut().insert(
                http::HeaderName::from_static(constants_str::X_CONTENT_TYPE_OPTIONS),
                http::HeaderValue::from_static(constants_str::NOSNIFF),
            );
            let _frame_options = response.headers_mut().insert(
                http::HeaderName::from_static(constants_str::X_FRAME_OPTIONS),
                http::HeaderValue::from_static(constants_str::DENY),
            );
            let _referrer_policy = response.headers_mut().insert(
                http::HeaderName::from_static(constants_str::REFERRER_POLICY),
                http::HeaderValue::from_static(constants_str::SAME_ORIGIN),
            );
            if let Some(resolved_content_security_policy) = content_security_policy {
                let _previous_content_security_policy = response.headers_mut().insert(
                    http::HeaderName::from_static(constants_str::CONTENT_SECURITY_POLICY_HEADER),
                    resolved_content_security_policy.into_inner(),
                );
            }
            response.headers_mut().iter_mut().for_each(|(name, value)| {
                if name == http::header::SET_COOKIE {
                    value.set_sensitive(true);
                }
            });
            if is_api_path {
                let _cache_control = response.headers_mut().insert(
                    http::header::CACHE_CONTROL,
                    http::HeaderValue::from_static(constants_str::NO_STORE),
                );
            }
            if is_forwarded_https {
                let _strict_transport_security = response.headers_mut().insert(
                    http::HeaderName::from_static(constants_str::STRICT_TRANSPORT_SECURITY),
                    http::HeaderValue::from_static(
                        constants_str::MAX_AGE_31536000_INCLUDESUBDOMAINS,
                    ),
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
