pub fn parse_trusted_proxy_ranges(
    value: crate::trusted_proxy_ranges_text_ref::TrustedProxyRangesTextRef<'_>,
) -> Result<
    crate::trusted_proxy_ranges::TrustedProxyRanges,
    crate::trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError,
> {
    if value.0.trim().is_empty() {
        return Ok(crate::trusted_proxy_ranges::TrustedProxyRanges::default());
    }
    let ranges = value
        .0
        .split(',')
        .map(str::trim)
        .map(|item| {
            crate::trusted_proxy_range::TrustedProxyRange::try_from(item.to_owned()).map_err(
                crate::trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError::Range,
            )
        })
        .collect::<Result<
            Vec<crate::trusted_proxy_range::TrustedProxyRange>,
            crate::trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError,
        >>()?;
    crate::trusted_proxy_ranges::TrustedProxyRanges::try_from(ranges)
        .map_err(crate::trusted_proxy_ranges_parse_error::TrustedProxyRangesParseError::Ranges)
}
