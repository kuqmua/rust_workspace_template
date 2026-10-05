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

#[cfg(test)]
mod tests {
    #[test]
    fn test_http_contract_status_accepts_exact_protocol_range() {
        assert!((u16::MIN..=u16::MAX).all(|value| {
            let result = crate::http_contract_status::HttpContractStatus::try_from(value);
            if (100u16..=999u16).contains(&value) {
                result.is_ok()
            } else {
                matches!(result, Err(frontend_contract::http_status_try_from_u16_error::HttpStatusTryFromU16Error::OutOfRange))
            }
        }));
    }
}
