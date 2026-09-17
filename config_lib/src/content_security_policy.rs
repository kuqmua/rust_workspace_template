#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
)]
pub struct ContentSecurityPolicy(
    bounded_types::bounded_string::BoundedString<1usize, 4_096usize, false>,
);

impl TryFrom<String> for ContentSecurityPolicy {
    type Error = crate::content_security_policy_error::ContentSecurityPolicyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(Self::Error::Empty)
        } else if trimmed.len() > constants_usize::VALUE_4_096 || trimmed.contains(['\r', '\n']) {
            Err(Self::Error::Invalid)
        } else {
            bounded_types::bounded_string::BoundedString::try_from(trimmed.to_owned())
                .map(Self)
                .map_err(|source| match source {
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                        ..
                    } => Self::Error::Invalid,
                    bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                        ..
                    } => Self::Error::Empty,
                })
        }
    }
}

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for ContentSecurityPolicy {
    type Error = crate::content_security_policy_error::ContentSecurityPolicyError;

    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        Self::try_from(String::from(std_env_var_ok))
    }
}
