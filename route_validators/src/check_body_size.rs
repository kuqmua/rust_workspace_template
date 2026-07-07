#[derive(Debug, newtype::Newtype)]
#[newtype(from)]
pub struct Body(pub axum::body::Body);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::Newtype,
)]
#[newtype(from, to_err_string)]
pub struct BodySizeLimitBytes(pub usize);
#[derive(Debug, newtype::Newtype)]
#[newtype(to_err_string)]
pub struct BodySizeAxumEr(pub axum::Error);
#[derive(Debug)]
pub struct BodySizeHint(pub http_body::SizeHint);
impl loc_lib::ToErrString for BodySizeHint {
    fn to_err_string(&self) -> loc_lib::ToErrStringValue {
        loc_lib::ToErrStringValue(format!("{:#?}", self.0))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyBytes(pub bytes::Bytes);
impl std::ops::Deref for BodyBytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
impl AsRef<[u8]> for BodyBytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
#[derive(Debug, thiserror::Error, loc_lib::Location, optml::Optml)]
pub enum BodySizeEr {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        er: BodySizeAxumEr,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: BodySizeLimitBytes,
        #[eo_to_err_string]
        size_hint: BodySizeHint,
        loc: loc_lib::loc::Loc,
    },
}
impl crate::GetAxumHttpStatusCode for BodySizeEr {
    const AXUM_HTTP_STATUS_CODE: axum::http::StatusCode = axum::http::StatusCode::PAYLOAD_TOO_LARGE;
}
impl BodySizeEr {
    #[allow(clippy::single_call_fn)] // keeps body-size error construction reusable and testable in one place
    fn reached_maximum_size_of_body(
        er: BodySizeAxumEr,
        maximum_size_of_body_limit_in_bytes: BodySizeLimitBytes,
        size_hint: BodySizeHint,
    ) -> Self {
        Self::ReachedMaximumSizeOfBody {
            er,
            maximum_size_of_body_limit_in_bytes,
            size_hint,
            loc: loc_lib::loc!(),
        }
    }
}
pub async fn check_body_size<BodyTy, LimitTy>(
    body: BodyTy,
    limit: LimitTy,
) -> Result<BodyBytes, BodySizeEr>
where
    BodyTy: Into<Body>,
    LimitTy: Into<BodySizeLimitBytes>,
{
    let body_value = body.into();
    let limit_value = limit.into();
    let size_hint = axum::body::HttpBody::size_hint(&body_value.0);
    axum::body::to_bytes(body_value.0, limit_value.0)
        .await
        .map(BodyBytes)
        .map_err(|er: axum::Error| {
            BodySizeEr::reached_maximum_size_of_body(
                BodySizeAxumEr(er),
                limit_value,
                BodySizeHint(size_hint),
            )
        })
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
            } => (maximum_size_of_body_limit_in_bytes.0, size_hint.0.upper()),
        }
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
        let actual = crate::test_hlp::expect_ok(
            crate::test_hlp::block_on(super::check_body_size(body, limit)),
            exp_id,
        );
        assert_eq!(actual.0, bytes::Bytes::from_static(exp));
    }
    #[test]
    fn check_body_size_returns_bytes_when_body_fits_limit() {
        assert_body_bytes_eq(axum::body::Body::from("ok"), 8, "2fb3e958", b"ok");
    }
    #[test]
    fn check_body_size_returns_bytes_when_size_eq_limit() {
        assert_body_bytes_eq(axum::body::Body::from("ok"), 2, "1736f4db", b"ok");
    }
    #[test]
    fn check_body_size_returns_bytes_for_empty_body_with_zero_limit() {
        assert_body_bytes_eq(axum::body::Body::empty(), 0, "44c8ad59", b"");
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
