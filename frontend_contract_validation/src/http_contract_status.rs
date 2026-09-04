#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct HttpContractStatus(u16);
impl TryFrom<u16> for HttpContractStatus {
    type Error = frontend_contract::http_status_try_from_u16_error::HttpStatusTryFromU16Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if !(100u16..1_000u16).contains(&value) {
            return Err(frontend_contract::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange);
        }
        Ok(Self(value))
    }
}
