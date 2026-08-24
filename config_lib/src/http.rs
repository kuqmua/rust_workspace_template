#[derive(
    Debug,
    Clone,
    Copy,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
)]
pub struct MaximumSizeOfHttpBodyInBytes(usize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout,
)]
pub enum MaximumSizeOfHttpBodyInBytesTryFromUsizeError {
    #[error("maximum size of http body in bytes must be greater than zero")]
    IsZero,
}
impl TryFrom<usize> for MaximumSizeOfHttpBodyInBytes {
    type Error = MaximumSizeOfHttpBodyInBytesTryFromUsizeError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(Debug, thiserror::Error, optimal_memory_layout::OptimalMemoryLayout)]
pub enum TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError {
    #[error("{maximum_size_of_http_body_in_bytes:?}")]
    MaximumSizeOfHttpBodyInBytes {
        maximum_size_of_http_body_in_bytes: MaximumSizeOfHttpBodyInBytesTryFromUsizeError,
    },
    #[error("{:?}", .usize_parsing)]
    UsizeParsing {
        usize_parsing: super::StdUsizeParsingError,
    },
}
impl super::TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: usize = super::parse_from_str_with_error(
            super::StdEnvVarOkRef::from(v.0.as_str()),
            |usize_parsing| Self::Error::UsizeParsing {
                usize_parsing: super::StdUsizeParsingError::from(usize_parsing),
            },
        )?;
        Self::try_from(parsed).map_err(|maximum_size_of_http_body_in_bytes| {
            Self::Error::MaximumSizeOfHttpBodyInBytes {
                maximum_size_of_http_body_in_bytes,
            }
        })
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq, newtype::AsRefOwned,
)]
pub struct ContentSecurityPolicy(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq, thiserror::Error,
)]
pub enum ContentSecurityPolicyError {
    #[error("content security policy must not be empty")]
    Empty,
    #[error("content security policy is too long or contains forbidden line breaks")]
    Invalid,
}
impl TryFrom<String> for ContentSecurityPolicy {
    type Error = ContentSecurityPolicyError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(Self::Error::Empty)
        } else if trimmed.len() > 4096usize || trimmed.contains(['\r', '\n']) {
            Err(Self::Error::Invalid)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }
}
impl super::TryFromStdEnvVarOk for ContentSecurityPolicy {
    type Error = ContentSecurityPolicyError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        Self::try_from(v.0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn http_limits_and_csp_validate_boundary_values() {
        let body_limit = <super::MaximumSizeOfHttpBodyInBytes as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
            super::super::StdEnvVarOk::try_from(String::from("1")).expect("42f6d81c http_limits_and_csp_validate_boundary_values invariant must hold"),
        )
        .expect("85a01fbd http_limits_and_csp_validate_boundary_values invariant must hold");
        assert_eq!(body_limit.0, 1usize);
        assert!(matches!(
            super::ContentSecurityPolicy::try_from(String::from("\n")),
            Err(super::ContentSecurityPolicyError::Empty)
        ));
    }
}
