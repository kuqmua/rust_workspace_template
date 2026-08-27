use super::*;
#[must_use]
pub fn find_admin_cookie(
    headers: HttpAdminHeaderMapRef<'_>,
    kind: AdminCookieKind,
) -> Option<super::super::StdAdminStrRef<'_>> {
    match server_runtime_http::domain_types::resolve_unique_cookie(
        server_runtime_http::domain_types::HttpCookieHeadersRef::from(headers.0),
        server_runtime_http::domain_types::HttpCookieNameRef::from(kind.name().as_ref()),
    ) {
        server_runtime_http::domain_types::CookieResolution::Resolved(value) => {
            Some(super::super::StdAdminStrRef::from(<&str>::from(value)))
        }
        server_runtime_http::domain_types::CookieResolution::Invalid
        | server_runtime_http::domain_types::CookieResolution::Missing => None,
    }
}
