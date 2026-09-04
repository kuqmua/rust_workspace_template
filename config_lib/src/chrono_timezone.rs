#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_generate_accessor_traits_for_struct_fields_generate_accessor_trait::GenerateAccessorTrait,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_deref_inner::DerefInner,
)]
pub struct ChronoTimezone(chrono::FixedOffset);
impl TryFrom<chrono::FixedOffset> for ChronoTimezone {
    type Error = crate::chrono_fixed_offset_error::ChronoFixedOffsetError;
    fn try_from(value: chrono::FixedOffset) -> Result<Self, Self::Error> {
        crate::parse_east_fixed_offset::parse_east_fixed_offset(
            crate::timezone_seconds::TimezoneSeconds::from(value.local_minus_utc()),
        )
    }
}
impl TryFrom<crate::timezone_seconds::TimezoneSeconds> for ChronoTimezone {
    type Error = crate::chrono_fixed_offset_error::ChronoFixedOffsetError;
    fn try_from(value: crate::timezone_seconds::TimezoneSeconds) -> Result<Self, Self::Error> {
        chrono::FixedOffset::east_opt(*value)
            .map(Self)
            .ok_or_else(|| {
                crate::chrono_fixed_offset_error::ChronoFixedOffsetError::from(
                    constants_str::CONFIG_TIMEZONE_NOT_EAST_MSG,
                )
            })
    }
}
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for ChronoTimezone {
    type Error = crate::try_from_std_env_var_ok_timezone_error::TryFromStdEnvVarOkTimezoneError;
    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        let i32_v = crate::timezone_seconds::TimezoneSeconds::from(
            crate::parse_from_str_with_error::parse_from_str_with_error::<i32, _, _>(
                crate::std_env_var_ok_ref::StdEnvVarOkRef::from(std_env_var_ok.as_ref()),
                |i32_parsing| Self::Error::I32Parsing {
                    i32_parsing: crate::i32_parse_int_error::I32ParseIntError::from(i32_parsing),
                },
            )?,
        );
        crate::parse_east_fixed_offset::parse_east_fixed_offset(i32_v).map_err(
            |chrono_fixed_offset| Self::Error::ChronoFixedOffset {
                chrono_fixed_offset,
            },
        )
    }
}
