pub(super) fn service_catalog_string_value(
    line: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
    key: crate::scaffold_text_ref::ScaffoldTextRef<'_>,
) -> Result<Option<crate::scaffold_text::ScaffoldText>, crate::scaffold_error::ScaffoldError> {
    line.0
        .strip_prefix(key.0)
        .and_then(|value| value.trim().strip_prefix('='))
        .map(str::trim)
        .and_then(|value| value.strip_prefix('"'))
        .and_then(|value| value.strip_suffix('"'))
        .map(str::to_owned)
        .map(crate::scaffold_text::ScaffoldText::try_from)
        .transpose()
        .map_err(|_error| crate::scaffold_error::ScaffoldError::Catalog)
}
