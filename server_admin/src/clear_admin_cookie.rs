#[must_use]
pub fn clear_admin_cookie(
    kind: crate::admin_cookie_kind::AdminCookieKind,
    secure: crate::admin_cookie_secure::AdminCookieSecure,
) -> crate::std_admin_cookie::StdAdminCookie {
    crate::build_admin_cookie::build_admin_cookie(
        kind,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
        ),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(0),
        secure,
    )
}
