#[tokio::test]
async fn test_html_form_auth_rejects_cookie_without_trusted_origin() {
    let mut headers = http::HeaderMap::new();
    let _cookie = headers.insert(
        http::header::COOKIE,
        http::HeaderValue::from_static(constants_str::VALUE_BF7FDCFF),
    );
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect(constants_str::DIAGNOSTIC_1C2A7F54);
    let state = crate::application_tests_helper::auth_state(pool, constants_str::HTTP_LOCALHOST)
        .expect(constants_str::DIAGNOSTIC_ADF9C06E);
    let request = crate::admin_auth_req::AdminAuthReq::new(
        crate::http_admin_header_map::HttpAdminHeaderMap::from(headers),
        crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc::from(
            std::sync::Arc::new(state),
        ),
        crate::admin_peer_addr::AdminPeerAddr::from(
            server_admin_core::admin_socket_addr::AdminSocketAddr::from(
                constants_str::VALUE_127_0_0_1_43210
                    .parse::<std::net::SocketAddr>()
                    .expect(constants_str::DIAGNOSTIC_0CE8FF47),
            ),
        ),
    );
    assert!(matches!(
        crate::form_auth_impl::form_auth_impl(request),
        Err(crate::admin_error::AdminError::Csrf)
    ));
}

#[tokio::test]
async fn test_admin_root_redirects_to_users() {
    let response = crate::root::root().await;
    assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static(
            constants_str::VALUE_074B6E5E
        ))
    );
}

#[test]
fn test_successful_mutation_redirects_to_visible_server_feedback() {
    let response = crate::success_redirect_impl::success_redirect_impl(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users,
    );
    assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static(
            constants_str::VALUE_B6E7A6E1
        ))
    );
}

#[test]
fn test_assignment_id_lists_reject_empty_entries() {
    let empty = crate::admin_html_form_text::AdminHtmlFormText::try_from(String::new())
        .expect(constants_str::DIAGNOSTIC_1A37EF06);
    assert!(matches!(
        crate::role_ids_impl::role_ids_impl(&empty),
        Ok(_ids)
    ));
    assert!(matches!(
        crate::permission_ids_impl::permission_ids_impl(&empty),
        Ok(_ids)
    ));

    let malformed = crate::admin_html_form_text::AdminHtmlFormText::try_from(String::from(
        constants_str::VALUE_A2688517,
    ))
    .expect(constants_str::DIAGNOSTIC_C2D76F19);
    assert!(matches!(
        crate::role_ids_impl::role_ids_impl(&malformed),
        Err(crate::admin_error::AdminError::Validation)
    ));
    assert!(matches!(
        crate::permission_ids_impl::permission_ids_impl(&malformed),
        Err(crate::admin_error::AdminError::Validation)
    ));
}

#[tokio::test]
async fn test_role_assignment_form_accepts_dynamic_checkbox_fields() {
    let request = http::Request::builder()
        .method(http::Method::POST)
        .header(
            http::header::CONTENT_TYPE,
            constants_str::APPLICATION_X_WWW_FORM_URLENCODED,
        )
        .body(axum::body::Body::from(constants_str::VALUE_08400B3F));
    let Ok(request) = request else {
        std::panic::panic_any(constants_str::PANIC_6F44BD85);
    };
    let result =
        <crate::axum_admin_form::AxumAdminForm<crate::user_roles_form::UserRolesForm> as axum::extract::FromRequest<
            (),
        >>::from_request(request, &())
        .await;
    let Ok(form) = result else {
        std::panic::panic_any(constants_str::PANIC_F639D7D1);
    };
    let form = form.into_inner();

    assert_eq!(i64::from(*form.get_user_id()), 7i64);
    assert_eq!(
        form.get_expected_role_ids().as_str(),
        constants_str::VALUE_17F8AF97
    );
    assert_eq!(form.get_selected().len().get(), 2usize);
}

#[test]
fn test_selected_form_fields_reject_oversized_maps() {
    let values = (constants_usize::ZERO
        ..=crate::admin_html_form_selected_max_items::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS)
        .map(|idx| {
            (
                crate::admin_html_form_key::AdminHtmlFormKey::try_from(idx.to_string())
                    .expect(constants_str::DIAGNOSTIC_763B9EC0),
                crate::admin_html_form_text::AdminHtmlFormText::try_from(String::new())
                    .expect(constants_str::DIAGNOSTIC_EF54739A),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let Err(_error) = crate::std_admin_html_selected::StdAdminHtmlSelected::try_from(values) else {
        std::panic::panic_any(constants_str::PANIC_C86589E3);
    };
}
