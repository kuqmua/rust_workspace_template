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
    let request = crate::admin_auth_request::AdminAuthRequest::new(
        crate::http_admin_header_map::HttpAdminHeaderMap::from(headers),
        crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc::from(
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
