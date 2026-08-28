// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::shadow_reuse)]
pub(crate) use crate::assert_err_status_code::*;
pub(crate) use crate::assert_err_status_code_only::*;
pub(crate) use crate::assert_err_status_code_variant_ref::*;
pub(crate) use crate::assert_ok_eq::*;
pub(crate) use crate::assert_panics::*;
pub(crate) use crate::axum_test_header_value::*;
pub(crate) use crate::axum_test_headers::*;
pub(crate) use crate::axum_test_headers_mut_ref::*;
pub(crate) use crate::block_on::*;
pub(crate) use crate::expect_err_variant_ref_with_status::*;
pub(crate) use crate::expect_error::*;
pub(crate) use crate::expect_error_mapped::*;
pub(crate) use crate::expect_error_variant_ref::*;
pub(crate) use crate::expect_ok::*;
pub(crate) use crate::expect_variant::*;
pub(crate) use crate::expect_variant_ref::*;
pub(crate) use crate::increment_block_on_poll_count::*;
pub(crate) use crate::insert_header_no_prev::*;
pub(crate) use crate::is_block_on_poll_limit_reached::*;
pub(crate) use crate::make_headers_with_entry::*;
pub(crate) use crate::map_err::*;
pub(crate) use crate::map_err_after_status_check::*;
pub(crate) use crate::map_or_panic_unexpected_variant::*;
pub(crate) use crate::max_block_on_polls::*;
pub(crate) use crate::non_utf8_header_value::*;
pub(crate) use crate::panic_unexpected_result::*;
pub(crate) use crate::panic_unexpected_variant::*;
pub(crate) use crate::replace_header_name::*;
pub(crate) use crate::test_exp_id::*;
pub(crate) use crate::test_panic_text::*;
pub(crate) use crate::test_poll_count::*;
pub(crate) use crate::test_poll_limit_reached::*;

#[cfg(test)]
mod tests {
    #[test]
    fn block_on_panics_for_never_ready_future() {
        super::assert_panics(
            || {
                let _ignored = super::poll_test_future(std::future::poll_fn(|_| {
                    std::task::Poll::<u8>::Pending
                }));
            },
            constants_str::VALUE_1FC8C9F0,
        );
    }
    #[test]
    fn poll_limit_helper_returns_false_below_limit_and_true_at_limit() {
        assert!(!super::is_block_on_poll_limit_reached(
            super::TestPollCount::from(0)
        ));
        assert!(
            super::is_block_on_poll_limit_reached(super::TestPollCount::from(
                super::MAX_BLOCK_ON_POLLS,
            ))
            .0
        );
    }
    #[test]
    fn poll_count_increment_helper_increments_once() {
        let mut poll_count = super::TestPollCount::from(constants_usize::ZERO);
        super::increment_block_on_poll_count(&mut poll_count);
        assert_eq!(poll_count.0, constants_usize::ONE);
    }
    #[test]
    fn expect_ok_returns_inner_value() {
        let v = super::expect_ok::<u8, u16>(Ok(7), constants_str::VALUE_4F607799);
        assert_eq!(v, 7);
    }
    #[test]
    fn assert_ok_eq_checks_ok_result_value() {
        super::assert_ok_eq::<u8, u16>(Ok(7), constants_str::VALUE_9665F80A, &7);
    }
    #[test]
    fn expect_error_returns_inner_error() {
        let v = super::expect_error::<u8, u16>(Err(9), constants_str::VALUE_5CD39E4B);
        assert_eq!(v, 9);
    }
    #[test]
    fn expect_error_mapped_passes_error_and_exp_id_to_mapper() {
        let v = super::expect_error_mapped::<u8, u16, (u16, &'static str)>(
            Err(9),
            constants_str::VALUE_8CE7A316,
            |error, exp_id| (error, exp_id),
        );
        assert_eq!(v, (9, "8ce7a316"));
    }
    #[test]
    fn panic_unexpected_variant_always_panics() {
        super::assert_panics(
            || super::panic_unexpected_variant(constants_str::F66647AB),
            constants_str::B6DBA95D,
        );
    }
    #[test]
    fn expect_variant_returns_mapped_value_for_matching_variant() {
        let v = super::expect_variant(Some(7u8), |v| v, constants_str::VALUE_0DFD9A91);
        assert_eq!(v, 7);
    }
    #[test]
    fn expect_variant_ref_returns_mapped_value_for_matching_variant() {
        let value = Some(7u8);
        let v = super::expect_variant_ref(&value, |v| *v, constants_str::A2FCBAD4);
        assert_eq!(v, 7);
    }
    #[test]
    fn expect_variant_panics_for_unexpected_variant() {
        super::assert_panics(
            || {
                let _: u8 =
                    super::expect_variant::<Option<u8>, u8>(None, |v| v, constants_str::DBA097B9);
            },
            constants_str::A9651F69,
        );
    }
    #[test]
    fn expect_error_variant_maps_matching_error_variant() {
        #[derive(optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A(u8),
        }
        let v = super::expect_error_mapped::<(), TestError, u8>(
            Err(TestError::A(3)),
            constants_str::VALUE_9BF4CE17,
            |error, mapped_exp_id| {
                super::expect_variant(
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
    fn expect_error_variant_ref_maps_matching_error_variant_without_move() {
        #[derive(optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A(u8),
        }
        let v = super::expect_error_variant_ref::<(), TestError, u8>(
            Err(TestError::A(3)),
            constants_str::VALUE_8DFC4389,
            |error| match error {
                TestError::A(v) => Some(*v),
            },
        );
        assert_eq!(v, 3);
    }
    #[test]
    fn assert_err_status_code_variant_checks_status_and_extracts_variant() {
        #[derive(optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A,
        }
        impl crate::domain_types::AxumHttpStatusCodeProvider for TestError {
            fn axum_http_status_code(&self) -> crate::domain_types::AxumHttpStatusCode {
                crate::domain_types::AxumHttpStatusCode::bad_request()
            }
        }
        let _: () = super::map_err_after_status_check::<(), TestError, ()>(
            Err(TestError::A),
            constants_str::C1D74A8E,
            crate::domain_types::AxumHttpStatusCode::bad_request(),
            |error, mapped_exp_id| {
                super::expect_variant(
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
    fn assert_err_status_code_variant_ref_checks_status_and_extracts_variant_without_move() {
        #[derive(optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        enum TestError {
            A(u8),
        }
        impl crate::domain_types::AxumHttpStatusCodeProvider for TestError {
            fn axum_http_status_code(&self) -> crate::domain_types::AxumHttpStatusCode {
                crate::domain_types::AxumHttpStatusCode::bad_request()
            }
        }
        let v = super::assert_err_status_code_variant_ref::<(), TestError, u8>(
            Err(TestError::A(7)),
            constants_str::VALUE_8AFB4FFD,
            crate::domain_types::AxumHttpStatusCode::bad_request(),
            |error| match error {
                TestError::A(v) => Some(*v),
            },
        );
        assert_eq!(v, 7);
    }
    #[test]
    fn make_headers_with_entry_inserts_value_for_case_insensitive_name() {
        let headers = super::make_headers_with_entry(
            constants_str::COMMIT,
            axum::http::HeaderValue::from_static(constants_str::TEST_VALUES_WRONG_COMMIT),
        );
        let actual = headers.get(constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME);
        assert_eq!(
            actual,
            Some(&axum::http::HeaderValue::from_static("deadbeef"))
        );
    }
    #[test]
    fn replace_header_name_moves_value_to_new_key() {
        let mut headers = super::make_headers_with_entry(
            constants_str::X_COMMIT,
            axum::http::HeaderValue::from_static(constants_str::TEST_VALUES_WRONG_COMMIT),
        );
        super::replace_header_name(
            &mut headers,
            constants_str::X_COMMIT,
            axum::http::HeaderName::from_static(constants_str::ROUTE_VALIDATORS_COMMIT_HEADER_NAME),
            constants_str::VALUE_348C0E57,
        );
        assert!(headers.get("x-commit").is_none());
        assert_eq!(
            headers.get("commit"),
            Some(&axum::http::HeaderValue::from_static("deadbeef"))
        );
    }
    #[test]
    fn non_utf8_header_value_creates_non_utf8_header() {
        assert_eq!(
            super::non_utf8_header_value().to_str().err().map(|_| true),
            Some(true)
        );
    }
    #[test]
    fn assert_err_status_code_returns_error_after_status_check() {
        #[derive(optimal_memory_layout::OptimalMemoryLayout, std::fmt::Debug)]
        struct TestErr;
        impl crate::domain_types::AxumHttpStatusCodeProvider for TestErr {
            fn axum_http_status_code(&self) -> crate::domain_types::AxumHttpStatusCode {
                crate::domain_types::AxumHttpStatusCode::bad_request()
            }
        }
        let _err = super::assert_err_status_code::<(), TestErr>(
            Err(TestErr),
            constants_str::VALUE_4A1791D2,
            crate::domain_types::AxumHttpStatusCode::bad_request(),
        );
        super::assert_err_status_code_only::<(), TestErr>(
            Err(TestErr),
            constants_str::VALUE_773C5AF2,
            crate::domain_types::AxumHttpStatusCode::bad_request(),
        );
    }
}
