#[path = "http_status/frontend_contract_body_error.rs"]
mod frontend_contract_body_error;
#[path = "http_status/frontend_contract_body_max_bytes.rs"]
mod frontend_contract_body_max_bytes;
#[path = "http_status/http_status_try_from_u16_error.rs"]
mod http_status_try_from_u16_error;
#[path = "http_status/known_http_status.rs"]
mod known_http_status;

pub use frontend_contract_body_error::FrontendContractBodyError;
pub use frontend_contract_body_max_bytes::FRONTEND_CONTRACT_BODY_MAX_BYTES;
pub use http_status_try_from_u16_error::HttpStatusTryFromU16Error;
pub use known_http_status::KnownHttpStatus;

#[cfg(test)]
mod tests {
    #[test]
    fn known_status_preserves_protocol_code() {
        assert_eq!(super::KnownHttpStatus::TooManyRequests.get(), 429u16);
    }
}
