#[test]
fn test_unexpected_results_preserve_expectation_diagnostics_without_payload_text() {
    let results = [
        std::panic::catch_unwind(|| {
            let _unexpected_value = crate::expect_ok::expect_ok::<
                crate::test_expectation_id::TestExpectationId,
                crate::test_panic_text::TestPanicText,
            >(
                Err(crate::test_panic_text::TestPanicText::from(
                    constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES,
                )),
                constants_str::VALUE_4F607799,
            );
        }),
        std::panic::catch_unwind(|| {
            let _unexpected_value = crate::expect_error::expect_error::<
                crate::test_panic_text::TestPanicText,
                crate::test_expectation_id::TestExpectationId,
            >(
                Ok(crate::test_panic_text::TestPanicText::from(
                    constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES,
                )),
                constants_str::VALUE_5CD39E4B,
            );
        }),
    ];
    assert!(
        results
            .into_iter()
            .zip([
                (
                    constants_str::ROUTE_VALIDATORS_EXPECT_OK_ER_ID,
                    constants_str::EXPECT_OK,
                    constants_str::ERR,
                    constants_str::VALUE_4F607799
                ),
                (
                    constants_str::ROUTE_VALIDATORS_EXPECT_ER_ER_ID,
                    constants_str::EXPECT_ERROR,
                    constants_str::OK,
                    constants_str::VALUE_5CD39E4B
                ),
            ])
            .all(
                |(result, (diagnostic_id, helper_name, actual_variant, expectation_id))| {
                    result.err().is_some_and(|payload| {
                        payload.downcast_ref::<String>().is_some_and(|message| {
                            message.starts_with(diagnostic_id)
                                && message.contains(helper_name)
                                && message.contains(actual_variant)
                                && message.contains(expectation_id)
                                && !message.contains(
                                    constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES,
                                )
                        })
                    })
                }
            )
    );
}

#[test]
fn test_error_variant_expectations_check_optional_status_and_reject_mismatches() {
    let headers = axum::http::HeaderMap::new();
    let missing_commit = || {
        crate::read_commit_header_str::read_commit_header_str(
            crate::axum_headers_ref::AxumHeadersRef::from(&headers),
        )
    };
    let statuses = [
        None,
        Some(crate::axum_http_status_code::AxumHttpStatusCode::bad_request()),
    ];
    assert!(statuses.into_iter().all(|status| {
        let extracted =
            crate::expect_err_variant_ref_with_status::expect_err_variant_ref_with_status(
                missing_commit(),
                constants_str::VALUE_8AFB4FFD,
                status,
                |commit_error| match commit_error {
                    crate::commit_error::CommitError::NoCommitHeader {
                        no_commit_header, ..
                    } => Some(*no_commit_header),
                    crate::commit_error::CommitError::CommitNotEq { .. }
                    | crate::commit_error::CommitError::CommitToStrConversion { .. } => None,
                },
            );
        extracted.as_str() == constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG
    }));
    let wrong_status = std::panic::catch_unwind(|| {
        crate::expect_err_variant_ref_with_status::expect_err_variant_ref_with_status::<
            _,
            _,
            crate::test_expectation_id::TestExpectationId,
        >(
            missing_commit(),
            constants_str::VALUE_8AFB4FFD,
            Some(crate::axum_http_status_code::AxumHttpStatusCode::im_a_teapot()),
            |_commit_error| {
                crate::panic_unexpected_variant::panic_unexpected_variant(
                    constants_str::VALUE_8AFB4FFD,
                )
            },
        )
    });
    assert!(wrong_status.err().is_some_and(|payload| {
        payload.downcast_ref::<String>().is_some_and(|message| {
            message.contains(&format!(
                "{:?}",
                crate::axum_http_status_code::AxumHttpStatusCode::bad_request()
            )) && message.contains(&format!(
                "{:?}",
                crate::axum_http_status_code::AxumHttpStatusCode::im_a_teapot()
            ))
        })
    }));
    assert!(statuses.into_iter().all(|status| {
        std::panic::catch_unwind(|| {
            crate::expect_err_variant_ref_with_status::expect_err_variant_ref_with_status(
                missing_commit(),
                constants_str::VALUE_8AFB4FFD,
                status,
                |_commit_error| None::<crate::test_expectation_id::TestExpectationId>,
            )
        })
        .err()
        .is_some_and(|payload| {
            payload.downcast_ref::<String>().is_some_and(|message| {
                constants_str::PANIC_4FE6F2E6
                    .split_whitespace()
                    .next()
                    .is_some_and(|diagnostic_id| message.starts_with(diagnostic_id))
                    && message.contains(constants_str::VALUE_8AFB4FFD)
            })
        })
    }));
    assert!(statuses.into_iter().all(|status| {
        std::panic::catch_unwind(|| {
            crate::expect_err_variant_ref_with_status::expect_err_variant_ref_with_status::<
                _,
                crate::commit_error::CommitError,
                crate::test_expectation_id::TestExpectationId,
            >(
                Ok(crate::header_str_ref::HeaderStrRef::from(constants_str::X)),
                constants_str::VALUE_8AFB4FFD,
                status,
                |_commit_error| {
                    crate::panic_unexpected_variant::panic_unexpected_variant(
                        constants_str::VALUE_8AFB4FFD,
                    )
                },
            )
        })
        .err()
        .is_some_and(|payload| {
            payload.downcast_ref::<String>().is_some_and(|message| {
                message.starts_with(constants_str::ROUTE_VALIDATORS_EXPECT_ER_ER_ID)
                    && message.contains(constants_str::EXPECT_ERROR)
                    && message.contains(constants_str::OK)
                    && message.contains(constants_str::VALUE_8AFB4FFD)
            })
        })
    }));
}

#[test]
fn test_error_mapping_checks_before_mapping_and_preserves_error_and_identifier() {
    let checked = std::cell::Cell::new(false);
    let mapped = crate::map_err::map_err::<(), _, _>(
        Err(crate::test_expectation_id::TestExpectationId::from(
            constants_str::X,
        )),
        constants_str::VALUE_8CE7A316,
        |test_expectation_id| {
            assert_eq!(test_expectation_id.get(), constants_str::X);
            assert!(!checked.replace(true));
        },
        |test_expectation_id, expectation_id| {
            assert!(checked.get());
            assert_eq!(expectation_id, constants_str::VALUE_8CE7A316);
            test_expectation_id
        },
    );
    assert!(checked.get());
    assert_eq!(mapped.get(), constants_str::X);
}

#[test]
fn test_error_mapping_skips_mapper_when_check_panics() {
    let mapped = std::cell::Cell::new(false);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::map_err::map_err::<(), _, _>(
            Err(crate::test_expectation_id::TestExpectationId::from(
                constants_str::X,
            )),
            constants_str::VALUE_8CE7A316,
            |_test_expectation_id| {
                crate::panic_unexpected_variant::panic_unexpected_variant(
                    constants_str::VALUE_8CE7A316,
                )
            },
            |test_expectation_id, _expectation_id| {
                mapped.set(true);
                test_expectation_id
            },
        )
    }));
    assert!(result.is_err());
    assert!(!mapped.get());
}

#[test]
fn test_borrowed_variant_rejection_preserves_exact_diagnostic_and_input() {
    let value = crate::test_expectation_id::TestExpectationId::from(constants_str::X);
    let result = std::panic::catch_unwind(|| {
        crate::expect_variant_ref::expect_variant_ref(
            &value,
            |test_expectation_id| {
                assert_eq!(test_expectation_id.get(), constants_str::X);
                None::<crate::test_expectation_id::TestExpectationId>
            },
            constants_str::VALUE_8CE7A316,
        )
    });
    let expected = constants_str::PANIC_4FE6F2E6.replacen(
        constants_str::PANIC_PLACEHOLDER_D8C45567,
        constants_str::VALUE_8CE7A316,
        constants_usize::ONE,
    );
    assert!(
        result
            .err()
            .is_some_and(|payload| { payload.downcast_ref::<String>() == Some(&expected) })
    );
    assert_eq!(value.get(), constants_str::X);
}
