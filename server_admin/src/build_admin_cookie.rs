#[must_use]
pub fn build_admin_cookie(
    admin_cookie_kind: crate::admin_cookie_kind::AdminCookieKind,
    std_admin_str_ref: server_admin_core::std_admin_str_ref::StdAdminStrRef<'_>,
    admin_cookie_max_age_seconds: crate::admin_cookie_max_age_seconds::AdminCookieMaxAgeSeconds,
    runtime_admin_cookie_secure: crate::runtime_admin_cookie_secure::RuntimeAdminCookieSecure,
) -> crate::std_admin_cookie::StdAdminCookie {
    let http_only = if matches!(
        admin_cookie_kind,
        crate::admin_cookie_kind::AdminCookieKind::Csrf
    ) {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    } else {
        constants_str::HTTPONLY
    };
    let secure_attr = if *runtime_admin_cookie_secure.get_inner() {
        constants_str::SECURE
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    crate::std_admin_cookie::StdAdminCookie::try_from(format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attr}",
        admin_cookie_kind.name().as_ref(),
        std_admin_str_ref.as_ref(),
        admin_cookie_max_age_seconds.get_inner()
    ))
    .unwrap_or_else(crate::std_admin_cookie::StdAdminCookie::from)
}
