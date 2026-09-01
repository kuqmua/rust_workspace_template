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
        crate::request_id::RequestId::try_from("a".repeat(129usize)),
        Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)
    );
    assert_eq!(
        crate::request_id::RequestId::try_from(
            String::from_utf8(vec![0xc3u8, 0xa9u8]).expect(constants_str::DIAGNOSTIC_F246E4F8)
        ),
        Err(crate::request_id_try_from_string_error::RequestIdTryFromStringError::Invalid)
    );
    assert!(matches!(
        crate::request_id::RequestId::try_from(
            &http::HeaderValue::from_bytes(&[0xffu8])
                .expect(constants_str::DIAGNOSTIC_DCB3F9A8)
        ),
        Err(crate::request_id_try_from_http_header_value_error::RequestIdTryFromHttpHeaderValueError::ToStr(_))
    ));
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
