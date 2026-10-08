#[test]
fn test_request_id_ascii_policy_and_header_round_trip() {
    assert!((0u8..=127u8).all(|byte| {
        let text = char::from(byte).to_string();
        let result = crate::request_id::RequestId::try_from(text.clone());
        if (32u8..=126u8).contains(&byte) {
            result.is_ok_and(|request_id| {
                request_id.to_string() == text
                    && http::HeaderValue::try_from(&request_id).is_ok_and(|header_value| {
                        crate::request_id::RequestId::try_from(&header_value)
                            .is_ok_and(|parsed| parsed == request_id)
                    })
            })
        } else {
            result
                == Err(
                    crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid,
                )
        }
    }));
}

#[test]
fn test_request_id_text_header_validation_errors_remain_typed() {
    assert!([String::default(), '\t'.to_string(), constants_str::X.repeat(129usize)]
        .into_iter().all(|text| {
            http::HeaderValue::from_str(text.as_str()).is_ok_and(|header_value| {
                matches!(crate::request_id::RequestId::try_from(&header_value),
                    Err(crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::Invalid(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)))
            })
        }));
}

#[test]
fn test_validates_string_and_header_boundaries() {
    assert_eq!(
        crate::request_id::RequestId::try_from(String::new()),
        Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)
    );
    let maximum = constants_str::A_ALT.repeat(128usize);
    let request_id = crate::request_id::RequestId::try_from(maximum.clone())
        .expect(constants_str::DIAGNOSTIC_3FF39236);
    assert_eq!(request_id.to_string(), maximum);
    assert_eq!(
        crate::request_id::RequestId::try_from(constants_str::A_ALT.repeat(129usize)),
        Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)
    );
    assert!([0u8, 9u8, 10u8, 13u8, 127u8].into_iter().all(|byte| {
        crate::request_id::RequestId::try_from(char::from(byte).to_string())
            == Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)
    }));
    assert_eq!(
        crate::request_id::RequestId::try_from(
            String::from_utf8(vec![0xc3u8, 0xa9u8]).expect(constants_str::DIAGNOSTIC_F246E4F8)
        ),
        Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)
    );
    assert!([vec![0xffu8], vec![b'x', 0xffu8], vec![0xc3u8], vec![0xc3u8, 0xa9u8]]
        .into_iter().all(|bytes| {
            let header = http::HeaderValue::from_bytes(&bytes)
                .expect(constants_str::DIAGNOSTIC_DCB3F9A8);
            header.to_str().is_err_and(|expected| {
                crate::request_id::RequestId::try_from(&header).is_err_and(|error| match error {
                    crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::ToStr(source) => {
                        source.to_string() == expected.to_string()
                            && format!("{source:?}") == format!("{:?}", crate::http_header_to_str_error::HttpHeaderToStrError::from(expected))
                    }
                    crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::Invalid(_) => false,
                })
            })
        }));
    assert_eq!(
        http::HeaderValue::try_from(&request_id).expect(constants_str::DIAGNOSTIC_B0A0854A),
        http::HeaderValue::from_str(maximum.as_str()).expect(constants_str::DIAGNOSTIC_07132954)
    );
}

#[tokio::test]
async fn test_layer_propagates_existing_and_generated_values() {
    let make_router = || {
        axum::Router::from(crate::request_id_layer::RequestIdLayer::default().apply(
            crate::axum_router::AxumRouter::from(axum::Router::new().route(
                constants_str::SLASH,
                axum::routing::get(async || http::StatusCode::OK),
            )),
        ))
    };
    let existing = http::HeaderValue::from_static(constants_str::EXISTING_REQUEST_ID);
    let existing_response = tower::ServiceExt::oneshot(
        make_router(),
        axum::extract::Request::builder()
            .uri(constants_str::SLASH)
            .header(
                constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID,
                existing.clone(),
            )
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_319B3CB4),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_D5A0693B);
    assert_eq!(
        existing_response
            .headers()
            .get(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
        Some(&existing)
    );
    assert_eq!(
        existing_response
            .headers()
            .get(constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME),
        Some(&existing)
    );
    let generated_response = tower::ServiceExt::oneshot(
        make_router(),
        axum::extract::Request::builder()
            .uri(constants_str::SLASH)
            .body(axum::body::Body::empty())
            .expect(constants_str::DIAGNOSTIC_27CE5FBD),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_4CD32371);
    let generated = generated_response
        .headers()
        .get(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID)
        .expect(constants_str::DIAGNOSTIC_12ED6F85);
    assert_eq!(generated.as_bytes().len(), 36usize);
    assert_eq!(
        generated_response
            .headers()
            .get(constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME),
        Some(generated)
    );
}

#[tokio::test(start_paused = true)]
async fn test_request_id_layer_replaces_conflicting_error_response_headers() {
    let router = axum::Router::from(crate::request_id_layer::RequestIdLayer::default().apply(
        crate::axum_router::AxumRouter::from(axum::Router::new().route(
            constants_str::SLASH,
            axum::routing::get(async || {
                let mut response =
                    axum::response::Response::new(axum::body::Body::from(constants_str::ABCD_ALT));
                *response.status_mut() = http::StatusCode::INTERNAL_SERVER_ERROR;
                let _previous_request_id = response.headers_mut().insert(
                    http::HeaderName::from_static(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
                    http::HeaderValue::from_static(constants_str::X),
                );
                let _previous_correlation_id = response.headers_mut().insert(
                    http::HeaderName::from_static(
                        constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME,
                    ),
                    http::HeaderValue::from_static(constants_str::X),
                );
                response
            }),
        )),
    ));
    let mut request = axum::extract::Request::new(axum::body::Body::empty());
    let expected = http::HeaderValue::from_static(constants_str::EXISTING_REQUEST_ID);
    let _previous = request.headers_mut().insert(
        http::HeaderName::from_static(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
        expected.clone(),
    );
    let result = tower::ServiceExt::oneshot(router, request).await;
    assert!(result.is_ok());
    let Ok(response) = result;
    assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        response
            .headers()
            .get(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
        Some(&expected)
    );
    assert_eq!(
        response
            .headers()
            .get(constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME),
        Some(&expected)
    );
    let body = axum::body::to_bytes(response.into_body(), 4usize).await;
    assert!(body.is_ok_and(|bytes| bytes.as_ref() == constants_str::ABCD_ALT.as_bytes()));
}

#[tokio::test]
async fn test_request_id_header_precedence_preserves_valid_correlation_fallback() {
    let check_precedence = async |header_value, expected_header_value| {
        let router = axum::Router::from(crate::request_id_layer::RequestIdLayer::default().apply(
            crate::axum_router::AxumRouter::from(axum::Router::new().route(
                constants_str::SLASH,
                axum::routing::get(async || http::StatusCode::OK),
            )),
        ));
        let mut request = axum::extract::Request::new(axum::body::Body::empty());
        let _previous_primary = request.headers_mut().insert(
            http::HeaderName::from_static(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
            header_value,
        );
        let _previous_correlation = request.headers_mut().insert(
            http::HeaderName::from_static(constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME),
            http::HeaderValue::from_static(constants_str::X),
        );
        let result = tower::ServiceExt::oneshot(router, request).await;
        assert!(result.is_ok());
        let Ok(response) = result;
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
            Some(&expected_header_value)
        );
        assert_eq!(
            response
                .headers()
                .get(constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME),
            Some(&expected_header_value)
        );
    };
    check_precedence(
        http::HeaderValue::from_static(constants_str::EXISTING_REQUEST_ID),
        http::HeaderValue::from_static(constants_str::EXISTING_REQUEST_ID),
    )
    .await;
    check_precedence(
        http::HeaderValue::from_static(constants_str::EMPTY),
        http::HeaderValue::from_static(constants_str::X),
    )
    .await;
    let opaque_result = http::HeaderValue::from_bytes(&[u8::MAX]);
    assert!(opaque_result.is_ok());
    if let Ok(opaque) = opaque_result {
        check_precedence(opaque, http::HeaderValue::from_static(constants_str::X)).await;
    }
}

#[tokio::test]
async fn test_request_id_service_replaces_inbound_extension_and_preserves_inner_error() {
    let stale_result = crate::request_id::RequestId::try_from(constants_str::X.to_owned());
    assert!(stale_result.is_ok());
    let Ok(stale) = stale_result else {
        return;
    };
    let inner = tower::service_fn(|request: axum::extract::Request| {
        assert!(
            request
                .extensions()
                .get::<crate::request_id::RequestId>()
                .is_some_and(
                    |request_id| request_id.to_string() == constants_str::EXISTING_REQUEST_ID
                )
        );
        std::future::ready(Err::<axum::response::Response, _>(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero))
    });
    let mut request = axum::extract::Request::new(axum::body::Body::empty());
    let previous = request.extensions_mut().insert(stale);
    assert!(previous.is_none());
    let previous_header = request.headers_mut().insert(
        http::HeaderName::from_static(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
        http::HeaderValue::from_static(constants_str::EXISTING_REQUEST_ID),
    );
    assert!(previous_header.is_none());
    let result = tower::ServiceExt::oneshot(
        crate::request_id_service::RequestIdService::new(inner, None),
        request,
    )
    .await;
    assert!(matches!(result, Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero)));
}
