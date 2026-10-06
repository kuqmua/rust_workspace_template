#[cfg(test)]
mod tests {
    #[test]
    fn test_local_runtime_configuration_uses_registered_service_urls() {
        assert!(crate::local_config().is_ok_and(|runtime_test_config| {
            runtime_test_config.application_base_url().as_ref() == constants_str::VALUE_D30A576C
                && runtime_test_config.notification_service_base_url().as_ref()
                    == constants_str::VALUE_08D5F409
        }));
    }

    #[test]
    fn test_runtime_failure_diagnostics_preserve_test_kind_and_status_order() {
        [
            (
                crate::runtime_test_kind::RuntimeTestKind::ApplicationLiveness,
                constants_str::VALUE_2AE6635F,
            ),
            (
                crate::runtime_test_kind::RuntimeTestKind::ApplicationReadiness,
                constants_str::VALUE_27B02AA0,
            ),
            (
                crate::runtime_test_kind::RuntimeTestKind::NotificationCreation,
                constants_str::VALUE_D1712BA9,
            ),
            (
                crate::runtime_test_kind::RuntimeTestKind::NotificationServiceLiveness,
                constants_str::VALUE_FA6BAA20,
            ),
            (
                crate::runtime_test_kind::RuntimeTestKind::NotificationServiceReadiness,
                constants_str::VALUE_7595852C,
            ),
        ]
        .into_iter()
        .fold((), |(), (runtime_test_kind, label)| {
            assert_eq!(runtime_test_kind.to_string(), label);
            let actual = crate::http_runtime_test_status::HttpRuntimeTestStatus::from(503u16);
            let expected = crate::http_runtime_test_status::HttpRuntimeTestStatus::from(200u16);
            let status_error = crate::runtime_test_error::RuntimeTestError::Status {
                actual,
                expected,
                test: runtime_test_kind,
            };
            assert_eq!(
                status_error.to_string(),
                format!(
                    "{}{}{}{}{}",
                    label,
                    constants_str::TEST_RUNTIME_STATUS_RETURNED_TEXT,
                    actual,
                    constants_str::TEST_RUNTIME_STATUS_EXPECTED_TEXT,
                    expected
                )
            );
            assert!(std::error::Error::source(&status_error).is_none());
            let unhealthy_error = crate::runtime_test_error::RuntimeTestError::Unhealthy {
                test: runtime_test_kind,
            };
            assert!(unhealthy_error.to_string().starts_with(label));
            assert!(std::error::Error::source(&unhealthy_error).is_none());
        });
    }

    #[test]
    fn test_service_base_url_normalizes_trailing_slashes() {
        let base_url = crate::service_base_url::ServiceBaseUrl::try_from(String::from(
            constants_str::VALUE_88B6A990,
        ))
        .expect(constants_str::DIAGNOSTIC_087DA3F2);
        assert_eq!(base_url.as_ref(), constants_str::VALUE_D30A576C);
    }

    #[test]
    fn test_service_base_url_rejects_non_http_urls_and_suffixes() {
        assert_eq!(
            crate::service_base_url::ServiceBaseUrl::try_from(String::from(
                constants_str::VALUE_A22A210E
            )),
            Err(crate::service_base_url_error::ServiceBaseUrlError::Scheme)
        );
        assert_eq!(
            crate::service_base_url::ServiceBaseUrl::try_from(String::from(
                constants_str::VALUE_9380378A
            )),
            Err(crate::service_base_url_error::ServiceBaseUrlError::Suffix)
        );
    }
}
