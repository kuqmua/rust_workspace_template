#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
pub async fn check_body_size<BodyTy, LimitTy>(
    body: BodyTy,
    limit: LimitTy,
) -> Result<crate::bytes_body_bytes::BytesBodyBytes, crate::body_size_error::BodySizeError>
where
    BodyTy: Into<crate::axum_body::AxumBody>,
    LimitTy: Into<crate::body_size_limit_bytes::BodySizeLimitBytes>,
{
    let body_value = body.into();
    let limit_value = limit.into();
    let size_hint = axum::body::HttpBody::size_hint(&body_value.0);
    axum::body::to_bytes(body_value.0, limit_value.0)
        .await
        .map(crate::bytes_body_bytes::BytesBodyBytes::from)
        .map_err(
            |error| crate::body_size_error::BodySizeError::ReachedMaximumSizeOfBody {
                error: crate::axum_body_size_error::AxumBodySizeError::from(error),
                maximum_size_of_body_limit_in_bytes: limit_value,
                size_hint: crate::http_body_size_hint::HttpBodySizeHint::from(size_hint),
                location: location_macros::location!(),
            },
        )
}
#[cfg(test)]
mod tests {
    fn expect_reached_max_size(
        body: axum::body::Body,
        limit: usize,
        exp_id: &'static str,
    ) -> (usize, Option<u64>) {
        crate::assert_err_status_code_variant_ref::assert_err_status_code_variant_ref(
            crate::poll_test_future::poll_test_future(crate::check_body_size::check_body_size(
                body, limit,
            )),
            exp_id,
            crate::axum_http_status_code::AxumHttpStatusCode::payload_too_large(),
            |v| {
                Some(match v {
                    crate::body_size_error::BodySizeError::ReachedMaximumSizeOfBody {
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
        let actual = crate::expect_ok::expect_ok(
            crate::poll_test_future::poll_test_future(crate::check_body_size::check_body_size(
                body, limit,
            )),
            exp_id,
        );
        assert_eq!(actual.0, bytes::Bytes::from_static(exp));
    }
    #[test]
    fn check_body_size_returns_bytes_when_body_fits_limit() {
        assert_body_bytes_eq(
            axum::body::Body::from(constants_str::catalog::OK_ALT),
            8,
            constants_str::catalog::VALUE_2FB3E958,
            b"ok",
        );
    }
    #[test]
    fn check_body_size_returns_bytes_when_size_eq_limit() {
        assert_body_bytes_eq(
            axum::body::Body::from(constants_str::catalog::OK_ALT),
            2,
            constants_str::catalog::VALUE_1736F4DB,
            b"ok",
        );
    }
    #[test]
    fn check_body_size_returns_bytes_for_empty_body_with_zero_limit() {
        assert_body_bytes_eq(
            axum::body::Body::empty(),
            0,
            constants_str::catalog::VALUE_44C8AD59,
            b"",
        );
    }
    #[test]
    fn check_body_size_returns_error_when_body_exceeds_limit() {
        assert_reached_max_size_limit(
            axum::body::Body::from(constants_str::catalog::OVERSIZED),
            2,
            constants_str::catalog::DDF0983A,
        );
    }
    #[test]
    fn check_body_size_returns_error_when_body_not_empty_and_limit_is_zero() {
        assert_reached_max_size_limit(
            axum::body::Body::from(constants_str::catalog::X),
            0,
            constants_str::catalog::VALUE_7DA3CAE4,
        );
    }
    #[test]
    fn check_body_size_error_contains_expected_non_zero_size_hint_for_static_body() {
        let (_, size_hint_upper) = expect_reached_max_size(
            axum::body::Body::from(constants_str::catalog::OVERSIZED),
            2,
            constants_str::catalog::CC0F2F3E,
        );
        assert_eq!(size_hint_upper, Some(9));
        assert_eq!(size_hint_upper.map(|v| v > 0), Some(true));
    }
    #[test]
    fn body_size_error_maps_to_payload_too_large() {
        crate::assert_err_status_code_only::assert_err_status_code_only(
            crate::poll_test_future::poll_test_future(crate::check_body_size::check_body_size(
                axum::body::Body::from(constants_str::catalog::TOO_BIG),
                1,
            )),
            constants_str::catalog::VALUE_7ED49BA1,
            crate::axum_http_status_code::AxumHttpStatusCode::payload_too_large(),
        );
    }
    #[test]
    fn body_size_error_keeps_limit_when_limit_is_one() {
        assert_reached_max_size_limit(
            axum::body::Body::from(constants_str::catalog::AB),
            1,
            constants_str::catalog::VALUE_1FE7A3B4,
        );
    }
}
