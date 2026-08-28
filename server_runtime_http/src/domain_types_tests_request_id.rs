#[test]
fn validates_string_and_header_boundaries() {
    assert_eq!(
        crate::RequestId::try_from(String::new()),
        Err(crate::RequestIdTryFromStringError)
    );
    let maximum = constants_str::A_ALT.repeat(128usize);
    let request_id = crate::RequestId::try_from(maximum.clone())
        .expect("3ff39236 validates_string_and_header_boundaries invariant must hold");
    assert_eq!(request_id.to_string(), maximum);
    assert_eq!(
        crate::RequestId::try_from("a".repeat(129usize)),
        Err(crate::RequestIdTryFromStringError)
    );
    assert_eq!(
        crate::RequestId::try_from(
            String::from_utf8(vec![0xc3u8, 0xa9u8])
                .expect("f246e4f8 validates_string_and_header_boundaries invariant must hold")
        ),
        Err(crate::RequestIdTryFromStringError)
    );
    assert!(matches!(
        crate::RequestId::try_from(
            &http::HeaderValue::from_bytes(&[0xffu8])
                .expect("dcb3f9a8 validates_string_and_header_boundaries invariant must hold")
        ),
        Err(crate::RequestIdTryFromHttpHeaderValueError::ToStr(_))
    ));
    assert_eq!(
        http::HeaderValue::try_from(&request_id)
            .expect("b0a0854a validates_string_and_header_boundaries invariant must hold"),
        http::HeaderValue::from_str(maximum.as_str())
            .expect("07132954 validates_string_and_header_boundaries invariant must hold")
    );
}

#[tokio::test]
async fn layer_propagates_existing_and_generated_values() {
    let make_router = || {
        axum::Router::from(
            crate::RequestIdLayer::default().apply(crate::AxumRouter::from(
                axum::Router::new().route(
                    constants_str::SLASH,
                    axum::routing::get(async || http::StatusCode::OK),
                ),
            )),
        )
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
            .expect("319b3cb4 layer_propagates_existing_and_generated_values invariant must hold"),
    )
    .await
    .expect("d5a0693b layer_propagates_existing_and_generated_values invariant must hold");
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
            .expect("27ce5fbd layer_propagates_existing_and_generated_values invariant must hold"),
    )
    .await
    .expect("4cd32371 layer_propagates_existing_and_generated_values invariant must hold");
    let generated = generated_response
        .headers()
        .get(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID)
        .expect("12ed6f85 layer_propagates_existing_and_generated_values invariant must hold");
    assert_eq!(generated.as_bytes().len(), 36usize);
    assert_eq!(
        generated_response
            .headers()
            .get(constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME),
        Some(generated)
    );
}
