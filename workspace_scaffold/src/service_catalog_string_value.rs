pub(super) fn string_value(
    line: super::ScaffoldTextRef<'_>,
    key: super::ScaffoldTextRef<'_>,
) -> Result<Option<super::ScaffoldText>, super::ScaffoldError> {
    line.0
        .strip_prefix(key.0)
        .and_then(|value| value.trim().strip_prefix('='))
        .map(str::trim)
        .and_then(|value| value.strip_prefix('"'))
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_owned)
        .map(super::ScaffoldText::try_from)
        .transpose()
        .map_err(|_error| super::ScaffoldError::Catalog)
}
