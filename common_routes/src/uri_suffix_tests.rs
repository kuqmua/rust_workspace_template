#[cfg(test)]
pub(super) fn uri_suffix(
    uri: crate::axum_http_uri_ref::AxumHttpUriRef<'_>,
) -> crate::uri_suffix_ref::UriSuffixRef<'_> {
    crate::uri_suffix_ref::UriSuffixRef::from(
        uri.path_and_query()
            .map_or_else(|| uri.path(), |v| v.as_str()),
    )
}
