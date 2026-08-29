pub use super::forwarded_proto_trust::ForwardedProtoTrust;
pub use super::http_content_security_policy::HttpContentSecurityPolicy;
pub use super::http_content_security_policy_error::HttpContentSecurityPolicyError;
pub use super::security_headers_layer::SecurityHeadersLayer;
use super::security_headers_service::SecurityHeadersService;
use super::security_headers_tower_layer::SecurityHeadersTowerLayer;
#[cfg(test)]
mod tests {
    #[test]
    fn content_security_policy_rejects_header_injection() {
        let _error =
            super::HttpContentSecurityPolicy::try_from(constants_str::VALUE_0E50D890.to_owned())
                .expect_err(constants_str::VALUE_1E8BE8A1);
    }
}

// Root-owned module compatibility wrappers.
mod forwarded_proto_trust {
    pub use super::super::forwarded_proto_trust::*;
}
mod http_content_security_policy {
    pub use super::super::http_content_security_policy::*;
}
mod http_content_security_policy_error {
    pub use super::super::http_content_security_policy_error::*;
}
mod security_headers_layer {
    pub use super::super::security_headers_layer::*;
}
mod security_headers_service {
    pub use super::super::security_headers_service::*;
}
mod security_headers_tower_layer {
    pub use super::super::security_headers_tower_layer::*;
}
