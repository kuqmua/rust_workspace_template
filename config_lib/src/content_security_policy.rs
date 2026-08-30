#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq, newtype::AsRefOwned,
)]
pub struct ContentSecurityPolicy(String);

impl TryFrom<String> for ContentSecurityPolicy {
    type Error = crate::content_security_policy_error::ContentSecurityPolicyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(Self::Error::Empty)
        } else if trimmed.len() > constants_usize::VALUE_4_096 || trimmed.contains(['\r', '\n']) {
            Err(Self::Error::Invalid)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for ContentSecurityPolicy {
    type Error = crate::content_security_policy_error::ContentSecurityPolicyError;

    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        Self::try_from(String::from(v))
    }
}
