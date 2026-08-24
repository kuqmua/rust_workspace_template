#[tokio::test]
async fn protected_routes_reject_missing_authentication_without_database_io() {
    let users_response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("b319e84d protected_routes_reject_missing_authentication_without_database_io invariant must hold"),
    )
    .await
    .expect("0ac617de protected_routes_reject_missing_authentication_without_database_io invariant must hold");
    assert_eq!(users_response.status(), http::StatusCode::UNAUTHORIZED);
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("895e12fc protected_routes_reject_missing_authentication_without_database_io invariant must hold"),
    )
    .await
    .expect("1fe80ad3 protected_routes_reject_missing_authentication_without_database_io invariant must hold");
    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}
#[tokio::test]
#[allow(
    clippy::needless_for_each,
    reason = "repository policy requires iterator methods instead of for loops"
)]
async fn runtime_auth_router_contains_every_open_api_operation() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        server_admin::auth::open_api(),
    ))
    .expect("71599514 runtime_auth_router_contains_every_open_api_operation invariant must hold");
    let paths = document
        .get(str_constants::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect(
            "d908872f runtime_auth_router_contains_every_open_api_operation invariant must hold",
        );
    let responses = futures::future::join_all(
        paths
            .iter()
            .flat_map(|(documented_path, path_item)| {
                path_item
                    .as_object()
                    .into_iter()
                    .flat_map(|operation_map| operation_map.keys())
                    .map(move |method| (documented_path, method))
            })
            .map(|(path, method)| (path.to_owned(), method.to_owned()))
            .map(|(documented_path, documented_method)| {
                let runtime_path = documented_path
                    .replace(
                        str_constants::ADMIN_SESSION_ID_PLACEHOLDER,
                        str_constants::VALUE_1,
                    )
                    .replace(
                        str_constants::ADMIN_USER_ID_PLACEHOLDER,
                        str_constants::VALUE_1,
                    )
                    .replace(
                        str_constants::ADMIN_ROLE_ID_PLACEHOLDER,
                        str_constants::VALUE_1,
                    );
                let method =
                    http::Method::from_bytes(documented_method.to_ascii_uppercase().as_bytes())
                        .expect("9d31a7e4 runtime_auth_router_contains_every_open_api_operation invariant must hold");
                async move {
                    (
                        documented_method,
                        documented_path,
                        tower::ServiceExt::oneshot(
                            router().0,
                            http::Request::builder()
                                .method(method)
                                .uri(runtime_path)
                                .body(axum::body::Body::empty())
                                .expect("a3d6fb65 runtime_auth_router_contains_every_open_api_operation invariant must hold"),
                        )
                        .await,
                    )
                }
            }),
    )
    .await;
    responses.into_iter().for_each(|(method, path, response)| {
        let status = response.expect("f7bd9f15 runtime_auth_router_contains_every_open_api_operation invariant must hold").status();
        assert!(
            status != http::StatusCode::METHOD_NOT_ALLOWED && status != http::StatusCode::NOT_FOUND,
            "runtime router does not expose documented operation {method} {path}"
        );
    });
}
#[tokio::test]
async fn invalid_access_cookie_is_rejected_before_database_io() {
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            )
            .header(
                http::header::COOKIE,
                str_constants::ADMIN_ACCESS_TOKEN_INVALID_JWT_TOKEN,
            )
            .body(axum::body::Body::empty())
            .expect(
                "819acd53 invalid_access_cookie_is_rejected_before_database_io invariant must hold",
            ),
    )
    .await
    .expect("c3af0891 invalid_access_cookie_is_rejected_before_database_io invariant must hold");
    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn unknown_admin_api_route_is_not_captured_by_spa_fallback() {
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(str_constants::NOT_AN_API_ROUTE)
            .body(axum::body::Body::empty())
            .expect("1ca76f8d unknown_admin_api_route_is_not_captured_by_spa_fallback invariant must hold"),
    )
    .await
    .expect("ce417390 unknown_admin_api_route_is_not_captured_by_spa_fallback invariant must hold");
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn wrong_admin_http_method_uses_problem_details_contract() {
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .method(http::Method::GET)
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("4eb1c098 wrong_admin_http_method_uses_problem_details_contract invariant must hold"),
    )
    .await
    .expect("6764152a wrong_admin_http_method_uses_problem_details_contract invariant must hold");
    assert_eq!(response.status(), http::StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(
        response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static("application/problem+json")),
    );
}
#[tokio::test]
async fn invalid_admin_json_uses_problem_details_and_body_limit_contract() {
    let malformed_response = tower::ServiceExt::oneshot(
        router().0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_ALT),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("5fb0627d invalid_admin_json_uses_problem_details_and_body_limit_contract invariant must hold");
    assert_eq!(
        malformed_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        malformed_response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static("application/problem+json")),
    );
    let body_limit = <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
        .expect("a60751db invalid_admin_json_uses_problem_details_and_body_limit_contract invariant must hold")
        .get();
    let oversized_password = str_constants::X.repeat(body_limit.saturating_add(1usize));
    let oversized_body = format!(r#"{{"login":"admin","password":"{oversized_password}"}}"#);
    let oversized_response = tower::ServiceExt::oneshot(
        router().0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(oversized_body.as_str()),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("fcd3dd3f invalid_admin_json_uses_problem_details_and_body_limit_contract invariant must hold");
    assert_eq!(
        oversized_response.status(),
        http::StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(
        oversized_response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static("application/problem+json")),
    );
}
#[tokio::test]
async fn sign_in_requires_trusted_origin_without_database_io() {
    let make_request = |origin, referer| {
        let mut builder = http::Request::builder()
            .method(http::Method::POST)
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            )
            .header(http::header::CONTENT_TYPE, str_constants::APPLICATION_JSON);
        if let Some(value) = origin {
            builder = builder.header(http::header::ORIGIN, value);
        }
        if let Some(value) = referer {
            builder = builder.header(http::header::REFERER, value);
        }
        let mut request = builder
            .body(axum::body::Body::from(
                str_constants::LOGIN_ADMIN_PASSWORD_PASSWORD,
            ))
            .expect(
                "168060a3 sign_in_requires_trusted_origin_without_database_io invariant must hold",
            );
        let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
            str_constants::VALUE_127_0_0_1_43210
                .parse::<std::net::SocketAddr>()
                .expect("c90cba14 sign_in_requires_trusted_origin_without_database_io invariant must hold"),
        ));
        request
    };
    let missing_origin_response = tower::ServiceExt::oneshot(router().0, make_request(None, None))
        .await
        .expect("ed2f56fb sign_in_requires_trusted_origin_without_database_io invariant must hold");
    assert_eq!(
        missing_origin_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let blocked_origin_response = tower::ServiceExt::oneshot(
        router().0,
        make_request(
            Some(str_constants::HTTP_BLOCKED_EXAMPLE),
            Some(str_constants::HTTP_LOCALHOST_ADMIN_SIGN_IN),
        ),
    )
    .await
    .expect("df43c793 sign_in_requires_trusted_origin_without_database_io invariant must hold");
    assert_eq!(
        blocked_origin_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
}
#[cfg(test)]
use super::{HttpAdminApiTestMethod, StdAdminApiTestStrRef, request_with_peer, router};
