#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForwardedProtoTrust {
    Ignore,
    Trust,
}

#[derive(Clone, Debug, newtype::DerefInner)]
pub struct HttpContentSecurityPolicy(http::HeaderValue);

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("content security policy is not a valid HTTP header value")]
pub struct HttpContentSecurityPolicyError;

impl TryFrom<String> for HttpContentSecurityPolicy {
    type Error = HttpContentSecurityPolicyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 4096usize {
            return Err(HttpContentSecurityPolicyError);
        }
        http::HeaderValue::try_from(value)
            .map(Self)
            .map_err(|_error| HttpContentSecurityPolicyError)
    }
}

#[derive(Clone, Debug)]
pub struct SecurityHeadersLayer {
    content_security_policy: Option<HttpContentSecurityPolicy>,
    forwarded_proto_trust: ForwardedProtoTrust,
}

impl From<ForwardedProtoTrust> for SecurityHeadersLayer {
    fn from(value: ForwardedProtoTrust) -> Self {
        Self {
            content_security_policy: None,
            forwarded_proto_trust: value,
        }
    }
}

impl SecurityHeadersLayer {
    #[must_use]
    pub fn apply(self, router: super::AxumRouter) -> super::AxumRouter {
        super::AxumRouter::from(router.0.layer(SecurityHeadersTowerLayer {
            content_security_policy: self.content_security_policy,
            forwarded_proto_trust: self.forwarded_proto_trust,
        }))
    }

    #[must_use]
    pub fn with_content_security_policy(mut self, value: HttpContentSecurityPolicy) -> Self {
        self.content_security_policy = Some(value);
        self
    }
}

#[derive(Clone, Debug)]
struct SecurityHeadersTowerLayer {
    content_security_policy: Option<HttpContentSecurityPolicy>,
    forwarded_proto_trust: ForwardedProtoTrust,
}

#[derive(Clone, Debug)]
struct SecurityHeadersService<Service> {
    content_security_policy: Option<HttpContentSecurityPolicy>,
    forwarded_proto_trust: ForwardedProtoTrust,
    inner: Service,
}

impl<Service> tower::Layer<Service> for SecurityHeadersTowerLayer {
    type Service = SecurityHeadersService<Service>;

    fn layer(&self, inner: Service) -> Self::Service {
        SecurityHeadersService {
            content_security_policy: self.content_security_policy.clone(),
            forwarded_proto_trust: self.forwarded_proto_trust,
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

    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        let is_api_path = req.uri().path().starts_with(str_constants::V1_SLASH);
        let is_forwarded_https = matches!(self.forwarded_proto_trust, ForwardedProtoTrust::Trust)
            && req
                .headers()
                .get(str_constants::X_FORWARDED_PROTO)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| {
                    value.split(',').next().is_some_and(|first| {
                        first.trim().eq_ignore_ascii_case(str_constants::HTTPS)
                    })
                });
        req.headers_mut().iter_mut().for_each(|(name, value)| {
            if name == http::header::AUTHORIZATION
                || name == http::header::COOKIE
                || name.as_str() == str_constants::X_CSRF_TOKEN_ALT
            {
                value.set_sensitive(true);
            }
        });
        let content_security_policy = self.content_security_policy.clone();
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(async move {
            let mut response = response_future.await?;
            let _content_type_options = response.headers_mut().insert(
                http::HeaderName::from_static(str_constants::X_CONTENT_TYPE_OPTIONS),
                http::HeaderValue::from_static(str_constants::NOSNIFF),
            );
            let _frame_options = response.headers_mut().insert(
                http::HeaderName::from_static(str_constants::X_FRAME_OPTIONS),
                http::HeaderValue::from_static(str_constants::DENY),
            );
            let _referrer_policy = response.headers_mut().insert(
                http::HeaderName::from_static(str_constants::REFERRER_POLICY),
                http::HeaderValue::from_static(str_constants::SAME_ORIGIN),
            );
            if let Some(resolved_content_security_policy) = content_security_policy {
                let _previous_content_security_policy = response.headers_mut().insert(
                    http::HeaderName::from_static(str_constants::CONTENT_SECURITY_POLICY_HEADER),
                    resolved_content_security_policy.0,
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
                    http::HeaderValue::from_static(str_constants::NO_STORE),
                );
            }
            if is_forwarded_https {
                let _strict_transport_security = response.headers_mut().insert(
                    http::HeaderName::from_static(str_constants::STRICT_TRANSPORT_SECURITY),
                    http::HeaderValue::from_static(
                        str_constants::MAX_AGE_31536000_INCLUDESUBDOMAINS,
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

#[cfg(test)]
mod tests {
    #[test]
    fn content_security_policy_rejects_header_injection() {
        let _error = super::HttpContentSecurityPolicy::try_from(
            "default-src 'self'\r\ninvalid: value".to_owned(),
        )
        .expect_err("94d8f601");
    }
}
