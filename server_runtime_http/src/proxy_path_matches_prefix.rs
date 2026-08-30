#[must_use]
pub fn proxy_path_matches_prefix(
    path: &crate::http_proxy_path::HttpProxyPath,
    prefix: crate::http_allowed_path_prefix_ref::HttpAllowedPathPrefixRef<'_>,
) -> crate::http_proxy_path_prefix_match::HttpProxyPathPrefixMatch {
    let prefix_text = prefix.get();
    crate::http_proxy_path_prefix_match::HttpProxyPathPrefixMatch::from(
        path.as_ref() == prefix_text
            || path
                .as_ref()
                .strip_prefix(prefix_text)
                .is_some_and(|suffix| suffix.starts_with('/')),
    )
}
