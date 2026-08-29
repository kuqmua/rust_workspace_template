#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
)]
pub struct ChronoTimezone(pub(super) chrono::FixedOffset);
impl TryFrom<chrono::FixedOffset> for ChronoTimezone {
    type Error = crate::chrono_fixed_offset_error::ChronoFixedOffsetError;
    fn try_from(value: chrono::FixedOffset) -> Result<Self, Self::Error> {
        crate::parse_east_fixed_offset::parse_east_fixed_offset(
            crate::timezone_seconds::TimezoneSeconds(value.local_minus_utc()),
        )
    }
}
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for ChronoTimezone {
    type Error = crate::try_from_std_env_var_ok_timezone_error::TryFromStdEnvVarOkTimezoneError;
    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        let i32_v = crate::timezone_seconds::TimezoneSeconds::from(
            crate::parse_from_str_with_error::parse_from_str_with_error::<i32, _, _>(
                crate::std_env_var_ok_ref::StdEnvVarOkRef::from(v.0.as_str()),
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
