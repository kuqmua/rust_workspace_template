#[test]
fn test_successful_form_action_redirects_to_saved_page() {
    let path = server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles;
    let response = crate::action_result_impl::action_result_impl(
        Ok(crate::axum_admin_response::AxumAdminResponse::from(
            axum::response::Response::new(axum::body::Body::empty()),
        )),
        path,
    );
    assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
    assert!(
        response
            .headers()
            .get(http::header::LOCATION)
            .is_some_and(|location| location
                == format!("{}{}", path.get(), constants_str::ADMIN_HTML_SAVED_FRAGMENT).as_str())
    );
}

#[test]
fn test_failed_form_action_returns_error_without_redirect() {
    let response = crate::action_result_impl::action_result_impl(
        Err(crate::admin_error::AdminError::Validation),
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles,
    );
    assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
    assert!(response.headers().get(http::header::LOCATION).is_none());
    assert!(
        response
            .headers()
            .get(http::header::CONTENT_TYPE)
            .is_some_and(
                |content_type| content_type == constants_str::APPLICATION_PROBLEM_PLUS_JSON
            )
    );
}
