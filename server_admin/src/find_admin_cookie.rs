#[must_use]
pub fn find_admin_cookie(
    headers: crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'_>,
    kind: crate::admin_cookie_kind::AdminCookieKind,
) -> Option<server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>> {
    match server_runtime_http::resolve_unique_cookie::resolve_unique_cookie(
        server_runtime_http::http_cookie_headers_ref::HttpCookieHeadersRef::from(headers.get()),
        server_runtime_http::http_cookie_name_ref::HttpCookieNameRef::from(kind.name().as_ref()),
    ) {
        server_runtime_http::cookie_resolution::CookieResolution::Resolved(value) => Some(
            server_admin_core::std_admin_str_ref::StdAdminStrRef::from(<&str>::from(value)),
        ),
        server_runtime_http::cookie_resolution::CookieResolution::Invalid
        | server_runtime_http::cookie_resolution::CookieResolution::Missing => None,
    }
}
