#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
pub async fn check_body_size<BodyTy, LimitTy>(
    body: BodyTy,
    limit: LimitTy,
) -> Result<BytesBodyBytes, BodySizeError>
where
    BodyTy: Into<AxumBody>,
    LimitTy: Into<BodySizeLimitBytes>,
{
    let body_value = body.into();
    let limit_value = limit.into();
    let size_hint = axum::body::HttpBody::size_hint(&body_value.0);
    axum::body::to_bytes(body_value.0, limit_value.0)
        .await
        .map(BytesBodyBytes::from)
        .map_err(|error| BodySizeError::ReachedMaximumSizeOfBody {
            error: AxumBodySizeError::from(error),
            maximum_size_of_body_limit_in_bytes: limit_value,
            size_hint: HttpBodySizeHint::from(size_hint),
            location: location_macros::location!(),
        })
}

pub use crate::axum_body::AxumBody;
pub use crate::axum_body_size_error::AxumBodySizeError;
pub use crate::body_size_error::{BodySizeError, BodySizeErrorWithSerde};
pub use crate::body_size_limit_bytes::BodySizeLimitBytes;
pub use crate::bytes_body_bytes::BytesBodyBytes;
pub use crate::http_body_size_hint::HttpBodySizeHint;
#[cfg(test)]
mod tests {
    fn expect_reached_max_size(
        body: axum::body::Body,
        limit: usize,
        exp_id: &'static str,
    ) -> (usize, Option<u64>) {
        crate::domain_types::test_helper::assert_err_status_code_variant_ref(
            crate::domain_types::test_helper::poll_test_future(super::check_body_size(body, limit)),
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
            crate::domain_types::test_helper::poll_test_future(super::check_body_size(body, limit)),
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
            crate::domain_types::test_helper::poll_test_future(super::check_body_size(
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
