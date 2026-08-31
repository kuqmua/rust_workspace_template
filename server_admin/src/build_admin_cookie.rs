#[must_use]
pub fn build_admin_cookie(
    kind: crate::admin_cookie_kind::AdminCookieKind,
    value: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    max_age: crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds,
    secure: crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure,
) -> crate::std_admin_cookie::StdAdminCookie {
    let http_only = if matches!(kind, crate::admin_cookie_kind::AdminCookieKind::Csrf) {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    } else {
        constants_str::HTTPONLY
    };
    let secure_attr = if *secure.get_inner() {
        constants_str::SECURE
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    crate::std_admin_cookie::StdAdminCookie::try_from(format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attr}",
        kind.name().as_ref(),
        value.as_ref(),
        max_age.get_inner()
    ))
    .unwrap_or_else(crate::std_admin_cookie::StdAdminCookie::from)
}
