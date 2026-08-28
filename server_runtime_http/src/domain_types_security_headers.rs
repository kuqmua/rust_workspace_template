#[path = "forwarded_proto_trust.rs"]
mod forwarded_proto_trust;
#[path = "http_content_security_policy.rs"]
mod http_content_security_policy;
#[path = "http_content_security_policy_error.rs"]
mod http_content_security_policy_error;
#[path = "security_headers_layer.rs"]
mod security_headers_layer;
#[path = "security_headers_service.rs"]
mod security_headers_service;
#[path = "security_headers_tower_layer.rs"]
mod security_headers_tower_layer;

pub use forwarded_proto_trust::ForwardedProtoTrust;
pub use http_content_security_policy::HttpContentSecurityPolicy;
pub use http_content_security_policy_error::HttpContentSecurityPolicyError;
pub use security_headers_layer::SecurityHeadersLayer;
use security_headers_service::SecurityHeadersService;
use security_headers_tower_layer::SecurityHeadersTowerLayer;

#[cfg(test)]
mod tests {
    #[test]
    fn content_security_policy_rejects_header_injection() {
        let _error =
            super::HttpContentSecurityPolicy::try_from(constants_str::VALUE_0E50D890.to_owned())
                .expect_err(constants_str::VALUE_1E8BE8A1);
    }
}
