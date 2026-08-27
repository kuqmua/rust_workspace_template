#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{
    ChronoEastFixedOffset, ChronoFixedOffsetError, I32ParseIntError, StdEnvVarOk, StdEnvVarOkRef,
    TimezoneSeconds, TryFromStdEnvVarOk, TryFromStdEnvVarOkTimezoneError, parse_east_fixed_offset,
    parse_from_str_with_error,
};

#[derive(
    Debug,
    Clone,
    Copy,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
)]
pub struct ChronoTimezone(pub(super) chrono::FixedOffset);
impl From<ChronoEastFixedOffset> for ChronoTimezone {
    fn from(value: ChronoEastFixedOffset) -> Self {
        Self(value.0)
    }
}
impl TryFrom<chrono::FixedOffset> for ChronoTimezone {
    type Error = ChronoFixedOffsetError;
    fn try_from(value: chrono::FixedOffset) -> Result<Self, Self::Error> {
        parse_east_fixed_offset(TimezoneSeconds(value.local_minus_utc())).map(|_| Self(value))
    }
}
impl TryFromStdEnvVarOk for ChronoTimezone {
    type Error = TryFromStdEnvVarOkTimezoneError;
    fn try_from_std_env_var_ok(v: StdEnvVarOk) -> Result<Self, Self::Error> {
        let i32_v = TimezoneSeconds::from(parse_from_str_with_error::<i32, _, _>(
            StdEnvVarOkRef::from(v.0.as_str()),
            |i32_parsing| Self::Error::I32Parsing {
                i32_parsing: I32ParseIntError::from(i32_parsing),
            },
        )?);
        parse_east_fixed_offset(i32_v)
            .map_err(|chrono_fixed_offset| Self::Error::ChronoFixedOffset {
                chrono_fixed_offset,
            })
            .map(Self::from)
    }
}
