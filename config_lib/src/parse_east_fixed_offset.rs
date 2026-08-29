use super::{ChronoFixedOffsetError, ChronoTimezone, TimezoneSeconds};

pub(super) fn parse_east_fixed_offset(
    v: TimezoneSeconds,
) -> Result<ChronoTimezone, ChronoFixedOffsetError> {
    chrono::FixedOffset::east_opt(v.0)
        .map(ChronoTimezone)
        .ok_or_else(|| ChronoFixedOffsetError::from(constants_str::CONFIG_TIMEZONE_NOT_EAST_MSG))
}
