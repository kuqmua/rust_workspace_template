#[tokio::test]
async fn test_http_header_map_extractor_preserves_values_and_independent_request_headers() {
    let mut populated_headers = http::HeaderMap::new();
    let _first_cookie = populated_headers.append(
        http::header::COOKIE,
        http::HeaderValue::from_static(constants_str::TEST_FIRST),
    );
    let _second_cookie = populated_headers.append(
        http::header::COOKIE,
        http::HeaderValue::from_static(constants_str::ADMIN_ALT),
    );
    let _content_type = populated_headers.insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static(constants_str::APPLICATION_JSON),
    );
    futures::stream::StreamExt::fold(
        futures::stream::iter([http::HeaderMap::new(), populated_headers]),
        (),
        async |(), headers| {
            let (mut parts, _body) = http::Request::new(()).into_parts();
            parts.headers = headers;
            let extracted_result = <crate::http_admin_header_map::HttpAdminHeaderMap as
                axum::extract::FromRequestParts<server_admin_core::std_admin_bool::StdAdminBool>>::from_request_parts(
                    &mut parts,
                    &server_admin_core::std_admin_bool::StdAdminBool::from(true),
                ).await;
            let Ok(mut extracted) = extracted_result;
            assert_eq!(extracted.as_ref(), &parts.headers);
            let original_cookie_count = parts.headers.get_all(http::header::COOKIE).iter().count();
            assert!(original_cookie_count == 0usize || original_cookie_count == 2usize);
            assert_eq!(extracted.get_inner().get_all(http::header::COOKIE).iter().count(), original_cookie_count);
            let _replacement = extracted.get_inner_mut().insert(
                http::header::COOKIE,
                http::HeaderValue::from_static(constants_str::FIXED_TEST_TOKEN),
            );
            assert_eq!(parts.headers.get_all(http::header::COOKIE).iter().count(), original_cookie_count);
            assert_eq!(extracted.get_inner().get(http::header::COOKIE), Some(&http::HeaderValue::from_static(constants_str::FIXED_TEST_TOKEN)));
            assert!(parts.headers.get_all(http::header::COOKIE).iter().all(|value| value != constants_str::FIXED_TEST_TOKEN));
            let _authorization = parts.headers.insert(
                http::header::AUTHORIZATION,
                http::HeaderValue::from_static(constants_str::ADMIN_ALT),
            );
            assert!(extracted.get_inner().get(http::header::AUTHORIZATION).is_none());
            assert_eq!(parts.headers.get(http::header::AUTHORIZATION), Some(&http::HeaderValue::from_static(constants_str::ADMIN_ALT)));
        },
    ).await;
}
