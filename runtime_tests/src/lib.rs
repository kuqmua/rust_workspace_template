pub mod http_runtime_test_status;
pub mod reqwest_runtime_test_client;
pub mod reqwest_runtime_test_response;
pub mod runtime_test_config;
pub mod runtime_test_error;
pub mod runtime_test_kind;
pub mod runtime_test_report;
pub mod runtime_test_url;
pub mod service_base_url;
pub mod service_base_url_error;
#[cfg(test)]
pub mod tests_domain_types;

pub fn local_config()
-> Result<runtime_test_config::RuntimeTestConfig, service_base_url_error::ServiceBaseUrlError> {
    Ok(runtime_test_config::RuntimeTestConfig::new(
        service_base_url::ServiceBaseUrl::try_from(String::from(constants_str::VALUE_D30A576C))?,
        service_base_url::ServiceBaseUrl::try_from(String::from(constants_str::VALUE_08D5F409))?,
    ))
}

pub fn run(
    config: &runtime_test_config::RuntimeTestConfig,
) -> Result<runtime_test_report::RuntimeTestReport, runtime_test_error::RuntimeTestError> {
    let client = reqwest_runtime_test_client::ReqwestRuntimeTestClient::from(
        reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(server_runtime_http::reqwest_error::ReqwestError::from)
            .map_err(runtime_test_error::RuntimeTestError::Client)?,
    );
    let mut passed = Vec::with_capacity(5usize);

    run_health_test(
        &client,
        config.application_base_url(),
        common_routes::common_route::CommonRoute::HealthLive,
        runtime_test_kind::RuntimeTestKind::ApplicationLiveness,
    )?;
    passed.push(runtime_test_kind::RuntimeTestKind::ApplicationLiveness);
    run_health_test(
        &client,
        config.application_base_url(),
        common_routes::common_route::CommonRoute::HealthReady,
        runtime_test_kind::RuntimeTestKind::ApplicationReadiness,
    )?;
    passed.push(runtime_test_kind::RuntimeTestKind::ApplicationReadiness);
    run_health_test(
        &client,
        config.notification_service_base_url(),
        common_routes::common_route::CommonRoute::HealthLive,
        runtime_test_kind::RuntimeTestKind::NotificationServiceLiveness,
    )?;
    passed.push(runtime_test_kind::RuntimeTestKind::NotificationServiceLiveness);
    run_health_test(
        &client,
        config.notification_service_base_url(),
        common_routes::common_route::CommonRoute::HealthReady,
        runtime_test_kind::RuntimeTestKind::NotificationServiceReadiness,
    )?;
    passed.push(runtime_test_kind::RuntimeTestKind::NotificationServiceReadiness);
    let test = runtime_test_kind::RuntimeTestKind::NotificationCreation;
    let message =
        notification_service_contract::notification_message::NotificationMessage::try_from(
            String::from(constants_str::VALUE_FB13A725),
        )
        .map_err(runtime_test_error::RuntimeTestError::NotificationMessage)?;
    let request =
        notification_service_contract::create_notification_req::CreateNotificationReq::new(message);
    let route =
        notification_service_contract::notification_route::NotificationRoute::Create.contract();
    let url = route_url(config.notification_service_base_url(), route.path())?;
    let response = client
        .send_notification(&url, &request)
        .map_err(|source| runtime_test_error::RuntimeTestError::Request { test, source })?;
    let expected = http_runtime_test_status::HttpRuntimeTestStatus::from(u16::from(
        route.success_status().transport_status(),
    ));
    require_status(test, &response, expected)?;
    let _created = response
        .into_notification_res()
        .map_err(|source| runtime_test_error::RuntimeTestError::Response { test, source })?;
    passed.push(runtime_test_kind::RuntimeTestKind::NotificationCreation);

    Ok(runtime_test_report::RuntimeTestReport::from(
        bounded_types::bounded_vec::BoundedVec::try_from(passed)
            .map_err(runtime_test_error::RuntimeTestError::Report)?,
    ))
}

fn run_health_test(
    client: &reqwest_runtime_test_client::ReqwestRuntimeTestClient,
    base_url: &service_base_url::ServiceBaseUrl,
    route: common_routes::common_route::CommonRoute,
    test: runtime_test_kind::RuntimeTestKind,
) -> Result<(), runtime_test_error::RuntimeTestError> {
    let url = route_url(base_url, route.path())?;
    let response = client
        .send_get(&url)
        .map_err(|source| runtime_test_error::RuntimeTestError::Request { test, source })?;
    require_status(
        test,
        &response,
        http_runtime_test_status::HttpRuntimeTestStatus::from(200u16),
    )?;
    let report = response
        .into_health_report()
        .map_err(|source| runtime_test_error::RuntimeTestError::Response { test, source })?;
    if report.status() != common_routes::health_status::HealthStatus::Ok {
        return Err(runtime_test_error::RuntimeTestError::Unhealthy { test });
    }
    Ok(())
}

fn require_status(
    test: runtime_test_kind::RuntimeTestKind,
    response: &reqwest_runtime_test_response::ReqwestRuntimeTestResponse,
    expected: http_runtime_test_status::HttpRuntimeTestStatus,
) -> Result<(), runtime_test_error::RuntimeTestError> {
    let actual = response.status();
    if actual == expected {
        Ok(())
    } else {
        Err(runtime_test_error::RuntimeTestError::Status {
            test,
            actual,
            expected,
        })
    }
}

fn route_url(
    base_url: &service_base_url::ServiceBaseUrl,
    path: frontend_contract::contract_str::ContractStr,
) -> Result<runtime_test_url::RuntimeTestUrl, service_base_url_error::ServiceBaseUrlError> {
    runtime_test_url::RuntimeTestUrl::try_from(format!("{}{path}", base_url.as_ref()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_route_url_uses_contract_path() {
        let base_url = crate::service_base_url::ServiceBaseUrl::try_from(String::from(
            constants_str::VALUE_FF79C6DD,
        ))
        .expect("6cde5062 route_url_uses_contract_path invariant must hold");
        assert_eq!(
            super::route_url(
                &base_url,
                common_routes::common_route::CommonRoute::HealthLive.path()
            )
            .expect("ea911c48 route_url_uses_contract_path invariant must hold")
            .as_ref(),
            "http://application/health/live"
        );
    }
}
