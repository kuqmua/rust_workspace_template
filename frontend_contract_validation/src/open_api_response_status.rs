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
