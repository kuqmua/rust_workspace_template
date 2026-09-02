pub(super) fn parse_east_fixed_offset(
    timezone_seconds: crate::timezone_seconds::TimezoneSeconds,
) -> Result<
    crate::chrono_timezone::ChronoTimezone,
    crate::chrono_fixed_offset_error::ChronoFixedOffsetError,
> {
    crate::chrono_timezone::ChronoTimezone::try_from(timezone_seconds)
}
