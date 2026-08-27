#[path = "page_transport/client_error.rs"]
mod client_error;
#[path = "page_transport/decode_api_problem.rs"]
mod decode_api_problem;
#[path = "page_transport/page_contract.rs"]
mod page_contract;
#[path = "page_transport/transport.rs"]
mod transport;
#[path = "page_transport/transport_body.rs"]
mod transport_body;
#[path = "page_transport/transport_error.rs"]
mod transport_error;
#[path = "page_transport/transport_idempotency_key.rs"]
mod transport_idempotency_key;
#[path = "page_transport/transport_if_match.rs"]
mod transport_if_match;
#[path = "page_transport/transport_path.rs"]
mod transport_path;
#[path = "page_transport/transport_request.rs"]
mod transport_request;
#[path = "page_transport/transport_response.rs"]
mod transport_response;
#[path = "page_transport/transport_retry_after.rs"]
mod transport_retry_after;
#[path = "page_transport/transport_status.rs"]
mod transport_status;

pub use client_error::ClientError;
pub use decode_api_problem::decode_api_problem;
pub use page_contract::PageContract;
pub use transport::Transport;
pub use transport_body::TransportBody;
pub use transport_error::TransportError;
pub use transport_idempotency_key::*;
pub use transport_if_match::*;
pub use transport_path::*;
pub use transport_request::TransportRequest;
pub use transport_response::TransportResponse;
pub use transport_retry_after::*;
pub use transport_status::*;

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
