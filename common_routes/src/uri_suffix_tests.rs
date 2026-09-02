#[cfg(test)]
pub(super) fn uri_suffix(
    axum_http_uri_ref: crate::axum_http_uri_ref::AxumHttpUriRef<'_>,
) -> crate::uri_suffix_ref::UriSuffixRef<'_> {
    crate::uri_suffix_ref::UriSuffixRef::from(
        axum_http_uri_ref
            .path_and_query()
            .map_or_else(|| axum_http_uri_ref.path(), |v| v.as_str()),
    )
}
