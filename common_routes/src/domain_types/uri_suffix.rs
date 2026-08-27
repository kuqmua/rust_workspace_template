use super::{AxumHttpUriRef, UriSuffixRef};

#[allow(clippy::single_call_fn)] // keeps route text construction consistent for path-only and path+query URIs
pub(super) fn uri_suffix(uri: AxumHttpUriRef<'_>) -> UriSuffixRef<'_> {
    UriSuffixRef::from(
        uri.0
            .path_and_query()
            .map_or_else(|| uri.0.path(), |v| v.as_str()),
    )
}
