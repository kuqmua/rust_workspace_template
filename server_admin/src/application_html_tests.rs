#[allow(clippy::single_call_fn)] // test helper intentionally names repeated fixture construction at its sole call site
fn auth_with_headers(headers: http::HeaderMap) -> crate::AdminAuthReq {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect("1c2a7f54 auth_with_headers invariant must hold");
    let state = crate::auth_state(pool, constants_str::HTTP_LOCALHOST)
        .expect("adf9c06e auth_with_headers invariant must hold");
    crate::AdminAuthReq {
        headers: crate::HttpAdminHeaderMap::from(headers),
        peer: crate::AdminPeerAddr::from(crate::AdminSocketAddr::from(
            constants_str::VALUE_127_0_0_1_43210
                .parse::<std::net::SocketAddr>()
                .expect("0ce8ff47 auth_with_headers invariant must hold"),
        )),
        state: crate::SharedAdminAuthSvcStateArc::from(std::sync::Arc::new(state)),
    }
}

#[tokio::test]
async fn html_form_auth_rejects_cookie_without_trusted_origin() {
    let mut headers = http::HeaderMap::new();
    let _cookie = headers.insert(
        http::header::COOKIE,
        http::HeaderValue::from_static(constants_str::VALUE_BF7FDCFF),
    );
    assert!(matches!(
        crate::form_auth_impl::form_auth_impl(auth_with_headers(headers)),
        Err(crate::AdminError::Csrf)
    ));
}

#[tokio::test]
async fn admin_root_redirects_to_users() {
    let response = crate::root().await;
    assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static("/admin/users"))
    );
}

#[test]
fn successful_mutation_redirects_to_visible_server_feedback() {
    let response = crate::success_redirect_impl::success_redirect_impl(
        server_admin_contract::domain_types::AdminFrontendPath::Users,
    );
    assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static("/admin/users#saved"))
    );
}

#[test]
fn assignment_id_lists_reject_empty_entries() {
    let empty = crate::AdminHtmlFormText::try_from(String::new())
        .expect("1a37ef06 assignment_id_lists_reject_empty_entries invariant must hold");
    assert!(matches!(
        crate::role_ids_impl::role_ids_impl(&empty),
        Ok(_ids)
    ));
    assert!(matches!(
        crate::permission_ids_impl::permission_ids_impl(&empty),
        Ok(_ids)
    ));

    let malformed = crate::AdminHtmlFormText::try_from(String::from(constants_str::VALUE_A2688517))
        .expect("c2d76f19 assignment_id_lists_reject_empty_entries invariant must hold");
    assert!(matches!(
        crate::role_ids_impl::role_ids_impl(&malformed),
        Err(crate::AdminError::Validation)
    ));
    assert!(matches!(
        crate::permission_ids_impl::permission_ids_impl(&malformed),
        Err(crate::AdminError::Validation)
    ));
}

#[tokio::test]
async fn role_assignment_form_accepts_dynamic_checkbox_fields() {
    let request = http::Request::builder()
        .method(http::Method::POST)
        .header(
            http::header::CONTENT_TYPE,
            constants_str::APPLICATION_X_WWW_FORM_URLENCODED,
        )
        .body(axum::body::Body::from(constants_str::VALUE_08400B3F));
    let Ok(request) = request else {
        panic!("6f44bd85");
    };
    let result =
        <crate::AxumAdminForm<crate::UserRolesForm> as axum::extract::FromRequest<
            (),
        >>::from_request(request, &())
        .await;
    let Ok(crate::AxumAdminForm(form)) = result else {
        panic!("f639d7d1");
    };

    assert_eq!(i64::from(form.user_id), 7i64);
    assert_eq!(form.expected_role_ids.as_ref(), "1,2");
    assert_eq!(form.selected.len().get(), 2usize);
}

#[test]
fn selected_form_fields_reject_oversized_maps() {
    let values = (constants_usize::ZERO..=crate::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS)
        .map(|idx| {
            (
                crate::AdminHtmlFormKey::try_from(idx.to_string()).expect(
                    "763b9ec0 selected_form_fields_reject_oversized_maps invariant must hold",
                ),
                crate::AdminHtmlFormText::try_from(String::new()).expect(
                    "ef54739a selected_form_fields_reject_oversized_maps invariant must hold",
                ),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let Err(_error) = crate::StdAdminHtmlSelected::try_from(values) else {
        panic!("c86589e3");
    };
}
