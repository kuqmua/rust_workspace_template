pub mod domain_types;
mod http_runtime_test_status;
mod reqwest_runtime_test_client;
mod reqwest_runtime_test_response;
mod runtime_test_config;
mod runtime_test_error;
mod runtime_test_kind;
mod runtime_test_report;
mod runtime_test_url;
mod service_base_url;
mod service_base_url_error;

pub fn local_config() -> Result<domain_types::RuntimeTestConfig, domain_types::ServiceBaseUrlError>
{
    Ok(domain_types::RuntimeTestConfig::new(
        domain_types::ServiceBaseUrl::try_from(String::from(constants_str::VALUE_D30A576C))?,
        domain_types::ServiceBaseUrl::try_from(String::from(constants_str::VALUE_08D5F409))?,
    ))
}

pub fn run(
    config: &domain_types::RuntimeTestConfig,
) -> Result<domain_types::RuntimeTestReport, domain_types::RuntimeTestError> {
    let client = domain_types::ReqwestRuntimeTestClient::from(
        reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(server_runtime_http::domain_types::ReqwestError::from)
            .map_err(domain_types::RuntimeTestError::Client)?,
    );
    let mut passed = Vec::with_capacity(5usize);

    run_health_test(
        &client,
        config.application_base_url(),
        common_routes::domain_types::CommonRoute::HealthLive,
        domain_types::RuntimeTestKind::ApplicationLiveness,
    )?;
    passed.push(domain_types::RuntimeTestKind::ApplicationLiveness);
    run_health_test(
        &client,
        config.application_base_url(),
        common_routes::domain_types::CommonRoute::HealthReady,
        domain_types::RuntimeTestKind::ApplicationReadiness,
    )?;
    passed.push(domain_types::RuntimeTestKind::ApplicationReadiness);
    run_health_test(
        &client,
        config.notification_service_base_url(),
        common_routes::domain_types::CommonRoute::HealthLive,
        domain_types::RuntimeTestKind::NotificationServiceLiveness,
    )?;
    passed.push(domain_types::RuntimeTestKind::NotificationServiceLiveness);
    run_health_test(
        &client,
        config.notification_service_base_url(),
        common_routes::domain_types::CommonRoute::HealthReady,
        domain_types::RuntimeTestKind::NotificationServiceReadiness,
    )?;
    passed.push(domain_types::RuntimeTestKind::NotificationServiceReadiness);
    let test = domain_types::RuntimeTestKind::NotificationCreation;
    let message = notification_service_contract::domain_types::NotificationMessage::try_from(
        String::from(constants_str::VALUE_FB13A725),
    )
    .map_err(domain_types::RuntimeTestError::NotificationMessage)?;
    let request = notification_service_contract::domain_types::CreateNotificationReq::new(message);
    let route = notification_service_contract::domain_types::NotificationRoute::Create.contract();
    let url = route_url(config.notification_service_base_url(), route.path())?;
    let response = client
        .send_notification(&url, &request)
        .map_err(|source| domain_types::RuntimeTestError::Request { test, source })?;
    let expected = domain_types::HttpRuntimeTestStatus::from(u16::from(
        route.success_status().transport_status(),
    ));
    require_status(test, &response, expected)?;
    let _created = response
        .into_notification_res()
        .map_err(|source| domain_types::RuntimeTestError::Response { test, source })?;
    passed.push(domain_types::RuntimeTestKind::NotificationCreation);

    Ok(domain_types::RuntimeTestReport::from(
        bounded_types::domain_types::vector::BoundedVec::try_from(passed)
            .map_err(domain_types::RuntimeTestError::Report)?,
    ))
}

fn run_health_test(
    client: &domain_types::ReqwestRuntimeTestClient,
    base_url: &domain_types::ServiceBaseUrl,
    route: common_routes::domain_types::CommonRoute,
    test: domain_types::RuntimeTestKind,
) -> Result<(), domain_types::RuntimeTestError> {
    let url = route_url(base_url, route.path())?;
    let response = client
        .send_get(&url)
        .map_err(|source| domain_types::RuntimeTestError::Request { test, source })?;
    require_status(
        test,
        &response,
        domain_types::HttpRuntimeTestStatus::from(200u16),
    )?;
    let report = response
        .into_health_report()
        .map_err(|source| domain_types::RuntimeTestError::Response { test, source })?;
    if report.status() != common_routes::domain_types::HealthStatus::Ok {
        return Err(domain_types::RuntimeTestError::Unhealthy { test });
    }
    Ok(())
}

fn require_status(
    test: domain_types::RuntimeTestKind,
    response: &domain_types::ReqwestRuntimeTestResponse,
    expected: domain_types::HttpRuntimeTestStatus,
) -> Result<(), domain_types::RuntimeTestError> {
    let actual = response.status();
    if actual == expected {
        Ok(())
    } else {
        Err(domain_types::RuntimeTestError::Status {
            test,
            actual,
            expected,
        })
    }
}

fn route_url(
    base_url: &domain_types::ServiceBaseUrl,
    path: frontend_contract::domain_types::ContractStr,
) -> Result<domain_types::RuntimeTestUrl, domain_types::ServiceBaseUrlError> {
    domain_types::RuntimeTestUrl::try_from(format!("{}{path}", base_url.as_ref()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn route_url_uses_contract_path() {
        let base_url = super::domain_types::ServiceBaseUrl::try_from(String::from(
            constants_str::VALUE_FF79C6DD,
        ))
        .expect("6cde5062 route_url_uses_contract_path invariant must hold");
        assert_eq!(
            super::route_url(
                &base_url,
                common_routes::domain_types::CommonRoute::HealthLive.path()
            )
            .expect("ea911c48 route_url_uses_contract_path invariant must hold")
            .as_ref(),
            "http://application/health/live"
        );
    }
}
