pub(crate) fn authorization_origin_is_present_and_allowed(
    admin_auth_svc_state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    http_admin_header_map_ref: crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'_>,
) -> server_admin_core::std_admin_bool::StdAdminBool {
    server_admin_core::std_admin_bool::StdAdminBool::from(bool::from(
        server_runtime_http::resolve_request_origin_allowed::resolve_request_origin_allowed(
            server_runtime_http::http_origin_headers_ref::HttpOriginHeadersRef::from(
                http_admin_header_map_ref.get(),
            ),
            admin_auth_svc_state.get_allowed_origins(),
        ),
    ))
}
