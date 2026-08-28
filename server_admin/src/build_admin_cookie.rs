use crate::*;
#[must_use]
pub fn build_admin_cookie(
    kind: AdminCookieKind,
    value: StdAdminStrRef<'_>,
    max_age: AdminCookieMaxAgeSeconds,
    secure: AdminCookieSecure,
) -> StdAdminCookie {
    let http_only = if matches!(kind, AdminCookieKind::Csrf) {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    } else {
        constants_str::HTTPONLY
    };
    let secure_attr = if secure.0 {
        constants_str::SECURE
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    StdAdminCookie::try_from(format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attr}",
        kind.name().as_ref(),
        value.as_ref(),
        max_age.0
    ))
    .unwrap_or_else(StdAdminCookie::from)
}
