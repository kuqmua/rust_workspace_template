#[must_use]
pub fn proxy_path_matches_prefix(
    path: &crate::http_proxy_path::HttpProxyPath,
    prefix: crate::http_allowed_path_prefix_ref::HttpAllowedPathPrefixRef<'_>,
) -> crate::http_proxy_path_prefix_match::HttpProxyPathPrefixMatch {
    crate::http_proxy_path_prefix_match::HttpProxyPathPrefixMatch::from(
        path.as_ref() == prefix.0
            || path
                .as_ref()
                .strip_prefix(prefix.0)
                .is_some_and(|suffix| suffix.starts_with('/')),
    )
}
