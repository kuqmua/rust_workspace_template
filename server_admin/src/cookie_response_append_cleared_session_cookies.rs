#![allow(clippy::single_call_fn)] // sign-out flows own clearing session cookies

pub(super) fn append_cleared_session_cookies(
    response: &mut super::AxumAdminResponse,
    state: &super::AdminAuthSvcState,
) -> Result<(), super::AdminError> {
    [
        super::super::AdminCookieKind::Access,
        super::super::AdminCookieKind::Refresh,
        super::super::AdminCookieKind::Csrf,
    ]
    .into_iter()
    .try_for_each(|kind| {
        let cookie = super::super::clear_admin_cookie(kind, state.cookie_secure);
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                response
                    .0
                    .headers_mut()
                    .append(http::header::SET_COOKIE, header)
            })
            .map(drop)
            .map_err(|error| {
                super::AdminError::header(super::HttpAdminHeaderValueError::from(error))
            })
    })
}
