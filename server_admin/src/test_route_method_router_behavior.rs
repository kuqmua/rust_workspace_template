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
}
