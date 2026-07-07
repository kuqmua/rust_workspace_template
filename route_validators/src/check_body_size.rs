#[derive(Debug, thiserror::Error, loc_lib::Location, optml::Optml)]
pub enum BodySizeEr {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        er: axum::Error,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: usize,
        #[eo_to_err_string]
        size_hint: http_body::SizeHint,
        loc: loc_lib::loc::Loc,
    },
}
impl crate::GetAxumHttpStatusCode for BodySizeEr {
    const AXUM_HTTP_STATUS_CODE: axum::http::StatusCode = axum::http::StatusCode::PAYLOAD_TOO_LARGE;
}
impl BodySizeEr {
    #[allow(clippy::single_call_fn)] // keeps body-size error construction reusable and testable in one place
    fn reached_maximum_size_of_body(
        er: axum::Error,
        maximum_size_of_body_limit_in_bytes: usize,
        size_hint: http_body::SizeHint,
    ) -> Self {
        Self::ReachedMaximumSizeOfBody {
            er,
            maximum_size_of_body_limit_in_bytes,
            size_hint,
            loc: loc_lib::loc!(),
        }
    }
}
pub async fn check_body_size(
    body: axum::body::Body,
    limit: usize,
) -> Result<bytes::Bytes, BodySizeEr> {
    let size_hint = axum::body::HttpBody::size_hint(&body);
    axum::body::to_bytes(body, limit)
        .await
        .map_err(|er: axum::Error| BodySizeEr::reached_maximum_size_of_body(er, limit, size_hint))
}
#[cfg(test)]
mod tests {
    fn expect_reached_max_size(
        body: axum::body::Body,
        limit: usize,
        exp_id: &'static str,
    ) -> (usize, Option<u64>) {
        crate::test_hlp::assert_err_status_code_variant_ref(
            crate::test_hlp::block_on(super::check_body_size(body, limit)),
            exp_id,
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            |v| Some(reached_max_size_fields(v)),
        )
    }
    #[allow(clippy::single_call_fn)] // shared extractor keeps reached-max-size assertions reusable across tests
    fn reached_max_size_fields(v: &super::BodySizeEr) -> (usize, Option<u64>) {
        match v {
            super::BodySizeEr::ReachedMaximumSizeOfBody {
                maximum_size_of_body_limit_in_bytes,
                size_hint,
                ..
            } => (*maximum_size_of_body_limit_in_bytes, size_hint.upper()),
        }
    }
    fn assert_reached_max_size_limit(body: axum::body::Body, limit: usize, exp_id: &'static str) {
        let (maximum_size_of_body_limit_in_bytes, _) = expect_reached_max_size(body, limit, exp_id);
        assert_eq!(maximum_size_of_body_limit_in_bytes, limit);
    }
    #[test]
    fn check_body_size_returns_bytes_when_body_fits_limit() {
        crate::test_hlp::assert_ok_eq(
            crate::test_hlp::block_on(super::check_body_size(axum::body::Body::from("ok"), 8)),
            "2fb3e958",
            &bytes::Bytes::from_static(b"ok"),
        );
    }
    #[test]
    fn check_body_size_returns_bytes_when_size_eq_limit() {
        crate::test_hlp::assert_ok_eq(
            crate::test_hlp::block_on(super::check_body_size(axum::body::Body::from("ok"), 2)),
            "1736f4db",
            &bytes::Bytes::from_static(b"ok"),
        );
    }
    #[test]
    fn check_body_size_returns_bytes_for_empty_body_with_zero_limit() {
        crate::test_hlp::assert_ok_eq(
            crate::test_hlp::block_on(super::check_body_size(axum::body::Body::empty(), 0)),
            "44c8ad59",
            &bytes::Bytes::from_static(b""),
        );
    }
    #[test]
    fn check_body_size_returns_error_when_body_exceeds_limit() {
        assert_reached_max_size_limit(axum::body::Body::from("oversized"), 2, "ddf0983a");
    }
    #[test]
    fn check_body_size_returns_error_when_body_not_empty_and_limit_is_zero() {
        assert_reached_max_size_limit(axum::body::Body::from("x"), 0, "7da3cae4");
    }
    #[test]
    fn check_body_size_error_contains_expected_non_zero_size_hint_for_static_body() {
        let (_, size_hint_upper) =
            expect_reached_max_size(axum::body::Body::from("oversized"), 2, "cc0f2f3e");
        assert_eq!(size_hint_upper, Some(9));
        assert_eq!(size_hint_upper.map(|v| v > 0), Some(true));
    }
    #[test]
    fn body_size_error_maps_to_payload_too_large() {
        crate::test_hlp::assert_err_status_code_only(
            crate::test_hlp::block_on(super::check_body_size(axum::body::Body::from("too-big"), 1)),
            "7ed49ba1",
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
        );
    }
    #[test]
    fn body_size_error_keeps_limit_when_limit_is_one() {
        assert_reached_max_size_limit(axum::body::Body::from("ab"), 1, "1fe7a3b4");
    }
}
