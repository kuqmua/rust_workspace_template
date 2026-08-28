use super::{ChronoEastFixedOffset, ChronoFixedOffsetError, TimezoneSeconds};

pub(super) fn parse_east_fixed_offset(
    v: TimezoneSeconds,
) -> Result<ChronoEastFixedOffset, ChronoFixedOffsetError> {
    chrono::FixedOffset::east_opt(v.0)
        .map(ChronoEastFixedOffset)
        .ok_or_else(|| ChronoFixedOffsetError::from(constants_str::CONFIG_TIMEZONE_NOT_EAST_MSG))
}
