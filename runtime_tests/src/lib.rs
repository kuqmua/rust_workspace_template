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
        if value.len() > usize_constants::VALUE_8_192 {
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

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct RuntimeTestReport {
    passed: bounded_types::BoundedVec<RuntimeTestKind, { usize_constants::ZERO }, 5usize>,
}

impl RuntimeTestReport {
    #[must_use]
    pub const fn passed(&self) -> &[RuntimeTestKind] {
        self.passed.as_slice()
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
struct ReqwestRuntimeTestClient(reqwest::blocking::Client);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
struct ReqwestRuntimeTestResponse(reqwest::blocking::Response);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
struct RuntimeTestUrl(String);

impl TryFrom<String> for RuntimeTestUrl {
    type Error = ServiceBaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > usize_constants::VALUE_8_192 {
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
    Client(#[source] server_runtime_http::ReqwestError),
    #[error("runtime notification test message is invalid: {0}")]
    NotificationMessage(
        #[source] notification_service_contract::NotificationMessageTryFromStringError,
    ),
    #[error("runtime test report exceeded its result capacity: {0}")]
    Report(#[source] bounded_types::BoundedValueError),
    #[error("{test} request failed: {source}")]
    Request {
        #[source]
        source: server_runtime_http::ReqwestError,
        test: RuntimeTestKind,
    },
    #[error("{test} response could not be decoded: {source}")]
    Response {
        #[source]
        source: server_runtime_http::ReqwestError,
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

pub fn local_config() -> Result<RuntimeTestConfig, ServiceBaseUrlError> {
    Ok(RuntimeTestConfig::new(
        ServiceBaseUrl::try_from(String::from("http://127.0.0.1:8080"))?,
        ServiceBaseUrl::try_from(String::from("http://127.0.0.1:8081"))?,
    ))
}

pub fn run(config: &RuntimeTestConfig) -> Result<RuntimeTestReport, RuntimeTestError> {
    let client = ReqwestRuntimeTestClient::from(
        reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(server_runtime_http::ReqwestError::from)
            .map_err(RuntimeTestError::Client)?,
    );
    let mut passed = Vec::with_capacity(5usize);

    run_health_test(
        &client,
        config.application_base_url(),
        common_routes::CommonRoute::HealthLive,
        RuntimeTestKind::ApplicationLiveness,
    )?;
    passed.push(RuntimeTestKind::ApplicationLiveness);
    run_health_test(
        &client,
        config.application_base_url(),
        common_routes::CommonRoute::HealthReady,
        RuntimeTestKind::ApplicationReadiness,
    )?;
    passed.push(RuntimeTestKind::ApplicationReadiness);
    run_health_test(
        &client,
        config.notification_service_base_url(),
        common_routes::CommonRoute::HealthLive,
        RuntimeTestKind::NotificationServiceLiveness,
    )?;
    passed.push(RuntimeTestKind::NotificationServiceLiveness);
    run_health_test(
        &client,
        config.notification_service_base_url(),
        common_routes::CommonRoute::HealthReady,
        RuntimeTestKind::NotificationServiceReadiness,
    )?;
    passed.push(RuntimeTestKind::NotificationServiceReadiness);
    let test = RuntimeTestKind::NotificationCreation;
    let message =
        notification_service_contract::NotificationMessage::try_from(String::from("runtime-test"))
            .map_err(RuntimeTestError::NotificationMessage)?;
    let request = notification_service_contract::CreateNotificationReq::new(message);
    let route = notification_service_contract::NotificationRoute::Create.contract();
    let response = ReqwestRuntimeTestResponse::from(
        client
            .0
            .post(route_url(config.notification_service_base_url(), route.path())?.as_ref())
            .json(&request)
            .send()
            .map_err(server_runtime_http::ReqwestError::from)
            .map_err(|source| RuntimeTestError::Request { test, source })?,
    );
    let expected =
        HttpRuntimeTestStatus::from(u16::from(route.success_status().transport_status()));
    require_status(test, &response, expected)?;
    let _created = response
        .0
        .json::<notification_service_contract::CreateNotificationRes>()
        .map_err(server_runtime_http::ReqwestError::from)
        .map_err(|source| RuntimeTestError::Response { test, source })?;
    passed.push(RuntimeTestKind::NotificationCreation);

    Ok(RuntimeTestReport {
        passed: bounded_types::BoundedVec::try_from(passed).map_err(RuntimeTestError::Report)?,
    })
}

fn run_health_test(
    client: &ReqwestRuntimeTestClient,
    base_url: &ServiceBaseUrl,
    route: common_routes::CommonRoute,
    test: RuntimeTestKind,
) -> Result<(), RuntimeTestError> {
    let response = ReqwestRuntimeTestResponse::from(
        client
            .0
            .get(route_url(base_url, route.path())?.as_ref())
            .send()
            .map_err(server_runtime_http::ReqwestError::from)
            .map_err(|source| RuntimeTestError::Request { test, source })?,
    );
    require_status(test, &response, HttpRuntimeTestStatus::from(200u16))?;
    let report = response
        .0
        .json::<common_routes::HealthReport>()
        .map_err(server_runtime_http::ReqwestError::from)
        .map_err(|source| RuntimeTestError::Response { test, source })?;
    if report.status() != common_routes::HealthStatus::Ok {
        return Err(RuntimeTestError::Unhealthy { test });
    }
    Ok(())
}

fn require_status(
    test: RuntimeTestKind,
    response: &ReqwestRuntimeTestResponse,
    expected: HttpRuntimeTestStatus,
) -> Result<(), RuntimeTestError> {
    let actual = HttpRuntimeTestStatus::from(response.0.status().as_u16());
    if actual == expected {
        Ok(())
    } else {
        Err(RuntimeTestError::Status {
            test,
            actual,
            expected,
        })
    }
}

fn route_url(
    base_url: &ServiceBaseUrl,
    path: frontend_contract::ContractStr,
) -> Result<RuntimeTestUrl, ServiceBaseUrlError> {
    RuntimeTestUrl::try_from(format!("{}{path}", base_url.as_ref()))
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

    #[test]
    fn route_url_uses_contract_path() {
        let base_url = super::ServiceBaseUrl::try_from(String::from("http://application"))
            .expect("6cde5062 route_url_uses_contract_path invariant must hold");
        assert_eq!(
            super::route_url(&base_url, common_routes::CommonRoute::HealthLive.path())
                .expect("ea911c48 route_url_uses_contract_path invariant must hold"),
            super::RuntimeTestUrl(String::from("http://application/health/live"))
        );
    }
}
