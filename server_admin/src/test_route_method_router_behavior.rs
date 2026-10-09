#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_registered_method_routers_dispatch_exact_methods_and_get_head_fallback() {
        let cases = [
            (
                frontend_contract::route_method::RouteMethod::Connect,
                http::Method::CONNECT,
            ),
            (
                frontend_contract::route_method::RouteMethod::Delete,
                http::Method::DELETE,
            ),
            (
                frontend_contract::route_method::RouteMethod::Get,
                http::Method::GET,
            ),
            (
                frontend_contract::route_method::RouteMethod::Head,
                http::Method::HEAD,
            ),
            (
                frontend_contract::route_method::RouteMethod::Options,
                http::Method::OPTIONS,
            ),
            (
                frontend_contract::route_method::RouteMethod::Patch,
                http::Method::PATCH,
            ),
            (
                frontend_contract::route_method::RouteMethod::Post,
                http::Method::POST,
            ),
            (
                frontend_contract::route_method::RouteMethod::Put,
                http::Method::PUT,
            ),
            (
                frontend_contract::route_method::RouteMethod::Trace,
                http::Method::TRACE,
            ),
        ];
        let checks = cases.iter().map(async |(route_method, http_method)| {
            let router = axum::Router::from(frontend_contract::register_route::register_route(
                frontend_contract::frontend_contract_axum_router::FrontendContractAxumRouter::from(
                    axum::Router::<()>::new(),
                ),
                frontend_contract::contract_str::ContractStr::from(constants_str::VALUE_0587C50E),
                frontend_contract::route_method_router::route_method_router::<(), _, _>(
                    *route_method,
                    async || http::StatusCode::NO_CONTENT,
                ),
            ));
            let method_checks = cases
                .iter()
                .map(async |(_other_route_method, request_method)| {
                    let expected = if request_method == http_method
                        || matches!(
                            route_method,
                            frontend_contract::route_method::RouteMethod::Get
                        ) && request_method == http::Method::HEAD
                    {
                        http::StatusCode::NO_CONTENT
                    } else {
                        http::StatusCode::METHOD_NOT_ALLOWED
                    };
                    let mut request = axum::extract::Request::new(axum::body::Body::empty());
                    *request.method_mut() = request_method.clone();
                    *request.uri_mut() = http::Uri::from_static(constants_str::VALUE_0587C50E);
                    tower::ServiceExt::oneshot(router.clone(), request)
                        .await
                        .is_ok_and(|response| response.status() == expected)
                });
            let methods_match = futures::future::join_all(method_checks)
                .await
                .into_iter()
                .all(std::convert::identity);
            let mut unmatched = axum::extract::Request::new(axum::body::Body::empty());
            *unmatched.method_mut() = http_method.clone();
            *unmatched.uri_mut() = http::Uri::from_static(constants_str::V1);
            methods_match
                && tower::ServiceExt::oneshot(router, unmatched)
                    .await
                    .is_ok_and(|response| response.status() == http::StatusCode::NOT_FOUND)
        });
        assert!(
            futures::future::join_all(checks)
                .await
                .into_iter()
                .all(std::convert::identity)
        );
    }

    #[tokio::test]
    async fn test_admin_query_extractor_preserves_defaults_decoding_and_pagination_bounds() {
        let cases = [
            (
                constants_str::SLASH.to_owned(),
                serde_json::json!({
                    (stringify!(search)): constants_str::EMPTY,
                    (stringify!(sort)): constants_str::EMPTY,
                    (stringify!(offset)): 0u32,
                    (stringify!(limit)): 20u16,
                    (stringify!(direction)): server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending,
                }),
            ),
            (
                format!(
                    "{}?{}={}%20{}&{}={}&{}={}&{}={}&{}={}",
                    constants_str::SLASH,
                    stringify!(search),
                    constants_str::ADMIN_ALT,
                    constants_str::USER,
                    stringify!(sort),
                    stringify!(display_name),
                    stringify!(offset),
                    u32::MAX,
                    stringify!(limit),
                    100u16,
                    stringify!(direction),
                    server_admin_contract::admin_sort_direction::AdminSortDirection::Descending
                        .as_ref(),
                ),
                serde_json::json!({
                    (stringify!(search)): format!("{} {}", constants_str::ADMIN_ALT, constants_str::USER),
                    (stringify!(sort)): stringify!(display_name),
                    (stringify!(offset)): u32::MAX,
                    (stringify!(limit)): 100u16,
                    (stringify!(direction)): server_admin_contract::admin_sort_direction::AdminSortDirection::Descending,
                }),
            ),
            (
                format!(
                    "{}?{}={}&{}={}&{}={}",
                    constants_str::SLASH,
                    stringify!(search),
                    constants_str::A_ALT.repeat(128usize),
                    stringify!(sort),
                    constants_str::A_ALT.repeat(32usize),
                    stringify!(limit),
                    1u16,
                ),
                serde_json::json!({
                    (stringify!(search)): constants_str::A_ALT.repeat(128usize),
                    (stringify!(sort)): constants_str::A_ALT.repeat(32usize),
                    (stringify!(offset)): 0u32,
                    (stringify!(limit)): 1u16,
                    (stringify!(direction)): server_admin_contract::admin_sort_direction::AdminSortDirection::Ascending,
                }),
            ),
        ];
        let checks = cases.into_iter().map(async |(uri_text, expected)| {
            let Ok(uri) = uri_text.parse::<http::Uri>() else {
                return false;
            };
            let mut request = axum::extract::Request::new(axum::body::Body::empty());
            *request.uri_mut() = uri;
            let (mut parts, _body) = request.into_parts();
            <crate::axum_admin_query::AxumAdminQuery<
                    server_admin_contract::admin_table_query::AdminTableQuery,
                > as axum::extract::FromRequestParts<
                    server_admin_core::std_admin_bool::StdAdminBool,
                >>::from_request_parts(
                    &mut parts,
                    &server_admin_core::std_admin_bool::StdAdminBool::from(true),
                )
                .await
                .is_ok_and(|axum_admin_query| {
                    serde_json::to_value(axum_admin_query.into_inner())
                        .is_ok_and(|wire| wire == expected)
                })
        });
        assert!(
            futures::future::join_all(checks)
                .await
                .into_iter()
                .all(std::convert::identity)
        );
    }

    #[tokio::test]
    async fn test_admin_query_extractor_maps_invalid_fields_to_validation() {
        let cases = [
            (stringify!(limit), 0u16.to_string()),
            (stringify!(limit), 101u16.to_string()),
            (stringify!(limit), constants_str::ADMIN_ALT.to_owned()),
            (stringify!(offset), (-1i64).to_string()),
            (stringify!(offset), (u64::from(u32::MAX) + 1u64).to_string()),
            (stringify!(direction), constants_str::ADMIN_ALT.to_owned()),
            (stringify!(search), constants_str::A_ALT.repeat(129usize)),
            (stringify!(sort), constants_str::A_ALT.repeat(33usize)),
            (stringify!(limit), format!("1&{}=2", stringify!(limit))),
        ];
        let checks = cases.into_iter().map(async |(field, invalid_value)| {
            let Ok(uri) = format!("{}?{}={}", constants_str::SLASH, field, invalid_value)
                .parse::<http::Uri>()
            else {
                return false;
            };
            let mut request = axum::extract::Request::new(axum::body::Body::empty());
            *request.uri_mut() = uri;
            let (mut parts, _body) = request.into_parts();
            matches!(
                <crate::axum_admin_query::AxumAdminQuery<
                    server_admin_contract::admin_table_query::AdminTableQuery,
                > as axum::extract::FromRequestParts<
                    server_admin_core::std_admin_bool::StdAdminBool,
                >>::from_request_parts(
                    &mut parts,
                    &server_admin_core::std_admin_bool::StdAdminBool::from(false),
                )
                .await,
                Err(crate::admin_error::AdminError::Validation)
            )
        });
        assert!(
            futures::future::join_all(checks)
                .await
                .into_iter()
                .all(std::convert::identity)
        );
    }

    #[tokio::test]
    async fn test_admin_path_extractor_preserves_identifiers_and_rejects_invalid_or_missing_parameters()
     {
        let router = axum::Router::<()>::new().route(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersRead.get(),
            axum::routing::get(
                async |axum_admin_path: crate::axum_admin_path::AxumAdminPath<
                    server_admin_contract::admin_user_id::AdminUserId,
                >| { axum::Json(axum_admin_path.into_inner()) },
            ),
        );
        let cases = [
            (1i64.to_string(), Some(1i64)),
            (i64::MAX.to_string(), Some(i64::MAX)),
            (0i64.to_string(), None),
            ((-1i64).to_string(), None),
            ((i128::from(i64::MAX) + 1i128).to_string(), None),
            (constants_str::ADMIN_ALT.to_owned(), None),
        ];
        let checks = cases.into_iter().map(async |(identifier_text, expected)| {
            let route = server_admin_contract::admin_frontend_path::AdminFrontendPath::UsersRead
                .get()
                .replace(constants_str::ADMIN_USER_ID_PLACEHOLDER, &identifier_text);
            let Ok(uri) = route.parse::<http::Uri>() else {
                return false;
            };
            let mut request = axum::extract::Request::new(axum::body::Body::empty());
            *request.uri_mut() = uri;
            let Ok(response) = tower::ServiceExt::oneshot(router.clone(), request).await;
            match expected {
                Some(identifier) => {
                    if response.status() != http::StatusCode::OK {
                        return false;
                    }
                    axum::body::to_bytes(response.into_body(), 1024usize)
                        .await
                        .is_ok_and(|body| {
                            serde_json::from_slice::<i64>(&body)
                                .is_ok_and(|decoded| decoded == identifier)
                        })
                }
                None => response.status() == http::StatusCode::UNPROCESSABLE_ENTITY,
            }
        });
        assert!(
            futures::future::join_all(checks)
                .await
                .into_iter()
                .all(std::convert::identity)
        );
        let (mut parts, _body) =
            axum::extract::Request::new(axum::body::Body::empty()).into_parts();
        assert!(matches!(
            <crate::axum_admin_path::AxumAdminPath<
                server_admin_contract::admin_user_id::AdminUserId,
            > as axum::extract::FromRequestParts<
                server_admin_core::std_admin_bool::StdAdminBool,
            >>::from_request_parts(
                &mut parts,
                &server_admin_core::std_admin_bool::StdAdminBool::from(false),
            )
            .await,
            Err(crate::admin_error::AdminError::Validation)
        ));
    }
}
