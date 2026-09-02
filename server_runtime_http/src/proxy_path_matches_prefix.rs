#[must_use]
pub fn proxy_path_matches_prefix(
    http_proxy_path: &crate::http_proxy_path::HttpProxyPath,
    http_allowed_path_prefix_ref: crate::http_allowed_path_prefix_ref::HttpAllowedPathPrefixRef<'_>,
) -> crate::http_proxy_path_prefix_match::HttpProxyPathPrefixMatch {
    let prefix_text = http_allowed_path_prefix_ref.get();
    crate::http_proxy_path_prefix_match::HttpProxyPathPrefixMatch::from(
        http_proxy_path.as_ref() == prefix_text
            || http_proxy_path
                .as_ref()
                .strip_prefix(prefix_text)
                .is_some_and(|suffix| suffix.starts_with('/')),
    )
}
