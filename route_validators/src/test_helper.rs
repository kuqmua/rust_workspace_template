// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::shadow_reuse)]
#[cfg(test)]
mod tests {
    #[test]
    fn test_block_on_panics_for_never_ready_future() {
        crate::assert_panics::assert_panics(
            || {
                let _ignored =
                    crate::poll_test_future::poll_test_future(std::future::poll_fn(|_| {
                        std::task::Poll::<u8>::Pending
                    }));
            },
            constants_str::VALUE_1FC8C9F0,
        );
    }
    #[test]
    fn test_poll_limit_helper_returns_false_below_limit_and_true_at_limit() {
        assert!(
            !crate::is_block_on_poll_limit_reached::is_block_on_poll_limit_reached(
                crate::test_poll_count::TestPollCount::from(0)
            )
        );
        assert!(bool::from(
            crate::is_block_on_poll_limit_reached::is_block_on_poll_limit_reached(
                crate::test_poll_count::TestPollCount::from(
                    crate::max_block_on_polls::MAX_BLOCK_ON_POLLS,
                )
            )
        ));
    }
    #[test]
    fn test_poll_count_increment_helper_increments_once() {
        let mut poll_count = crate::test_poll_count::TestPollCount::from(constants_usize::ZERO);
        crate::increment_block_on_poll_count::increment_block_on_poll_count(&mut poll_count);
        assert_eq!(*poll_count, constants_usize::ONE);
    }
    #[test]
    fn test_expect_ok_returns_inner_value() {
        let v = crate::expect_ok::expect_ok::<u8, u16>(Ok(7), constants_str::VALUE_4F607799);
        assert_eq!(v, 7);
    }
    #[test]
    fn test_assert_ok_eq_checks_ok_result_value() {
        crate::assert_ok_eq::assert_ok_eq::<u8, u16>(Ok(7), constants_str::VALUE_9665F80A, &7);
    }
    #[test]
    fn test_expect_error_returns_inner_error() {
        let v = crate::expect_error::expect_error::<u8, u16>(Err(9), constants_str::VALUE_5CD39E4B);
        assert_eq!(v, 9);
    }
    #[test]
    fn test_expect_error_mapped_passes_error_and_exp_id_to_mapper() {
        let v = crate::expect_error_mapped::expect_error_mapped::<u8, u16, (u16, &'static str)>(
            Err(9),
            constants_str::VALUE_8CE7A316,
            |error, exp_id| (error, exp_id),
        );
        assert_eq!(v, (9, constants_str::VALUE_8CE7A316));
    }
    #[test]
    fn test_panic_unexpected_variant_always_panics() {
        crate::assert_panics::assert_panics(
            || crate::panic_unexpected_variant::panic_unexpected_variant(constants_str::F66647AB),
            constants_str::B6DBA95D,
        );
    }
    #[test]
    fn test_expect_variant_returns_mapped_value_for_matching_variant() {
        let v =
            crate::expect_variant::expect_variant(Some(7u8), |v| v, constants_str::VALUE_0DFD9A91);
        assert_eq!(v, 7);
    }
    #[test]
    fn test_expect_variant_ref_returns_mapped_value_for_matching_variant() {
        let value = Some(7u8);
        let v =
            crate::expect_variant_ref::expect_variant_ref(&value, |v| *v, constants_str::A2FCBAD4);
        assert_eq!(v, 7);
    }
    #[test]
    fn test_expect_variant_panics_for_unexpected_variant() {
        crate::assert_panics::assert_panics(
            || {
                let _: u8 = crate::expect_variant::expect_variant::<Option<u8>, u8>(
                    None,
                    |v| v,
                    constants_str::DBA097B9,
                );
            },
            constants_str::A9651F69,
        );
    }
    #[test]
    fn test_expect_error_variant_maps_matching_error_variant() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A(u8),
        }
        let v = crate::expect_error_mapped::expect_error_mapped::<(), TestError, u8>(
            Err(TestError::A(3)),
            constants_str::VALUE_9BF4CE17,
            |error, mapped_exp_id| {
                crate::expect_variant::expect_variant(
                    error,
                    |error| match error {
                        TestError::A(v) => Some(v),
                    },
                    mapped_exp_id,
                )
            },
        );
        assert_eq!(v, 3);
    }
    #[test]
    fn test_expect_error_variant_ref_maps_matching_error_variant_without_move() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A(u8),
        }
        let v = crate::expect_error_variant_ref::expect_error_variant_ref::<(), TestError, u8>(
            Err(TestError::A(3)),
            constants_str::VALUE_8DFC4389,
            |error| match error {
                TestError::A(v) => Some(*v),
            },
        );
        assert_eq!(v, 3);
    }
    #[test]
    fn test_assert_err_status_code_variant_checks_status_and_extracts_variant() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A,
        }
        impl crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider for TestError {
            fn axum_http_status_code(&self) -> crate::axum_http_status_code::AxumHttpStatusCode {
                crate::axum_http_status_code::AxumHttpStatusCode::bad_request()
            }
        }
        let _: () =
            crate::map_err_after_status_check::map_err_after_status_check::<(), TestError, ()>(
                Err(TestError::A),
                constants_str::C1D74A8E,
                crate::axum_http_status_code::AxumHttpStatusCode::bad_request(),
                |error, mapped_exp_id| {
                    crate::expect_variant::expect_variant(
                        error,
                        |error| match error {
                            TestError::A => Some(()),
                        },
                        mapped_exp_id,
                    );
                },
            );
    }
    #[test]
    fn test_assert_err_status_code_variant_ref_checks_status_and_extracts_variant_without_move() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A(u8),
        }
        impl crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider for TestError {
            fn axum_http_status_code(&self) -> crate::axum_http_status_code::AxumHttpStatusCode {
                crate::axum_http_status_code::AxumHttpStatusCode::bad_request()
            }
        }
        let v = crate::assert_err_status_code_variant_ref::assert_err_status_code_variant_ref::<
            (),
            TestError,
            u8,
        >(
            Err(TestError::A(7)),
            constants_str::VALUE_8AFB4FFD,
            crate::axum_http_status_code::AxumHttpStatusCode::bad_request(),
            |error| match error {
                TestError::A(v) => Some(*v),
            },
        );
        assert_eq!(v, 7);
    }
    #[test]
    fn test_make_headers_with_entry_inserts_value_for_case_insensitive_name() {
        let headers = crate::make_headers_with_entry::make_headers_with_entry(
            constants_str::COMMIT,
            axum::http::HeaderValue::from_static(constants_str::TEST_VALUES_WRONG_COMMIT),
        );
        let actual = headers.get(constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME);
        assert_eq!(
            actual,
            Some(&axum::http::HeaderValue::from_static(
                constants_str::TEST_VALUES_WRONG_COMMIT
            ))
        );
    }
    #[test]
    fn test_replace_header_name_moves_value_to_new_key() {
        let mut headers = crate::make_headers_with_entry::make_headers_with_entry(
            constants_str::X_COMMIT,
            axum::http::HeaderValue::from_static(constants_str::TEST_VALUES_WRONG_COMMIT),
        );
        crate::replace_header_name::replace_header_name(
            &mut headers,
            constants_str::X_COMMIT,
            axum::http::HeaderName::from_static(constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME),
            constants_str::VALUE_348C0E57,
        );
        assert!(headers.get(constants_str::X_COMMIT).is_none());
        assert_eq!(
            headers.get(constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME),
            Some(&axum::http::HeaderValue::from_static(
                constants_str::TEST_VALUES_WRONG_COMMIT
            ))
        );
    }
    #[test]
    fn test_non_utf8_header_value_creates_non_utf8_header() {
        assert_eq!(
            crate::non_utf8_header_value::non_utf8_header_value()
                .to_str()
                .err()
                .map(|_| true),
            Some(true)
        );
    }
    #[test]
    fn test_assert_err_status_code_returns_error_after_status_check() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        struct TestErr;
        impl crate::axum_http_status_code_provider::AxumHttpStatusCodeProvider for TestErr {
            fn axum_http_status_code(&self) -> crate::axum_http_status_code::AxumHttpStatusCode {
                crate::axum_http_status_code::AxumHttpStatusCode::bad_request()
            }
        }
        let _err = crate::assert_err_status_code::assert_err_status_code::<(), TestErr>(
            Err(TestErr),
            constants_str::VALUE_4A1791D2,
            crate::axum_http_status_code::AxumHttpStatusCode::bad_request(),
        );
        crate::assert_err_status_code_only::assert_err_status_code_only::<(), TestErr>(
            Err(TestErr),
            constants_str::VALUE_773C5AF2,
            crate::axum_http_status_code::AxumHttpStatusCode::bad_request(),
        );
    }
}
