#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    Debug,
    Clone,
    Copy,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
)]
pub struct MaximumSizeOfHttpBodyInBytes(usize);

impl TryFrom<usize> for MaximumSizeOfHttpBodyInBytes {
    type Error = crate::maximum_size_of_http_body_in_bytes_try_from_usize_error::MaximumSizeOfHttpBodyInBytesTryFromUsizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == constants_usize::ZERO {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError;

    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: usize = crate::parse_from_str_with_error::parse_from_str_with_error(
            crate::std_env_var_ok_ref::StdEnvVarOkRef::from(v.as_ref()),
            |usize_parsing| Self::Error::UsizeParsing {
                usize_parsing: crate::usize_parse_int_error::UsizeParseIntError::from(
                    usize_parsing,
                ),
            },
        )?;
        Self::try_from(parsed).map_err(|maximum_size_of_http_body_in_bytes| {
            Self::Error::MaximumSizeOfHttpBodyInBytes {
                maximum_size_of_http_body_in_bytes,
            }
        })
    }
}
