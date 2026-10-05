#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_deref_inner::DerefInner,
)]
pub struct OpenApiResponseStatus(u16);
impl TryFrom<u16> for OpenApiResponseStatus {
    type Error = frontend_contract::http_status_try_from_u16_error::HttpStatusTryFromU16Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        (100u16..1_000u16)
            .contains(&value)
            .then_some(Self(value))
            .ok_or(frontend_contract::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_openapi_response_status_accepts_exact_protocol_range() {
        assert!((u16::MIN..=u16::MAX).all(|value| {
            let result = crate::open_api_response_status::OpenApiResponseStatus::try_from(value);
            if (100u16..=999u16).contains(&value) {
                result.is_ok_and(|status| *status == value)
            } else {
                matches!(result, Err(frontend_contract::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange))
            }
        }));
    }
}
