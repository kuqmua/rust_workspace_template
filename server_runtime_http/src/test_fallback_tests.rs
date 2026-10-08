#[cfg(test)]
mod tests {
    #[test]
    fn test_api_path_is_machine_readable_without_accept_header() {
        assert_eq!(
            crate::resolve_fallback_response_mode::resolve_fallback_response_mode(
                crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(
                    constants_str::TEST_SERVICE_USERS_PATH
                ),
                crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(
                    constants_str::TEST_SERVICE_PREFIX
                ),
                crate::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef::from(
                    constants_str::METRICS
                ),
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(None),
                crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(
                    1024usize
                ),
            ),
            crate::fallback_response_mode::FallbackResponseMode::MachineReadable
        );
    }
    #[test]
    fn test_invalid_or_zero_quality_json_is_not_accepted() {
        let resolve_mode = |http_optional_accept_header_ref: crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef<'_>| {
            crate::resolve_fallback_response_mode::resolve_fallback_response_mode(
                    crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(
                        constants_str::TEST_SIGNIN_PATH,
                    ),
                    crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(
                        constants_str::TEST_SERVICE_PREFIX,
                    ),
                    crate::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef::from(
                        constants_str::METRICS,
                    ),
                    http_optional_accept_header_ref,
                    crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(
                        1024usize,
                    ),
                )
        };
        let invalid_accepts = [
            constants_str::TEST_ACCEPT_JSON_BARE_QUALITY,
            constants_str::TEST_ACCEPT_JSON_EMPTY_ONE_FRACTION_QUALITY,
            constants_str::TEST_ACCEPT_JSON_OUT_OF_RANGE_QUALITY,
            constants_str::TEST_ACCEPT_JSON_NONZERO_ONE_FRACTION_QUALITY,
            constants_str::TEST_ACCEPT_JSON_EXCESS_PRECISION_QUALITY,
        ];
        assert!(invalid_accepts.into_iter().all(|value| {
            let accept = http::HeaderValue::from_static(value);
            resolve_mode(
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(
                    &accept,
                )),
            ) == crate::fallback_response_mode::FallbackResponseMode::HumanReadable
        }));
        let accept =
            http::HeaderValue::from_static(constants_str::TEST_ACCEPT_HTML_JSON_ZERO_QUALITY);
        assert_eq!(
            resolve_mode(
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(
                    &accept,
                )),
            ),
            crate::fallback_response_mode::FallbackResponseMode::HumanReadable
        );
        let decimal_accept = http::HeaderValue::from_static(
            constants_str::TEST_ACCEPT_HTML_JSON_ZERO_DECIMAL_QUALITY,
        );
        assert_eq!(
            resolve_mode(
                crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(
                    &decimal_accept,
                )),
            ),
            crate::fallback_response_mode::FallbackResponseMode::HumanReadable
        );
        assert!(
            [
                constants_str::APPLICATION_JSON,
                constants_str::TEST_ACCEPT_JSON_MINIMUM_POSITIVE_QUALITY,
                constants_str::TEST_ACCEPT_JSON_ONE_DECIMAL_QUALITY,
            ]
            .into_iter()
            .all(|value| {
                let positive_accept = http::HeaderValue::from_static(value);
                resolve_mode(
                    crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(
                        Some(&positive_accept),
                    ),
                ) == crate::fallback_response_mode::FallbackResponseMode::MachineReadable
            })
        );
    }
    fn fallback_mode_fixture(
        http_fallback_request_path_ref: crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef<'_>,
        http_fallback_api_prefix_ref: crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef<
            '_,
        >,
        http_optional_accept_header_ref: crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef<'_>,
        http_accept_header_maximum_bytes: crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes,
    ) -> crate::fallback_response_mode::FallbackResponseMode {
        crate::resolve_fallback_response_mode::resolve_fallback_response_mode(
            http_fallback_request_path_ref,
            http_fallback_api_prefix_ref,
            crate::http_fallback_metrics_path_ref::HttpFallbackMetricsPathRef::from(
                constants_str::METRICS,
            ),
            http_optional_accept_header_ref,
            http_accept_header_maximum_bytes,
        )
    }

    #[test]
    fn test_fallback_paths_preserve_exact_prefix_boundaries_and_override_accept_guards() {
        assert!(http::HeaderValue::from_bytes(&[0xffu8]).is_ok_and(|binary| {
            binary.to_str().is_err()
                && [constants_str::TEST_SERVICE_PREFIX.to_owned(), format!("{}/", constants_str::TEST_SERVICE_PREFIX)]
                .into_iter().all(|prefix| {
                    [
                        (constants_str::TEST_SERVICE_PREFIX.to_owned(), true),
                        (constants_str::TEST_SERVICE_USERS_PATH.to_owned(), true),
                        (format!("{}{}", constants_str::TEST_SERVICE_PREFIX, constants_str::X), false),
                        (constants_str::TEST_SIGNIN_PATH.to_owned(), false),
                        (constants_str::METRICS.to_owned(), true),
                        (format!("{}/{}", constants_str::METRICS, constants_str::X), false),
                    ].into_iter().all(|(path, machine)| {
                        [None, Some(http::HeaderValue::from_static(constants_str::APPLICATION_JSON)), Some(binary.clone())]
                            .into_iter().all(|header| {
                                fallback_mode_fixture(
                                    crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(path.as_str()),
                                    crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(prefix.as_str()),
                                    crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(header.as_ref()),
                                    crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(0usize),
                                ) == if machine { crate::fallback_response_mode::FallbackResponseMode::MachineReadable } else { crate::fallback_response_mode::FallbackResponseMode::HumanReadable }
                            })
                    })
                })
        }));
    }

    #[test]
    fn test_fallback_accept_byte_text_and_media_range_limits() {
        let resolve = |http_optional_accept_header_ref: crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef<'_>, http_accept_header_maximum_bytes: crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes| {
            fallback_mode_fixture(
                crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(constants_str::TEST_SIGNIN_PATH),
                crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(constants_str::TEST_SERVICE_PREFIX),
                http_optional_accept_header_ref,
                http_accept_header_maximum_bytes,
            )
        };
        let json = http::HeaderValue::from_static(constants_str::APPLICATION_JSON);
        assert!(
            [
                json.as_bytes().len() - 1usize,
                json.as_bytes().len(),
                json.as_bytes().len() + 1usize
            ]
            .into_iter()
            .all(|maximum| {
                resolve(
                    crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(
                        Some(&json),
                    ),
                    crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(
                        maximum,
                    ),
                ) == if maximum < json.as_bytes().len() {
                    crate::fallback_response_mode::FallbackResponseMode::HumanReadable
                } else {
                    crate::fallback_response_mode::FallbackResponseMode::MachineReadable
                }
            })
        );
        assert!(
            http::HeaderValue::from_bytes(&[0xffu8]).is_ok_and(|binary| {
                resolve(
                    crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(
                        Some(&binary),
                    ),
                    crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(
                        1usize,
                    ),
                ) == crate::fallback_response_mode::FallbackResponseMode::HumanReadable
            })
        );
        assert!([0usize, 126usize, 127usize, 128usize].into_iter().all(|preceding| {
            let mut text = std::iter::repeat_n(constants_str::X, preceding)
                .flat_map(|value| value.chars().chain(std::iter::once(',')))
                .collect::<String>();
            text.push_str(constants_str::APPLICATION_JSON);
            http::HeaderValue::try_from(text.as_str()).is_ok_and(|header| {
                resolve(crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(&header)), crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(text.len()))
                    == if preceding < 128usize { crate::fallback_response_mode::FallbackResponseMode::MachineReadable } else { crate::fallback_response_mode::FallbackResponseMode::HumanReadable }
            })
        }));
    }

    #[test]
    fn test_fallback_json_quality_accepts_exact_three_digit_fraction_domain() {
        let resolve = |http_optional_accept_header_ref: crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef<'_>| {
            fallback_mode_fixture(
                crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(constants_str::TEST_SIGNIN_PATH),
                crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(constants_str::TEST_SERVICE_PREFIX),
                http_optional_accept_header_ref,
                crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(1024usize),
            )
        };
        assert!((0u16..=999u16).all(|fraction| {
            [(0u8, fraction != 0u16), (1u8, fraction == 0u16)].into_iter().all(|(integer, accepted)| {
                let text = format!("{};{}={integer}.{fraction:03}", constants_str::APPLICATION_JSON, constants_str::HTTP_ACCEPT_QUALITY_PARAMETER);
                http::HeaderValue::try_from(text.as_str()).is_ok_and(|header| {
                    resolve(crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(Some(&header)))
                        == if accepted { crate::fallback_response_mode::FallbackResponseMode::MachineReadable }
                        else { crate::fallback_response_mode::FallbackResponseMode::HumanReadable }
                })
            })
        }));
    }
    #[test]
    fn test_fallback_json_quality_rejects_malformed_values_and_every_disabling_duplicate() {
        let resolve = |http_optional_accept_header_ref: crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef<'_>| {
            fallback_mode_fixture(
                crate::http_fallback_request_path_ref::HttpFallbackRequestPathRef::from(constants_str::TEST_SIGNIN_PATH),
                crate::http_fallback_api_prefix_ref::HttpFallbackApiPrefixRef::from(constants_str::TEST_SERVICE_PREFIX),
                http_optional_accept_header_ref,
                crate::http_accept_header_maximum_bytes::HttpAcceptHeaderMaximumBytes::from(1024usize),
            )
        };
        assert!(
            [
                (&[][..], false),
                (&['0'][..], false),
                (&['1'][..], true),
                (&['0', '.'][..], false),
                (&['.', '1'][..], false),
                (&['0', '.', '1'][..], true),
                (&['0', '.', '0', '1'][..], true),
                (&['1', '.', '0'][..], true),
                (&['1', '.', '0', '0'][..], true),
                (&['0', '.', 'a'][..], false),
                (&['0', '.', '1', 'a'][..], false),
                (&['0', '.', '1', '.', '0'][..], false),
                (&['0', '1'][..], false),
                (&['+', '1'][..], false),
                (&['-', '1'][..], false),
                (&['0', '.', '1', ' ', '0'][..], false),
            ]
            .into_iter()
            .all(|(characters, accepted)| {
                let quality = characters.iter().collect::<String>();
                let text = format!(
                    "{}; {} = {} ",
                    constants_str::APPLICATION_JSON.to_ascii_uppercase(),
                    constants_str::HTTP_ACCEPT_QUALITY_PARAMETER.to_ascii_uppercase(),
                    quality
                );
                http::HeaderValue::try_from(text.as_str()).is_ok_and(|header| {
                    resolve(
                        crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(
                            Some(&header),
                        ),
                    ) == if accepted {
                        crate::fallback_response_mode::FallbackResponseMode::MachineReadable
                    } else {
                        crate::fallback_response_mode::FallbackResponseMode::HumanReadable
                    }
                })
            })
        );
        assert!(
            [
                (constants_str::VALUE_0, constants_str::VALUE_1, false),
                (constants_str::VALUE_1, constants_str::VALUE_0, false),
                (constants_str::VALUE_1, constants_str::VALUE_1, true),
            ]
            .into_iter()
            .all(|(first, second, accepted)| {
                let text = format!(
                    "{};{}={first};{}={second}",
                    constants_str::APPLICATION_JSON,
                    constants_str::HTTP_ACCEPT_QUALITY_PARAMETER,
                    constants_str::HTTP_ACCEPT_QUALITY_PARAMETER
                );
                http::HeaderValue::try_from(text.as_str()).is_ok_and(|header| {
                    resolve(
                        crate::http_optional_accept_header_ref::HttpOptionalAcceptHeaderRef::from(
                            Some(&header),
                        ),
                    ) == if accepted {
                        crate::fallback_response_mode::FallbackResponseMode::MachineReadable
                    } else {
                        crate::fallback_response_mode::FallbackResponseMode::HumanReadable
                    }
                })
            })
        );
    }
}
