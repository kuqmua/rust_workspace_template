#[must_use]
pub fn proxy_path_matches_prefix(
    path: &super::HttpProxyPath,
    prefix: super::HttpAllowedPathPrefixRef<'_>,
) -> super::HttpProxyPathPrefixMatch {
    super::HttpProxyPathPrefixMatch::from(
        path.as_ref() == prefix.0
            || path
                .as_ref()
                .strip_prefix(prefix.0)
                .is_some_and(|suffix| suffix.starts_with('/')),
    )
}
