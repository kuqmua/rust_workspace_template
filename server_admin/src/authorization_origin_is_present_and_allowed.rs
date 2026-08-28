pub(crate) fn authorization_origin_is_present_and_allowed(
    state: &crate::AdminAuthSvcState,
    headers: crate::HttpAdminHeaderMapRef<'_>,
) -> crate::StdAdminBool {
    crate::StdAdminBool::from(bool::from(
        server_runtime_http::domain_types::resolve_request_origin_allowed(
            server_runtime_http::domain_types::HttpOriginHeadersRef::from(headers.get()),
            &state.allowed_origins,
        ),
    ))
}
