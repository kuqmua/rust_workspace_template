#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::DerefInner)]
pub struct HttpContentSecurityPolicy(pub(super) http::HeaderValue);

impl TryFrom<String> for HttpContentSecurityPolicy {
    type Error = super::HttpContentSecurityPolicyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_4_096 {
            return Err(super::HttpContentSecurityPolicyError);
        }
        http::HeaderValue::try_from(value)
            .map(Self)
            .map_err(|_error| super::HttpContentSecurityPolicyError)
    }
}
