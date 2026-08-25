#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct ServiceBaseUrl(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum ServiceBaseUrlError {
    #[error("service base URL must include a host")]
    Host,
    #[error("service base URL exceeds its maximum length")]
    Length,
    #[error("service base URL must use HTTP or HTTPS")]
    Scheme,
    #[error("service base URL must not include a query or fragment")]
    Suffix,
}

impl TryFrom<String> for ServiceBaseUrl {
    type Error = ServiceBaseUrlError;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            return Err(ServiceBaseUrlError::Length);
        }
        while value.ends_with('/') {
            let _removed = value.pop();
        }
        let parsed = match reqwest::Url::parse(value.as_str()) {
            Ok(parsed) => parsed,
            Err(_error) if value.starts_with("http://") || value.starts_with("https://") => {
                return Err(ServiceBaseUrlError::Host);
            }
            Err(_error) => return Err(ServiceBaseUrlError::Scheme),
        };
        if parsed.scheme() != "http" && parsed.scheme() != "https" {
            return Err(ServiceBaseUrlError::Scheme);
        }
        if parsed.host().is_none() {
            return Err(ServiceBaseUrlError::Host);
        }
        if parsed.query().is_some() || parsed.fragment().is_some() {
            return Err(ServiceBaseUrlError::Suffix);
        }
        Ok(Self(value))
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeTestConfig {
    application_base_url: ServiceBaseUrl,
    notification_service_base_url: ServiceBaseUrl,
}

impl RuntimeTestConfig {
    #[must_use]
    pub const fn application_base_url(&self) -> &ServiceBaseUrl {
        &self.application_base_url
    }

    #[must_use]
    pub const fn new(
        application_base_url: ServiceBaseUrl,
        notification_service_base_url: ServiceBaseUrl,
    ) -> Self {
        Self {
            application_base_url,
            notification_service_base_url,
        }
    }

    #[must_use]
    pub const fn notification_service_base_url(&self) -> &ServiceBaseUrl {
        &self.notification_service_base_url
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeTestKind {
    ApplicationLiveness,
    ApplicationReadiness,
    NotificationCreation,
    NotificationServiceLiveness,
    NotificationServiceReadiness,
}

impl std::fmt::Display for RuntimeTestKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ApplicationLiveness => "application liveness",
            Self::ApplicationReadiness => "application readiness",
            Self::NotificationCreation => "notification creation",
            Self::NotificationServiceLiveness => "notification service liveness",
            Self::NotificationServiceReadiness => "notification service readiness",
        })
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::FromInner,
)]
pub struct RuntimeTestReport(
    bounded_types::domain_types::vector::BoundedVec<
        RuntimeTestKind,
        { constants_usize::ZERO },
        5usize,
    >,
);

impl RuntimeTestReport {
    #[must_use]
    pub const fn passed(&self) -> &[RuntimeTestKind] {
        self.0.as_slice()
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct HttpRuntimeTestStatus(u16);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct ReqwestRuntimeTestClient(reqwest::blocking::Client);

impl ReqwestRuntimeTestClient {
    #[allow(
        clippy::single_call_fn,
        reason = "the runtime adapter keeps the external HTTP client behind a domain boundary"
    )]
    pub(crate) fn send_get(
        &self,
        url: &RuntimeTestUrl,
    ) -> Result<ReqwestRuntimeTestResponse, server_runtime_http::domain_types::ReqwestError> {
        self.0
            .get(url.0.as_str())
            .send()
            .map(ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }

    #[allow(
        clippy::single_call_fn,
        reason = "the runtime adapter keeps the external HTTP client behind a domain boundary"
    )]
    pub(crate) fn send_notification(
        &self,
        url: &RuntimeTestUrl,
        request: &notification_service_contract::domain_types::CreateNotificationReq,
    ) -> Result<ReqwestRuntimeTestResponse, server_runtime_http::domain_types::ReqwestError> {
        self.0
            .post(url.0.as_str())
            .json(request)
            .send()
            .map(ReqwestRuntimeTestResponse::from)
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct ReqwestRuntimeTestResponse(reqwest::blocking::Response);

impl ReqwestRuntimeTestResponse {
    #[allow(
        clippy::single_call_fn,
        reason = "the runtime adapter decodes the health contract without exposing the HTTP response"
    )]
    pub(crate) fn into_health_report(
        self,
    ) -> Result<
        common_routes::domain_types::HealthReport,
        server_runtime_http::domain_types::ReqwestError,
    > {
        self.0
            .json::<common_routes::domain_types::HealthReport>()
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }

    #[allow(
        clippy::single_call_fn,
        reason = "the runtime adapter decodes the notification contract without exposing the HTTP response"
    )]
    pub(crate) fn into_notification_res(
        self,
    ) -> Result<
        notification_service_contract::domain_types::CreateNotificationRes,
        server_runtime_http::domain_types::ReqwestError,
    > {
        self.0
            .json::<notification_service_contract::domain_types::CreateNotificationRes>()
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
    }

    #[must_use]
    pub(crate) fn status(&self) -> HttpRuntimeTestStatus {
        HttpRuntimeTestStatus::from(self.0.status().as_u16())
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub(crate) struct RuntimeTestUrl(String);

impl TryFrom<String> for RuntimeTestUrl {
    type Error = ServiceBaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(ServiceBaseUrlError::Length)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum RuntimeTestError {
    #[error("runtime service URL is invalid: {0}")]
    BaseUrl(#[from] ServiceBaseUrlError),
    #[error("runtime HTTP client could not be built: {0}")]
    Client(#[source] server_runtime_http::domain_types::ReqwestError),
    #[error("runtime notification test message is invalid: {0}")]
    NotificationMessage(
        #[source]
        notification_service_contract::domain_types::NotificationMessageTryFromStringError,
    ),
    #[error("runtime test report exceeded its result capacity: {0}")]
    Report(#[source] bounded_types::domain_types::BoundedValueError),
    #[error("{test} request failed: {source}")]
    Request {
        #[source]
        source: server_runtime_http::domain_types::ReqwestError,
        test: RuntimeTestKind,
    },
    #[error("{test} response could not be decoded: {source}")]
    Response {
        #[source]
        source: server_runtime_http::domain_types::ReqwestError,
        test: RuntimeTestKind,
    },
    #[error("{test} returned HTTP {actual}; expected {expected}")]
    Status {
        actual: HttpRuntimeTestStatus,
        expected: HttpRuntimeTestStatus,
        test: RuntimeTestKind,
    },
    #[error("{test} reported an unhealthy service")]
    Unhealthy { test: RuntimeTestKind },
}

#[cfg(test)]
mod tests {
    #[test]
    fn service_base_url_normalizes_trailing_slashes() {
        let base_url = super::ServiceBaseUrl::try_from(String::from("http://127.0.0.1:8080///"))
            .expect("087da3f2 service_base_url_normalizes_trailing_slashes invariant must hold");
        assert_eq!(base_url.as_ref(), "http://127.0.0.1:8080");
    }

    #[test]
    fn service_base_url_rejects_non_http_urls_and_suffixes() {
        assert_eq!(
            super::ServiceBaseUrl::try_from(String::from("postgres://database/service")),
            Err(super::ServiceBaseUrlError::Scheme)
        );
        assert_eq!(
            super::ServiceBaseUrl::try_from(String::from("http://service/path?query=true")),
            Err(super::ServiceBaseUrlError::Suffix)
        );
    }
}
