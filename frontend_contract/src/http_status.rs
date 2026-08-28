#[path = "frontend_contract_body_error.rs"]
mod frontend_contract_body_error;
#[path = "http_status_try_from_u16_error.rs"]
mod http_status_try_from_u16_error;
#[path = "known_http_status.rs"]
mod known_http_status;

pub use frontend_contract_body_error::FrontendContractBodyError;
pub use http_status_try_from_u16_error::HttpStatusTryFromU16Error;
pub use known_http_status::KnownHttpStatus;

#[cfg(test)]
mod tests {
    #[test]
    fn known_status_preserves_protocol_code() {
        assert_eq!(super::KnownHttpStatus::TooManyRequests.get(), 429u16);
    }
}
