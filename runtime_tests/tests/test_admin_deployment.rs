#![allow(
    unused_crate_dependencies,
    reason = "this integration target uses the HTTP client and public contracts while other package dependencies serve the runtime binary"
)]

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "requires the owned local test_runtime_http_fixture.py server and SERVICE_SOCKET_ADDRESS"]
    fn test_runtime_failure_matrix_preserves_operation_status_and_sources() {
        let address_result = std::env::var(constants_str::ENV_NAMES_SERVICE_SOCKET_ADDRESS);
        assert!(address_result.is_ok());
        let Ok(address) = address_result else {
            return;
        };
        [
            runtime_tests::runtime_test_kind::RuntimeTestKind::ApplicationLiveness,
            runtime_tests::runtime_test_kind::RuntimeTestKind::ApplicationReadiness,
            runtime_tests::runtime_test_kind::RuntimeTestKind::NotificationServiceLiveness,
            runtime_tests::runtime_test_kind::RuntimeTestKind::NotificationServiceReadiness,
            runtime_tests::runtime_test_kind::RuntimeTestKind::NotificationCreation,
        ]
        .into_iter()
        .enumerate()
        .fold((), |(), (operation_index, runtime_test_kind)| {
            [stringify!(status), stringify!(response), stringify!(unhealthy), stringify!(request)]
                .into_iter()
                .filter(|failure| operation_index < 4usize || *failure != stringify!(unhealthy))
                .fold((), |(), failure| {
                    let urls = [stringify!(application), stringify!(notification)].map(|service| {
                        runtime_tests::service_base_url::ServiceBaseUrl::try_from(format!(
                            "{}{address}/{operation_index}/{failure}/{service}",
                            constants_str::VALUE_8C8DAC95
                        ))
                    });
                    assert!(urls.iter().all(Result::is_ok));
                    let [Ok(application_url), Ok(notification_url)] = urls else {
                        return;
                    };
                    let config = runtime_tests::runtime_test_config::RuntimeTestConfig::new(application_url, notification_url);
                    let result = runtime_tests::run(&config);
                    assert!(result.is_err_and(|error| {
                        let source_present = std::error::Error::source(&error).is_some();
                        match error {
                            runtime_tests::runtime_test_error::RuntimeTestError::Status { actual, expected, test } => {
                                let expected_status = if operation_index < 4usize { 200u16 } else {
                                    u16::from(notification_service_contract::notification_route::NotificationRoute::Create.contract().success_status().transport_status())
                                };
                                failure == stringify!(status) && test == runtime_test_kind && !source_present
                                    && actual == runtime_tests::http_runtime_test_status::HttpRuntimeTestStatus::from(503u16)
                                    && expected == runtime_tests::http_runtime_test_status::HttpRuntimeTestStatus::from(expected_status)
                            }
                            runtime_tests::runtime_test_error::RuntimeTestError::Response { test, .. } => failure == stringify!(response) && test == runtime_test_kind && source_present,
                            runtime_tests::runtime_test_error::RuntimeTestError::Request { test, .. } => failure == stringify!(request) && test == runtime_test_kind && source_present,
                            runtime_tests::runtime_test_error::RuntimeTestError::Unhealthy { test } => failure == stringify!(unhealthy) && test == runtime_test_kind && !source_present,
                            runtime_tests::runtime_test_error::RuntimeTestError::BaseUrl(_)
                            | runtime_tests::runtime_test_error::RuntimeTestError::Client(_)
                            | runtime_tests::runtime_test_error::RuntimeTestError::NotificationMessage(_)
                            | runtime_tests::runtime_test_error::RuntimeTestError::Report(_) => false,
                        }
                    }));
                });
        });
    }

    #[test]
    #[ignore = "requires an isolated provisioned application and notification service with their socket-address environment variables"]
    fn test_runtime_service_pair_completes_registered_checks_in_order() {
        let addresses = [
            constants_str::ENV_NAMES_SERVICE_SOCKET_ADDRESS,
            stringify!(NOTIFICATION_SERVICE_SOCKET_ADDRESS),
        ]
        .map(std::env::var);
        assert!(addresses.iter().all(Result::is_ok));
        let [Ok(application_address), Ok(notification_address)] = addresses else {
            return;
        };
        let urls = [application_address, notification_address].map(|address| {
            runtime_tests::service_base_url::ServiceBaseUrl::try_from(format!(
                "{}{address}",
                constants_str::VALUE_8C8DAC95
            ))
        });
        assert!(urls.iter().all(Result::is_ok));
        let [Ok(application_url), Ok(notification_url)] = urls else {
            return;
        };
        let runtime_test_config = runtime_tests::runtime_test_config::RuntimeTestConfig::new(
            application_url,
            notification_url,
        );
        let result = runtime_tests::run(&runtime_test_config);
        assert!(
            result.is_ok_and(|runtime_test_report| runtime_test_report.passed()
                == [
                    runtime_tests::runtime_test_kind::RuntimeTestKind::ApplicationLiveness,
                    runtime_tests::runtime_test_kind::RuntimeTestKind::ApplicationReadiness,
                    runtime_tests::runtime_test_kind::RuntimeTestKind::NotificationServiceLiveness,
                    runtime_tests::runtime_test_kind::RuntimeTestKind::NotificationServiceReadiness,
                    runtime_tests::runtime_test_kind::RuntimeTestKind::NotificationCreation,
                ])
        );
    }

    #[test]
    #[ignore = "requires a provisioned application server; launched by browser acceptance deployment setup"]
    fn test_public_admin_deployment_endpoints() {
        let address = std::env::var(constants_str::ENV_NAMES_SERVICE_SOCKET_ADDRESS)
            .expect(constants_str::DIAGNOSTIC_0102D093);
        let base_url = runtime_tests::service_base_url::ServiceBaseUrl::try_from(format!(
            "{}{address}",
            constants_str::VALUE_8C8DAC95
        ))
        .expect(constants_str::DIAGNOSTIC_2FC594DB);
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect(constants_str::DIAGNOSTIC_6B10A8A9);
        assert!(
            [
                common_routes::common_route::CommonRoute::HealthLive,
                common_routes::common_route::CommonRoute::HealthReady
            ]
            .into_iter()
            .all(|route| {
                client
                    .get(format!(
                        "{}{}",
                        base_url.as_ref(),
                        route.contract().path().as_ref()
                    ))
                    .send()
                    .expect(constants_str::DIAGNOSTIC_3F6C6637)
                    .status()
                    .as_u16()
                    == 200u16
            })
        );
        let branding_response = client
            .get(format!(
                "{}/{}/{}",
                base_url.as_ref(),
                constants_str::ADMIN_UI_BRANDING,
                stringify!(read)
            ))
            .send()
            .expect(constants_str::DIAGNOSTIC_AEE527FD);
        assert_eq!(branding_response.status().as_u16(), 200u16);
        let branding = branding_response
            .json::<std::collections::BTreeMap<String, Option<String>>>()
            .expect(constants_str::DIAGNOSTIC_04936E98);
        assert_eq!(
            branding
                .get(stringify!(default_admin_route))
                .and_then(Option::as_deref),
            Some(constants_str::VALUE_074B6E5E)
        );
        assert!(
            branding
                .get(stringify!(site_name))
                .is_some_and(Option::is_some)
        );
        let stylesheet = client
            .get(format!(
                "{}{}",
                base_url.as_ref(),
                constants_str::VALUE_688DB289
            ))
            .send()
            .expect(constants_str::DIAGNOSTIC_5B1652D3);
        assert_eq!(stylesheet.status().as_u16(), 200u16);
        assert!(
            stylesheet
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|header| header.to_str().ok())
                .is_some_and(|header| header.contains(constants_str::TEXT_CSS))
        );
        assert_eq!(
            stylesheet
                .headers()
                .get(constants_str::X_CONTENT_TYPE_OPTIONS)
                .and_then(|header| header.to_str().ok()),
            Some(constants_str::NOSNIFF)
        );
        let mut bytes = Vec::new();
        let _read_size = std::io::Read::read_to_end(
            &mut std::io::Read::take(stylesheet, 1_048_577u64),
            &mut bytes,
        )
        .expect(constants_str::DIAGNOSTIC_51625562);
        assert!(bytes.len() > 1_000usize && bytes.len() <= 1_048_576usize);
    }
}
