#![allow(
    clippy::module_inception,
    reason = "same-named type and function owners require nested modules under the facade"
)]
#[path = "axum_body.rs"]
mod axum_body;
#[path = "axum_body_size_error.rs"]
mod axum_body_size_error;
#[path = "body_size_error.rs"]
mod body_size_error;
#[path = "body_size_limit_bytes.rs"]
mod body_size_limit_bytes;
#[path = "bytes_body_bytes.rs"]
mod bytes_body_bytes;
#[path = "check_body_size/check_body_size.rs"]
mod check_body_size;
#[path = "http_body_size_hint.rs"]
mod http_body_size_hint;

pub use axum_body::AxumBody;
pub use axum_body_size_error::AxumBodySizeError;
pub use body_size_error::{BodySizeError, BodySizeErrorWithSerde};
pub use body_size_limit_bytes::BodySizeLimitBytes;
pub use bytes_body_bytes::BytesBodyBytes;
pub use check_body_size::check_body_size;
pub use http_body_size_hint::HttpBodySizeHint;
#[cfg(test)]
mod tests {
    fn expect_reached_max_size(
        body: axum::body::Body,
        limit: usize,
        exp_id: &'static str,
    ) -> (usize, Option<u64>) {
        crate::domain_types::test_helper::assert_err_status_code_variant_ref(
            crate::domain_types::test_helper::block_on(super::check_body_size(body, limit)),
            exp_id,
            crate::domain_types::AxumHttpStatusCode::payload_too_large(),
            |v| {
                Some(match v {
                    super::BodySizeError::ReachedMaximumSizeOfBody {
                        maximum_size_of_body_limit_in_bytes,
                        size_hint,
                        ..
                    } => (maximum_size_of_body_limit_in_bytes.0, size_hint.0.upper()),
                })
            },
        )
    }
    fn assert_reached_max_size_limit(body: axum::body::Body, limit: usize, exp_id: &'static str) {
        let (maximum_size_of_body_limit_in_bytes, _) = expect_reached_max_size(body, limit, exp_id);
        assert_eq!(maximum_size_of_body_limit_in_bytes, limit);
    }
    fn assert_body_bytes_eq(
        body: axum::body::Body,
        limit: usize,
        exp_id: &'static str,
        exp: &'static [u8],
    ) {
        let actual = crate::domain_types::test_helper::expect_ok(
            crate::domain_types::test_helper::block_on(super::check_body_size(body, limit)),
            exp_id,
        );
        assert_eq!(actual.0, bytes::Bytes::from_static(exp));
    }
    #[test]
    fn check_body_size_returns_bytes_when_body_fits_limit() {
        assert_body_bytes_eq(
            axum::body::Body::from(constants_str::OK_ALT),
            8,
            constants_str::VALUE_2FB3E958,
            b"ok",
        );
    }
    #[test]
    fn check_body_size_returns_bytes_when_size_eq_limit() {
        assert_body_bytes_eq(
            axum::body::Body::from(constants_str::OK_ALT),
            2,
            constants_str::VALUE_1736F4DB,
            b"ok",
        );
    }
    #[test]
    fn check_body_size_returns_bytes_for_empty_body_with_zero_limit() {
        assert_body_bytes_eq(
            axum::body::Body::empty(),
            0,
            constants_str::VALUE_44C8AD59,
            b"",
        );
    }
    #[test]
    fn check_body_size_returns_error_when_body_exceeds_limit() {
        assert_reached_max_size_limit(
            axum::body::Body::from(constants_str::OVERSIZED),
            2,
            constants_str::DDF0983A,
        );
    }
    #[test]
    fn check_body_size_returns_error_when_body_not_empty_and_limit_is_zero() {
        assert_reached_max_size_limit(
            axum::body::Body::from(constants_str::X),
            0,
            constants_str::VALUE_7DA3CAE4,
        );
    }
    #[test]
    fn check_body_size_error_contains_expected_non_zero_size_hint_for_static_body() {
        let (_, size_hint_upper) = expect_reached_max_size(
            axum::body::Body::from(constants_str::OVERSIZED),
            2,
            constants_str::CC0F2F3E,
        );
        assert_eq!(size_hint_upper, Some(9));
        assert_eq!(size_hint_upper.map(|v| v > 0), Some(true));
    }
    #[test]
    fn body_size_error_maps_to_payload_too_large() {
        crate::domain_types::test_helper::assert_err_status_code_only(
            crate::domain_types::test_helper::block_on(super::check_body_size(
                axum::body::Body::from(constants_str::TOO_BIG),
                1,
            )),
            constants_str::VALUE_7ED49BA1,
            crate::domain_types::AxumHttpStatusCode::payload_too_large(),
        );
    }
    #[test]
    fn body_size_error_keeps_limit_when_limit_is_one() {
        assert_reached_max_size_limit(
            axum::body::Body::from(constants_str::AB),
            1,
            constants_str::VALUE_1FE7A3B4,
        );
    }
}
