#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct HttpContentSecurityPolicy(http::HeaderValue);

impl TryFrom<String> for HttpContentSecurityPolicy {
    type Error = crate::http_content_security_policy_error::HttpContentSecurityPolicyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_4_096 {
            return Err(
                crate::http_content_security_policy_error::HttpContentSecurityPolicyError::InvalidHeaderValue,
            );
        }
        http::HeaderValue::try_from(value)
            .map(Self)
            .map_err(|_error| {
                crate::http_content_security_policy_error::HttpContentSecurityPolicyError::InvalidHeaderValue
            })
    }
}
