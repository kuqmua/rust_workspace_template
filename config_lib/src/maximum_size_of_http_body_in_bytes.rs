#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_generate_accessor_traits_for_struct_fields_generate_accessor_trait::GenerateAccessorTrait,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
)]
pub struct MaximumSizeOfHttpBodyInBytes(usize);

impl TryFrom<usize> for MaximumSizeOfHttpBodyInBytes {
    type Error = crate::maximum_size_of_http_body_in_bytes_try_from_usize_error::MaximumSizeOfHttpBodyInBytesTryFromUsizeError;

    fn try_from(usize: usize) -> Result<Self, Self::Error> {
        if usize == constants_usize::ZERO {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(usize))
        }
    }
}

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = crate::try_from_std_env_var_ok_maximum_size_of_http_body_in_bytes_error::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError;

    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        let parsed: usize = crate::parse_from_str_with_error::parse_from_str_with_error(
            crate::std_env_var_ok_ref::StdEnvVarOkRef::from(std_env_var_ok.as_ref()),
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
