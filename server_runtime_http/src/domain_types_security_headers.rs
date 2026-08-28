pub use crate::forwarded_proto_trust::ForwardedProtoTrust;
pub use crate::http_content_security_policy::HttpContentSecurityPolicy;
pub use crate::http_content_security_policy_error::HttpContentSecurityPolicyError;
pub use crate::security_headers_layer::SecurityHeadersLayer;
use crate::security_headers_service::SecurityHeadersService;
use crate::security_headers_tower_layer::SecurityHeadersTowerLayer;

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
    pub use crate::forwarded_proto_trust::*;
}
mod http_content_security_policy {
    pub use crate::http_content_security_policy::*;
}
mod http_content_security_policy_error {
    pub use crate::http_content_security_policy_error::*;
}
mod security_headers_layer {
    pub use crate::security_headers_layer::*;
}
mod security_headers_service {
    pub use crate::security_headers_service::*;
}
mod security_headers_tower_layer {
    pub use crate::security_headers_tower_layer::*;
}
