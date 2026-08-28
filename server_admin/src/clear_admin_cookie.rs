use crate::*;
#[must_use]
pub fn clear_admin_cookie(kind: AdminCookieKind, secure: AdminCookieSecure) -> StdAdminCookie {
    build_admin_cookie(
        kind,
        StdAdminStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        AdminCookieMaxAgeSeconds::from(0),
        secure,
    )
}
