use super::{AxumHttpUriRef, UriSuffixRef};

#[cfg(test)]
pub(super) fn uri_suffix(uri: AxumHttpUriRef<'_>) -> UriSuffixRef<'_> {
    UriSuffixRef::from(
        uri.0
            .path_and_query()
            .map_or_else(|| uri.0.path(), |v| v.as_str()),
    )
}
