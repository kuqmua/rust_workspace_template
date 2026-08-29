pub use super::client_error::ClientError;
pub use super::decode_api_problem::decode_api_problem;
pub use super::page_contract::PageContract;
pub use super::transport::Transport;
pub use super::transport_body::TransportBody;
pub use super::transport_error::TransportError;
pub use super::transport_idempotency_key::*;
pub use super::transport_if_match::*;
pub use super::transport_path::*;
pub use super::transport_request::TransportRequest;
pub use super::transport_response::TransportResponse;
pub use super::transport_retry_after::*;
pub use super::transport_status::*;
#[cfg(test)]
mod tests {
    #[test]
    fn transport_body_enforces_shared_limit() {
        let oversized =
            vec![constants_u8::ZERO; constants_usize::VALUE_16_777_216 + constants_usize::ONE];
        assert_eq!(
            super::TransportBody::try_from(oversized),
            Err(crate::FrontendContractBodyError),
        );
    }
}
