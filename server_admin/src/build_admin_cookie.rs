#[must_use]
pub fn build_admin_cookie(
    kind: crate::admin_cookie_kind::AdminCookieKind,
    value: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    max_age: crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds,
    secure: crate::admin_cookie_secure::AdminCookieSecure,
) -> crate::std_admin_cookie::StdAdminCookie {
    let http_only = if matches!(kind, crate::admin_cookie_kind::AdminCookieKind::Csrf) {
        constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX
    } else {
        constants_str::catalog::HTTPONLY
    };
    let secure_attr = if secure.0 {
        constants_str::catalog::SECURE
    } else {
        constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    crate::std_admin_cookie::StdAdminCookie::try_from(format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attr}",
        kind.name().as_ref(),
        value.as_ref(),
        max_age.0
    ))
    .unwrap_or_else(crate::std_admin_cookie::StdAdminCookie::from)
}
