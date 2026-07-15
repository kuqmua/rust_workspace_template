#[derive(Debug, newtype::Newtype)]
#[newtype(from)]
pub struct AxumBody(axum::body::Body);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Newtype,
)]
#[newtype(from, to_err_string)]
pub struct BodySizeLimitBytes(usize);
#[derive(Debug, newtype::Newtype)]
#[newtype(to_err_string)]
pub struct AxumBodySizeError(axum::Error);
#[derive(Debug)]
pub struct HttpBodySizeHint(http_body::SizeHint);
impl to_err_string::ToErrString for HttpBodySizeHint {
    fn to_err_string(&self) -> to_err_string::ToErrStringValue {
        to_err_string::ToErrStringValue::try_from(format!("{:#?}", self.0))
            .unwrap_or_else(to_err_string::ToErrStringValue::from)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
#[newtype(deref_target)]
pub struct BytesBodyBytes(bytes::Bytes);
impl AsRef<[u8]> for BytesBodyBytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
#[location::errors_with_location]
#[derive(Debug, thiserror::Error, location::Location, optml::Optml)]
#[location_to_schema]
pub enum BodySizeError {
    ReachedMaximumSizeOfBody {
        #[eo_to_err_string]
        error: AxumBodySizeError,
        #[eo_to_err_string_serde]
        maximum_size_of_body_limit_in_bytes: BodySizeLimitBytes,
        #[eo_to_err_string]
        size_hint: HttpBodySizeHint,
    },
}
impl crate::GetAxumHttpStatusCode for BodySizeError {
    const AXUM_HTTP_STATUS_CODE: crate::AxumHttpStatusCode =
        crate::AxumHttpStatusCode::PAYLOAD_TOO_LARGE;
}
impl BodySizeError {
    #[allow(clippy::single_call_fn)] // keeps body-size error construction reusable and testable in one place
    fn reached_maximum_size_of_body(
        error: AxumBodySizeError,
        maximum_size_of_body_limit_in_bytes: BodySizeLimitBytes,
        size_hint: HttpBodySizeHint,
    ) -> Self {
        Self::ReachedMaximumSizeOfBody {
            error,
            maximum_size_of_body_limit_in_bytes,
            size_hint,
            location: location_macros::location!(),
        }
    }
}
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
        .map(BytesBodyBytes)
        .map_err(|error| {
            BodySizeError::reached_maximum_size_of_body(
                AxumBodySizeError(error),
                limit_value,
                HttpBodySizeHint(size_hint),
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
            crate::AxumHttpStatusCode::PAYLOAD_TOO_LARGE,
            |v| Some(reached_max_size_fields(v)),
        )
    }
    #[allow(clippy::single_call_fn)] // shared extractor keeps reached-max-size assertions reusable across tests
    fn reached_max_size_fields(v: &super::BodySizeError) -> (usize, Option<u64>) {
        match v {
            super::BodySizeError::ReachedMaximumSizeOfBody {
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
        assert_body_bytes_eq(
            axum::body::Body::from(str_constants::OK_ALT),
            8,
            str_constants::VALUE_2FB3E958,
            b"ok",
        );
    }
    #[test]
    fn check_body_size_returns_bytes_when_size_eq_limit() {
        assert_body_bytes_eq(
            axum::body::Body::from(str_constants::OK_ALT),
            2,
            str_constants::VALUE_1736F4DB,
            b"ok",
        );
    }
    #[test]
    fn check_body_size_returns_bytes_for_empty_body_with_zero_limit() {
        assert_body_bytes_eq(
            axum::body::Body::empty(),
            0,
            str_constants::VALUE_44C8AD59,
            b"",
        );
    }
    #[test]
    fn check_body_size_returns_error_when_body_exceeds_limit() {
        assert_reached_max_size_limit(
            axum::body::Body::from(str_constants::OVERSIZED),
            2,
            str_constants::DDF0983A,
        );
    }
    #[test]
    fn check_body_size_returns_error_when_body_not_empty_and_limit_is_zero() {
        assert_reached_max_size_limit(
            axum::body::Body::from(str_constants::X),
            0,
            str_constants::VALUE_7DA3CAE4,
        );
    }
    #[test]
    fn check_body_size_error_contains_expected_non_zero_size_hint_for_static_body() {
        let (_, size_hint_upper) = expect_reached_max_size(
            axum::body::Body::from(str_constants::OVERSIZED),
            2,
            str_constants::CC0F2F3E,
        );
        assert_eq!(size_hint_upper, Some(9));
        assert_eq!(size_hint_upper.map(|v| v > 0), Some(true));
    }
    #[test]
    fn body_size_error_maps_to_payload_too_large() {
        crate::test_hlp::assert_err_status_code_only(
            crate::test_hlp::block_on(super::check_body_size(
                axum::body::Body::from(str_constants::TOO_BIG),
                1,
            )),
            str_constants::VALUE_7ED49BA1,
            crate::AxumHttpStatusCode::PAYLOAD_TOO_LARGE,
        );
    }
    #[test]
    fn body_size_error_keeps_limit_when_limit_is_one() {
        assert_reached_max_size_limit(
            axum::body::Body::from(str_constants::AB),
            1,
            str_constants::VALUE_1FE7A3B4,
        );
    }
}
