#[derive(
    Debug,
    Clone,
    Copy,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
)]
pub struct MaximumSizeOfHttpBodyInBytes(pub(super) usize);

impl TryFrom<usize> for MaximumSizeOfHttpBodyInBytes {
    type Error = super::MaximumSizeOfHttpBodyInBytesTryFromUsizeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == constants_usize::ZERO {
            Err(Self::Error::IsZero)
        } else {
            Ok(Self(value))
        }
    }
}

impl super::super::TryFromStdEnvVarOk for MaximumSizeOfHttpBodyInBytes {
    type Error = super::TryFromStdEnvVarOkMaximumSizeOfHttpBodyInBytesError;

    fn try_from_std_env_var_ok(v: super::super::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed: usize = super::super::parse_from_str_with_error(
            super::super::StdEnvVarOkRef::from(v.0.as_str()),
            |usize_parsing| Self::Error::UsizeParsing {
                usize_parsing: super::super::UsizeParseIntError::from(usize_parsing),
            },
        )?;
        Self::try_from(parsed).map_err(|maximum_size_of_http_body_in_bytes| {
            Self::Error::MaximumSizeOfHttpBodyInBytes {
                maximum_size_of_http_body_in_bytes,
            }
        })
    }
}
