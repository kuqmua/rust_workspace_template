pub fn parse_trusted_proxy_ranges(
    value: super::TrustedProxyRangesTextRef<'_>,
) -> Result<super::TrustedProxyRanges, super::TrustedProxyRangesParseError> {
    if value.0.trim().is_empty() {
        return Ok(super::TrustedProxyRanges::default());
    }
    let ranges = value
        .0
        .split(',')
        .map(str::trim)
        .map(|item| {
            super::TrustedProxyRange::try_from(item.to_owned())
                .map_err(super::TrustedProxyRangesParseError::Range)
        })
        .collect::<Result<Vec<super::TrustedProxyRange>, super::TrustedProxyRangesParseError>>()?;
    super::TrustedProxyRanges::try_from(ranges).map_err(super::TrustedProxyRangesParseError::Ranges)
}
