fn env<Value>(value: &str) -> Value
where
    Value: config_lib::TryFromStdEnvVarOk,
    Value::Error: std::fmt::Debug,
{
    Value::try_from_std_env_var_ok(
        config_lib::StdEnvVarOk::try_from(value.to_owned()).expect("82c951d4"),
    )
    .expect("135a22e8")
}

fn auth_with_headers(headers: http::HeaderMap) -> super::super::AdminAuthReq {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(str_constants::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect("1c2a7f54");
    let state = super::super::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool),
        &env(str_constants::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES),
        &env(str_constants::VALUE_900),
        &env(str_constants::VALUE_3600),
        &env(str_constants::VALUE_20),
        &env(str_constants::VALUE_2),
        &env(str_constants::VALUE_10),
        &env(str_constants::VALUE_1),
        &env(str_constants::FALSE),
        &env(str_constants::INTEGRATION_TEST),
        &env(str_constants::INTEGRATION_TEST_ADMIN),
        &config_lib::CorsAllowOrigin(str_constants::HTTP_LOCALHOST.to_owned()),
    )
    .expect("adf9c06e");
    super::super::AdminAuthReq {
        headers: super::super::HttpAdminHeaderMap::from(headers),
        peer: super::super::AdminPeerAddr::from(super::super::super::StdAdminSocketAddr::from(
            str_constants::VALUE_127_0_0_1_43210
                .parse::<std::net::SocketAddr>()
                .expect("0ce8ff47"),
        )),
        state: super::super::StdSharedAdminAuthSvcState::from(std::sync::Arc::new(state)),
    }
}

#[tokio::test]
async fn html_form_auth_rejects_cookie_without_trusted_origin() {
    let mut headers = http::HeaderMap::new();
    let _cookie = headers.insert(
        http::header::COOKIE,
        http::HeaderValue::from_static("admin_csrf_token=token"),
    );
    assert!(matches!(
        super::form_auth(auth_with_headers(headers)),
        Err(super::super::AdminError::Csrf)
    ));
}

#[tokio::test]
async fn admin_root_redirects_to_users() {
    let response = super::root().await;
    assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static("/admin/users"))
    );
}

#[test]
fn successful_mutation_redirects_to_visible_server_feedback() {
    let response = super::success_redirect(server_admin_contract::AdminFrontendPath::Users);
    assert_eq!(response.status(), http::StatusCode::SEE_OTHER);
    assert_eq!(
        response.headers().get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static("/admin/users#saved"))
    );
}

#[tokio::test]
async fn role_assignment_form_accepts_dynamic_checkbox_fields() {
    let request = http::Request::builder()
        .method(http::Method::POST)
        .header(
            http::header::CONTENT_TYPE,
            str_constants::APPLICATION_X_WWW_FORM_URLENCODED,
        )
        .body(axum::body::Body::from(
            "expected_role_ids=1%2C2&user_id=7&role_1=1&role_2=2",
        ));
    let Ok(request) = request else {
        panic!("6f44bd85");
    };
    let result = <super::super::AxumAdminForm<super::UserRolesForm> as axum::extract::FromRequest<
            (),
        >>::from_request(request, &())
        .await;
    let Ok(super::super::AxumAdminForm(form)) = result else {
        panic!("f639d7d1");
    };

    assert_eq!(i64::from(form.user_id), 7i64);
    assert_eq!(form.expected_role_ids.0.as_ref(), "1,2");
    assert_eq!(form.selected.0.len().get(), 2usize);
}

#[test]
fn selected_form_fields_reject_oversized_maps() {
    let values = (0usize..=super::ADMIN_HTML_FORM_SELECTED_MAX_ITEMS)
        .map(|idx| {
            (
                super::AdminHtmlFormKey::try_from(idx.to_string()).expect("763b9ec0"),
                super::AdminHtmlFormText::try_from(String::new()).expect("ef54739a"),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let Err(_error) = super::StdAdminHtmlSelected::try_from(values) else {
        panic!("c86589e3");
    };
}
