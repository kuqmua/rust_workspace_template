pub(super) fn parse_east_fixed_offset(
    v: crate::timezone_seconds::TimezoneSeconds,
) -> Result<
    crate::chrono_timezone::ChronoTimezone,
    crate::chrono_fixed_offset_error::ChronoFixedOffsetError,
> {
    chrono::FixedOffset::east_opt(v.0)
        .map(crate::chrono_timezone::ChronoTimezone)
        .ok_or_else(|| {
            crate::chrono_fixed_offset_error::ChronoFixedOffsetError::from(
                constants_str::catalog::CONFIG_TIMEZONE_NOT_EAST_MSG,
            )
        })
}
