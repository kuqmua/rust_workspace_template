pub(super) fn origin_is_present_and_allowed(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
) -> super::super::StdAdminBool {
    super::super::StdAdminBool::from(bool::from(
        server_runtime_http::domain_types::request_origin_allowed(
            server_runtime_http::domain_types::HttpOriginHeadersRef::from(headers.get()),
            &state.allowed_origins,
        ),
    ))
}
