pub use crate::client_error::ClientError;
pub use crate::decode_api_problem::decode_api_problem;
pub use crate::page_contract::PageContract;
pub use crate::transport::Transport;
pub use crate::transport_body::TransportBody;
pub use crate::transport_error::TransportError;
pub use crate::transport_idempotency_key::*;
pub use crate::transport_if_match::*;
pub use crate::transport_path::*;
pub use crate::transport_request::TransportRequest;
pub use crate::transport_response::TransportResponse;
pub use crate::transport_retry_after::*;
pub use crate::transport_status::*;

#[cfg(test)]
mod tests {
    #[test]
    fn transport_body_enforces_shared_limit() {
        let oversized =
            vec![constants_u8::ZERO; constants_usize::VALUE_16_777_216 + constants_usize::ONE];
        assert_eq!(
            super::TransportBody::try_from(oversized),
            Err(crate::domain_types::FrontendContractBodyError),
        );
    }
}
