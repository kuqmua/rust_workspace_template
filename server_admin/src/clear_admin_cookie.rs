#[must_use]
pub fn clear_admin_cookie(
    admin_cookie_kind: crate::admin_cookie_kind::AdminCookieKind,
    runtime_admin_cookie_secure: crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure,
) -> crate::std_admin_cookie::StdAdminCookie {
    crate::build_admin_cookie::build_admin_cookie(
        admin_cookie_kind,
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
        ),
        crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds::from(0),
        runtime_admin_cookie_secure,
    )
}
