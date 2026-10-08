#[test]
fn test_admin_selected_form_deserialization_enforces_exact_item_limit() {
    assert!(
        [0usize, 999usize, 1000usize, 1001usize]
            .into_iter()
            .all(|count| {
                let values = (0usize..count)
                    .map(|index| (index.to_string(), serde_json::json!(index.to_string())))
                    .collect::<serde_json::Map<_, _>>();
                let result = serde_json::from_value::<
                    crate::std_admin_html_selected::StdAdminHtmlSelected,
                >(serde_json::Value::Object(values));
                if count == 1001usize {
                    result.is_err_and(|error| error.is_data())
                } else {
                    result.is_ok_and(|selected| {
                        selected.len().get() == count
                            && selected
                                .iter()
                                .all(|(key, text)| key.get_inner().as_str() == text.as_str())
                    })
                }
            })
    );
}

#[test]
fn test_admin_selected_form_deserialization_validates_nested_text_and_value_types() {
    assert!([8192usize, 8193usize].into_iter().all(|length| {
        [false, true].into_iter().all(|oversized_key| {
            let key = if oversized_key {
                constants_str::X.repeat(length)
            } else {
                constants_str::X.to_owned()
            };
            let text = if oversized_key {
                constants_str::X.to_owned()
            } else {
                constants_str::X.repeat(length)
            };
            let mut values = serde_json::Map::new();
            let _previous = values.insert(key.clone(), serde_json::json!(text));
            let result = serde_json::from_value::<
                crate::std_admin_html_selected::StdAdminHtmlSelected,
            >(serde_json::Value::Object(values));
            if length == 8193usize {
                result.is_err_and(|error| error.is_data())
            } else {
                result.is_ok_and(|selected| {
                    selected.len().get() == 1usize
                        && selected.iter().all(|(stored_key, stored_text)| {
                            stored_key.get_inner().as_str() == key && stored_text.as_str() == text
                        })
                })
            }
        })
    }));
    assert!(
        [
            serde_json::Value::Null,
            serde_json::json!(true),
            serde_json::json!(7u64),
            serde_json::json!([])
        ]
        .into_iter()
        .all(|value| {
            let root_invalid = serde_json::from_value::<
                crate::std_admin_html_selected::StdAdminHtmlSelected,
            >(value.clone())
            .is_err_and(|error| error.is_data());
            let mut values = serde_json::Map::new();
            let _previous = values.insert(constants_str::X.to_owned(), value);
            root_invalid
                && serde_json::from_value::<crate::std_admin_html_selected::StdAdminHtmlSelected>(
                    serde_json::Value::Object(values),
                )
                .is_err_and(|error| error.is_data())
        })
    );
}

#[test]
fn test_admin_form_key_and_text_validate_exact_ascii_and_utf8_byte_limits() {
    let inputs = [0usize, 1usize, 8191usize, 8192usize, 8193usize]
        .map(|length| constants_str::X.repeat(length));
    let mut utf8_at_limit = '\u{00e9}'.to_string().repeat(4096usize);
    let utf8_above_limit = {
        let mut text = utf8_at_limit.clone();
        text.push('x');
        text
    };
    let _removed_character = utf8_at_limit.pop();
    utf8_at_limit.push('x');
    assert!(
        inputs
            .into_iter()
            .chain([
                utf8_at_limit,
                '\u{00e9}'.to_string().repeat(4096usize),
                utf8_above_limit
            ])
            .all(|input| {
                let key = crate::admin_html_form_key::AdminHtmlFormKey::try_from(input.clone());
                let text = crate::admin_html_form_text::AdminHtmlFormText::try_from(input.clone());
                let converted = if input.len() > 8192usize {
                    matches!(
                        key,
                        Err(crate::admin_html_form_key_error::AdminHtmlFormKeyError::TooLong)
                    ) && matches!(
                        text,
                        Err(crate::admin_html_form_text_error::AdminHtmlFormTextError::TooLong)
                    )
                } else {
                    key.is_ok_and(|value| value.get_inner().as_str() == input)
                        && text.is_ok_and(|value| value.as_str() == input)
                };
                converted
                    && serde_json::to_string(&input).is_ok_and(|encoded| {
                        let decoded_key = serde_json::from_str::<
                            crate::admin_html_form_key::AdminHtmlFormKey,
                        >(&encoded);
                        let decoded_text = serde_json::from_str::<
                            crate::admin_html_form_text::AdminHtmlFormText,
                        >(&encoded);
                        if input.len() > 8192usize {
                            decoded_key.is_err_and(|error| error.is_data())
                                && decoded_text.is_err_and(|error| error.is_data())
                        } else {
                            decoded_key.is_ok_and(|value| value.get_inner().as_str() == input)
                                && decoded_text.is_ok_and(|value| value.as_str() == input)
                        }
                    })
            })
    );
}

#[test]
fn test_admin_assignment_lists_preserve_empty_order_duplicates_and_maximum_id() {
    assert!(
        [Vec::new(), vec![1i64], vec![7i64, 1i64, 7i64, i64::MAX]]
            .into_iter()
            .all(|expected| {
                let input = expected
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(&','.to_string());
                crate::admin_html_form_text::AdminHtmlFormText::try_from(input).is_ok_and(|text| {
                    crate::role_ids_impl::role_ids_impl(&text).is_ok_and(|ids| {
                        serde_json::to_value(ids)
                            .is_ok_and(|value| value == serde_json::json!(expected))
                    }) && crate::rule_ids_impl::rule_ids_impl(&text).is_ok_and(|ids| {
                        serde_json::to_value(ids)
                            .is_ok_and(|value| value == serde_json::json!(expected))
                    })
                })
            })
    );
}

#[test]
fn test_admin_assignment_lists_reject_invalid_numeric_and_separator_entries() {
    let maximum_overflow =
        (u64::try_from(i64::MAX).map(|value| value + 1u64)).map(|value| value.to_string());
    assert!(maximum_overflow.is_ok_and(|overflow| {
        let mut trailing = 1i64.to_string();
        trailing.push(',');
        let mut leading = ','.to_string();
        leading.push_str(constants_str::VALUE_1);
        let mut padded = constants_str::SPACE.to_owned();
        padded.push_str(constants_str::VALUE_1);
        [
            0i64.to_string(),
            (-1i64).to_string(),
            overflow,
            constants_str::X.to_owned(),
            leading,
            trailing,
            padded,
        ]
        .into_iter()
        .all(|input| {
            crate::admin_html_form_text::AdminHtmlFormText::try_from(input).is_ok_and(|text| {
                matches!(
                    crate::role_ids_impl::role_ids_impl(&text),
                    Err(crate::admin_error::AdminError::Validation)
                ) && matches!(
                    crate::rule_ids_impl::rule_ids_impl(&text),
                    Err(crate::admin_error::AdminError::Validation)
                )
            })
        })
    }));
}

#[test]
fn test_admin_selected_form_map_exact_item_boundaries_preserve_entries() {
    assert!([0usize, 999usize, 1000usize, 1001usize].into_iter().all(|count| {
        (0usize..count).try_fold(std::collections::BTreeMap::new(), |mut values, index| {
            let (Ok(key), Ok(text)) = (
                crate::admin_html_form_key::AdminHtmlFormKey::try_from(index.to_string()),
                crate::admin_html_form_text::AdminHtmlFormText::try_from(index.to_string()),
            ) else { return None; };
            let _previous = values.insert(key, text);
            Some(values)
        }).is_some_and(|values| {
            let result = crate::std_admin_html_selected::StdAdminHtmlSelected::try_from(values);
            if count == 1001usize {
                matches!(result, Err(crate::std_admin_html_selected_error::StdAdminHtmlSelectedError::TooMany))
            } else {
                result.is_ok_and(|selected| selected.len().get() == count
                    && selected.iter().all(|(key, text)| key.get_inner().as_str() == text.as_str()))
            }
        })
    }));
}

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
    let shared_state =
        crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc::from_state(
            state,
        );
    let layer =
        crate::admin_generated_auth_layer::AdminGeneratedAuthLayer::from(shared_state.clone());
    assert!(std::sync::Arc::ptr_eq(
        layer.get_state().as_ref(),
        shared_state.as_ref()
    ));
    let open_api_path = server_admin_contract::admin_route::AdminRoute::OpenApi
        .contract()
        .path();
    let metrics_path = server_admin_contract::admin_route::AdminRoute::Metrics
        .contract()
        .path();
    let cases = std::iter::once((
        constants_str::SLASH,
        http::Method::GET,
        http::StatusCode::FORBIDDEN,
    ))
    .chain(
        [
            open_api_path.as_ref(),
            server_admin_contract::admin_frontend_path::AdminFrontendPath::OpenApiDocument.get(),
            metrics_path.as_ref(),
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Metrics.get(),
        ]
        .into_iter()
        .flat_map(|path| {
            [
                http::Method::GET,
                http::Method::POST,
                http::Method::PUT,
                http::Method::PATCH,
                http::Method::DELETE,
                http::Method::HEAD,
                http::Method::OPTIONS,
            ]
            .into_iter()
            .map(move |method| {
                let status = if method == http::Method::GET {
                    http::StatusCode::UNAUTHORIZED
                } else {
                    http::StatusCode::METHOD_NOT_ALLOWED
                };
                (path, method, status)
            })
        }),
    );
    futures::stream::StreamExt::fold(
        futures::stream::iter(cases),
        (),
        |(), (path, method, status)| {
            let mut service = tower::Layer::layer(
                &layer,
                tower::service_fn(|_request| {
                    std::future::ready(Err::<axum::response::Response, http::StatusCode>(
                        http::StatusCode::IM_A_TEAPOT,
                    ))
                }),
            );
            assert!(std::sync::Arc::ptr_eq(
                service.get_state().as_ref(),
                shared_state.as_ref()
            ));
            async move {
                let mut context = std::task::Context::from_waker(std::task::Waker::noop());
                assert!(matches!(
                    tower::Service::poll_ready(&mut service, &mut context),
                    std::task::Poll::Ready(Ok(()))
                ));
                let mut request = axum::extract::Request::new(axum::body::Body::empty());
                let uri = path.parse::<http::Uri>();
                assert!(uri.is_ok());
                if let Ok(uri) = uri {
                    *request.uri_mut() = uri;
                    *request.method_mut() = method;
                    assert!(
                        tower::Service::call(&mut service, request)
                            .await
                            .is_ok_and(|response| response.status() == status)
                    );
                }
            }
        },
    )
    .await;
    let (mut parts, ()) = http::Request::new(()).into_parts();
    parts.headers = headers;
    let missing_peer =
        <crate::admin_auth_request::AdminAuthRequest as axum::extract::FromRequestParts<
            crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc,
        >>::from_request_parts(&mut parts, &shared_state)
        .await;
    assert!(matches!(
        missing_peer,
        Err(crate::admin_error::AdminError::Authentication)
    ));
    let peer = constants_str::VALUE_127_0_0_1_43210
        .parse::<std::net::SocketAddr>()
        .expect(constants_str::DIAGNOSTIC_0CE8FF47);
    assert!(
        parts
            .extensions
            .insert(axum::extract::ConnectInfo(peer))
            .is_none()
    );
    let extracted =
        <crate::admin_auth_request::AdminAuthRequest as axum::extract::FromRequestParts<
            crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc,
        >>::from_request_parts(&mut parts, &shared_state)
        .await;
    assert_eq!(
        parts.headers.remove(http::header::COOKIE),
        Some(http::HeaderValue::from_static(
            constants_str::VALUE_BF7FDCFF
        ))
    );
    let admin_peer_addr = crate::admin_peer_addr::AdminPeerAddr::from(
        server_admin_core::admin_socket_addr::AdminSocketAddr::from(peer),
    );
    assert!(matches!(
        crate::authorization_authenticate::authorization_authenticate(
            shared_state.as_ref(),
            crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(&parts.headers),
            admin_peer_addr,
        )
        .await,
        Err(crate::admin_error::AdminError::Authentication)
    ));
    let malformed_cookie = format!(
        "{}={}",
        constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
        constants_str::X
    );
    let malformed_cookie_value = malformed_cookie.parse::<http::HeaderValue>();
    assert!(malformed_cookie_value.is_ok());
    if let Ok(malformed_cookie_value) = malformed_cookie_value {
        let malformed_cookie_headers =
            http::HeaderMap::from_iter([(http::header::COOKIE, malformed_cookie_value)]);
        assert!(matches!(
            crate::authorization_authenticate::authorization_authenticate(
                shared_state.as_ref(),
                crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
                    &malformed_cookie_headers
                ),
                admin_peer_addr,
            )
            .await,
            Err(crate::admin_error::AdminError::Authentication)
        ));
    }
    assert!(extracted.is_ok_and(|request| {
        assert!(std::sync::Arc::ptr_eq(
            request.get_state().as_ref(),
            shared_state.as_ref()
        ));
        assert_eq!(
            request.get_peer().socket_addr(),
            server_admin_core::admin_socket_addr::AdminSocketAddr::from(peer)
        );
        assert_eq!(request.get_headers().as_ref().len(), 1usize);
        assert_eq!(
            request.get_headers().as_ref().get(http::header::COOKIE),
            Some(&http::HeaderValue::from_static(
                constants_str::VALUE_BF7FDCFF
            ))
        );
        matches!(
            crate::form_auth_impl::form_auth_impl(request),
            Err(crate::admin_error::AdminError::Csrf)
        )
    }));
    assert!(
        parts
            .headers
            .insert(
                http::header::ORIGIN,
                http::HeaderValue::from_static(constants_str::HTTP_LOCALHOST)
            )
            .is_none()
    );
    assert!(
        parts
            .headers
            .insert(
                http::header::COOKIE,
                http::HeaderValue::from_static(constants_str::VALUE_BF7FDCFF)
            )
            .is_none()
    );
    assert!(
        parts
            .headers
            .insert(
                http::HeaderName::from_static(constants_str::X_CSRF_TOKEN_ALT),
                http::HeaderValue::from_static(constants_str::X)
            )
            .is_none()
    );
    let trusted_origin =
        <crate::admin_auth_request::AdminAuthRequest as axum::extract::FromRequestParts<
            crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc,
        >>::from_request_parts(&mut parts, &shared_state)
        .await;
    assert!(trusted_origin.is_ok_and(|request| {
        crate::form_auth_impl::form_auth_impl(request).is_ok_and(|request| {
            std::sync::Arc::ptr_eq(request.get_state().as_ref(), shared_state.as_ref())
                && request.get_peer().socket_addr()
                    == server_admin_core::admin_socket_addr::AdminSocketAddr::from(peer)
                && request
                    .get_headers()
                    .as_ref()
                    .get(http::HeaderName::from_static(
                        constants_str::X_CSRF_TOKEN_ALT,
                    ))
                    == Some(&http::HeaderValue::from_static(
                        constants_str::VALUE_3C469E9D,
                    ))
                && request.get_headers().as_ref().get(http::header::COOKIE)
                    == Some(&http::HeaderValue::from_static(
                        constants_str::VALUE_BF7FDCFF,
                    ))
                && request.get_headers().as_ref().get(http::header::ORIGIN)
                    == Some(&http::HeaderValue::from_static(
                        constants_str::HTTP_LOCALHOST,
                    ))
        })
    }));
    futures::StreamExt::fold(
        futures::stream::iter([
            (serde_json::json!({(stringify!(updates)): []}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {}, (stringify!(changes)): {(stringify!(name)): constants_str::X}}]}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(role_id)): 1i64}, (stringify!(changes)): {}}]}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(name)): constants_str::X}, (stringify!(changes)): {(stringify!(rules)): {(stringify!(expected_rule_ids)): [], (stringify!(rule_ids)): []}}}]}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(is_system)): false}, (stringify!(changes)): {(stringify!(rules)): {(stringify!(expected_rule_ids)): [], (stringify!(rule_ids)): []}}}]}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(role_id)): 1i64, (stringify!(name)): constants_str::X}, (stringify!(changes)): {(stringify!(rules)): {(stringify!(expected_rule_ids)): [], (stringify!(rule_ids)): []}}}]}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(role_id)): 1i64, (stringify!(is_system)): false}, (stringify!(changes)): {(stringify!(rules)): {(stringify!(expected_rule_ids)): [], (stringify!(rule_ids)): []}}}]}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(role_id)): 1i64}, (stringify!(changes)): {(stringify!(name)): constants_str::X}}, {(stringify!(filter)): {(stringify!(role_id)): 2i64}, (stringify!(changes)): {}}]}), true),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(role_id)): 1i64}, (stringify!(changes)): {(stringify!(name)): constants_str::X}}]}), false),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(role_id)): 1i64}, (stringify!(changes)): {(stringify!(rules)): {(stringify!(expected_rule_ids)): [], (stringify!(rule_ids)): []}}}]}), false),
            (serde_json::json!({(stringify!(updates)): [{(stringify!(filter)): {(stringify!(role_id)): 1i64}, (stringify!(changes)): {(stringify!(name)): constants_str::X, (stringify!(rules)): {(stringify!(expected_rule_ids)): [], (stringify!(rule_ids)): []}}}]}), false),
        ]),
        (),
        async |(), (wire, expected_validation)| {
            let admin_update_roles_request_result = serde_json::from_value::<server_admin_contract::admin_update_roles_request::AdminUpdateRolesRequest>(wire);
            assert!(admin_update_roles_request_result.as_ref().err().is_none());
            if let Ok(admin_update_roles_request) = admin_update_roles_request_result {
                let admin_auth_request = crate::admin_auth_request::AdminAuthRequest::new(
                    crate::http_admin_header_map::HttpAdminHeaderMap::from(parts.headers.clone()),
                    shared_state.clone(),
                    admin_peer_addr,
                );
                let result = crate::role_mutations_update_many::role_mutations_update_many(
                    admin_auth_request,
                    crate::admin_role_update_slice::AdminRoleUpdateSlice::from(admin_update_roles_request.updates()),
                ).await;
                assert!(if expected_validation {
                    matches!(result, Err(crate::admin_error::AdminError::Validation))
                } else {
                    matches!(result, Err(crate::admin_error::AdminError::Authentication))
                });
            }
        },
    ).await;

    let admin_password_hasher = shared_state.as_ref().get_password_hasher();
    let available_permits = admin_password_hasher
        .get_semaphore()
        .get_inner()
        .available_permits();
    let password = || {
        crate::runtime_admin_password::RuntimeAdminPassword::try_from(
            constants_str::CORRECT_PASSWORD_ALT.to_owned(),
        )
    };
    let invalid_hash = || {
        pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret::try_from(
            constants_str::X.to_owned(),
        )
        .map(crate::admin_password_hash::AdminPasswordHash::new)
    };
    let verification_password_result = password();
    let invalid_hash_result = invalid_hash();
    assert!(verification_password_result.as_ref().err().is_none());
    assert!(invalid_hash_result.as_ref().err().is_none());
    if let (Ok(runtime_admin_password), Ok(admin_password_hash)) =
        (verification_password_result, invalid_hash_result)
    {
        assert!(matches!(
            admin_password_hasher
                .verify(runtime_admin_password, admin_password_hash)
                .await,
            Err(crate::admin_password_hash_error::AdminPasswordHashError::PasswordHash(_))
        ));
        assert_eq!(
            admin_password_hasher
                .get_semaphore()
                .get_inner()
                .available_permits(),
            available_permits
        );
    }
    admin_password_hasher.get_semaphore().get_inner().close();
    assert!(matches!(
        admin_password_hasher.acquire().await,
        Err(crate::admin_password_hash_error::AdminPasswordHashError::SemaphoreClosed(_))
    ));
    let hashing_password_result = password();
    assert!(hashing_password_result.as_ref().err().is_none());
    if let Ok(runtime_admin_password) = hashing_password_result {
        assert!(matches!(
            admin_password_hasher.hash(runtime_admin_password).await,
            Err(crate::admin_password_hash_error::AdminPasswordHashError::SemaphoreClosed(_))
        ));
    }
    let closed_verification_password_result = password();
    let closed_invalid_hash_result = invalid_hash();
    assert!(closed_verification_password_result.as_ref().err().is_none());
    assert!(closed_invalid_hash_result.as_ref().err().is_none());
    if let (Ok(runtime_admin_password), Ok(admin_password_hash)) = (
        closed_verification_password_result,
        closed_invalid_hash_result,
    ) {
        assert!(matches!(
            admin_password_hasher
                .verify(runtime_admin_password, admin_password_hash)
                .await,
            Err(crate::admin_password_hash_error::AdminPasswordHashError::SemaphoreClosed(_))
        ));
    }
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
        crate::rule_ids_impl::rule_ids_impl(&empty),
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
        crate::rule_ids_impl::rule_ids_impl(&malformed),
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
        .map(|index| {
            (
                crate::admin_html_form_key::AdminHtmlFormKey::try_from(index.to_string())
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
#[tokio::test]
async fn test_admin_json_extractors_preserve_sign_in_contract_and_classify_rejections() {
    let valid_json = serde_json::json!({
        (stringify!(login)): constants_str::ADMIN_ALT,
        (stringify!(password)): constants_str::X,
    });
    let mut unknown_field_json = valid_json.clone();
    assert!(unknown_field_json.as_object_mut().is_some_and(|object| {
        object
            .insert(
                stringify!(unknown).to_owned(),
                serde_json::Value::Bool(true),
            )
            .is_none()
    }));
    let cases = [
        (valid_json.to_string(), true, true),
        (valid_json.to_string(), false, false),
        (constants_str::X.to_owned(), true, false),
        (serde_json::Value::Null.to_string(), true, false),
        (serde_json::json!({}).to_string(), true, false),
        (unknown_field_json.to_string(), true, false),
        (
            serde_json::json!({
                (stringify!(login)): constants_str::ADMIN,
                (stringify!(password)): constants_str::X,
            })
            .to_string(),
            true,
            false,
        ),
    ];
    futures::stream::StreamExt::fold(futures::stream::iter(cases.into_iter().flat_map(|(text, content_type, valid)| {
        [false, true].into_iter().map(move |generic| (text.clone(), content_type, valid, generic))
    })), (), async move |(), (text, content_type, valid, generic)| {
        let mut request = axum::extract::Request::new(axum::body::Body::from(text));
        *request.method_mut() = http::Method::POST;
        if content_type {
            assert!(request.headers_mut().insert(http::header::CONTENT_TYPE,
                http::HeaderValue::from_static(constants_str::APPLICATION_JSON)).is_none());
        }
        let extracted = if generic {
            <crate::axum_admin_json::AxumAdminJson<server_admin_contract::admin_sign_in_request::AdminSignInRequest> as axum::extract::FromRequest<()>>::from_request(request, &()).await.map(crate::axum_admin_json::AxumAdminJson::into_inner)
        } else {
            <crate::admin_sign_in_json::AdminSignInJson as axum::extract::FromRequest<()>>::from_request(request, &()).await.map(crate::admin_sign_in_json::AdminSignInJson::into_inner)
        };
        if valid {
            assert!(extracted.is_ok_and(|admin_sign_in_request| {
                let (admin_login, admin_password) = admin_sign_in_request.into_parts();
                admin_login.as_ref().as_str() == constants_str::ADMIN_ALT
                    && admin_password.as_ref().as_str() == constants_str::X
            }));
        } else {
            assert!(matches!(extracted, Err(crate::admin_error::AdminError::Validation)));
        }
    }).await;
}
#[tokio::test]
async fn test_admin_form_extractor_preserves_get_query_and_classifies_rejections() {
    let query_uri = format!("{}?{}", constants_str::SLASH, constants_str::VALUE_08400B3F);
    let cases = [
        (
            http::Method::GET,
            query_uri.as_str(),
            constants_str::EMPTY,
            None,
            true,
        ),
        (
            http::Method::GET,
            constants_str::SLASH,
            constants_str::VALUE_08400B3F,
            None,
            false,
        ),
        (
            http::Method::POST,
            constants_str::SLASH,
            constants_str::VALUE_08400B3F,
            None,
            false,
        ),
        (
            http::Method::POST,
            constants_str::SLASH,
            constants_str::VALUE_08400B3F,
            Some(constants_str::APPLICATION_JSON),
            false,
        ),
        (
            http::Method::POST,
            constants_str::SLASH,
            constants_str::X,
            Some(constants_str::APPLICATION_X_WWW_FORM_URLENCODED),
            false,
        ),
        (
            http::Method::POST,
            constants_str::SLASH,
            constants_str::EMPTY,
            Some(constants_str::APPLICATION_X_WWW_FORM_URLENCODED),
            false,
        ),
    ];
    futures::stream::StreamExt::fold(
        futures::stream::iter(cases),
        (),
        async move |(), (method, uri, text, content_type, valid)| {
            let mut request = axum::extract::Request::new(axum::body::Body::from(text));
            *request.method_mut() = method;
            let parsed_uri = uri.parse::<http::Uri>();
            assert!(parsed_uri.is_ok());
            if let Ok(parsed_uri) = parsed_uri {
                *request.uri_mut() = parsed_uri;
                if let Some(content_type) = content_type {
                    assert!(
                        request
                            .headers_mut()
                            .insert(
                                http::header::CONTENT_TYPE,
                                http::HeaderValue::from_static(content_type)
                            )
                            .is_none()
                    );
                }
                let extracted = <crate::axum_admin_form::AxumAdminForm<
                    crate::user_roles_form::UserRolesForm,
                > as axum::extract::FromRequest<()>>::from_request(
                    request, &()
                )
                .await;
                if valid {
                    assert!(extracted.is_ok_and(|axum_admin_form| {
                        let user_roles_form = axum_admin_form.into_inner();
                        i64::from(*user_roles_form.get_user_id()) == 7i64
                            && user_roles_form.get_expected_role_ids().as_str()
                                == constants_str::VALUE_17F8AF97
                            && user_roles_form.get_selected().len().get() == 2usize
                    }));
                } else {
                    assert!(matches!(
                        extracted,
                        Err(crate::admin_error::AdminError::Validation)
                    ));
                }
            }
        },
    )
    .await;
}
#[tokio::test]
async fn test_admin_body_extractors_classify_limit_before_deserialization() {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
    enum AdminBodyLimitExtractorFixtureKind {
        SignInJson,
        GenericJson,
        Form,
    }
    futures::stream::StreamExt::fold(futures::stream::iter([
        AdminBodyLimitExtractorFixtureKind::SignInJson,
        AdminBodyLimitExtractorFixtureKind::GenericJson,
        AdminBodyLimitExtractorFixtureKind::Form,
    ]), (), async move |(), admin_body_limit_extractor_fixture_kind| {
        let admin_body_limit_extractor_fixture_kind_ref = &admin_body_limit_extractor_fixture_kind;
        let service = tower::Layer::layer(&axum::extract::DefaultBodyLimit::max(0usize), tower::service_fn(async move |request| {
            match *admin_body_limit_extractor_fixture_kind_ref {
                AdminBodyLimitExtractorFixtureKind::SignInJson => {
                    <crate::admin_sign_in_json::AdminSignInJson as axum::extract::FromRequest<()>>::from_request(request, &()).await.map(drop)
                }
                AdminBodyLimitExtractorFixtureKind::GenericJson => {
                    <crate::axum_admin_json::AxumAdminJson<server_admin_contract::admin_sign_in_request::AdminSignInRequest> as axum::extract::FromRequest<()>>::from_request(request, &()).await.map(drop)
                }
                AdminBodyLimitExtractorFixtureKind::Form => {
                    <crate::axum_admin_form::AxumAdminForm<crate::user_roles_form::UserRolesForm> as axum::extract::FromRequest<()>>::from_request(request, &()).await.map(drop)
                }
            }
        }));
        let mut request = axum::extract::Request::new(axum::body::Body::from(constants_str::X));
        *request.method_mut() = http::Method::POST;
        let content_type = match admin_body_limit_extractor_fixture_kind {
            AdminBodyLimitExtractorFixtureKind::SignInJson | AdminBodyLimitExtractorFixtureKind::GenericJson => constants_str::APPLICATION_JSON,
            AdminBodyLimitExtractorFixtureKind::Form => constants_str::APPLICATION_X_WWW_FORM_URLENCODED,
        };
        assert!(request.headers_mut().insert(http::header::CONTENT_TYPE,
            http::HeaderValue::from_static(content_type)).is_none());
        assert!(matches!(tower::ServiceExt::oneshot(service, request).await,
            Err(crate::admin_error::AdminError::PayloadTooLarge)));
    }).await;
}

#[tokio::test]
async fn test_sign_in_errors_render_generic_html_without_internal_diagnostics() {
    futures::StreamExt::fold(
        futures::stream::iter([
            (
                crate::admin_error::AdminError::Authentication,
                http::StatusCode::UNAUTHORIZED,
            ),
            (
                crate::admin_error::AdminError::Authorization,
                http::StatusCode::FORBIDDEN,
            ),
            (
                crate::admin_error::AdminError::Csrf,
                http::StatusCode::FORBIDDEN,
            ),
            (
                crate::admin_error::AdminError::Conflict,
                http::StatusCode::CONFLICT,
            ),
            (
                crate::admin_error::AdminError::RateLimited,
                http::StatusCode::TOO_MANY_REQUESTS,
            ),
            (
                crate::admin_error::AdminError::Validation,
                http::StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                crate::admin_error::AdminError::PayloadTooLarge,
                http::StatusCode::PAYLOAD_TOO_LARGE,
            ),
            (
                crate::admin_error::AdminError::MethodNotAllowed,
                http::StatusCode::METHOD_NOT_ALLOWED,
            ),
        ]),
        (),
        async |(), (admin_error, expected_status)| {
            let diagnostic = admin_error.to_string();
            let response = crate::sign_in_error_response::sign_in_error_response(admin_error, None);
            assert_eq!(response.status(), expected_status);
            let body_result = axum::body::to_bytes(response.into_body(), 16_384usize).await;
            assert!(body_result.is_ok());
            if let Ok(body) = body_result {
                let html_result = std::str::from_utf8(&body);
                assert!(html_result.as_ref().err().is_none());
                if let Ok(html) = html_result {
                    assert!(html.contains(constants_str::SIGN_IN_FAILED));
                    assert!(!html.contains(diagnostic.as_str()));
                    assert!(html.contains(constants_str::ADMIN_UI_ADMINISTRATOR_SIGN_IN));
                    assert!(html.contains(
                        server_admin_contract::admin_html_action::AdminHtmlAction::SignIn.get()
                    ));
                    assert!(!html.contains(constants_str::VALUE_A8036BFC));
                }
            }
        },
    )
    .await;
}
