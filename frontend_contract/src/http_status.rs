pub use crate::frontend_contract_body_error::FrontendContractBodyError;
pub use crate::http_status_try_from_u16_error::HttpStatusTryFromU16Error;
pub use crate::known_http_status::KnownHttpStatus;

#[cfg(test)]
mod tests {
    #[test]
    fn known_status_preserves_protocol_code() {
        assert_eq!(super::KnownHttpStatus::TooManyRequests.get(), 429u16);
    }
}
