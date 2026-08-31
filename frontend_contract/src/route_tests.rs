#![allow(
    clippy::items_after_test_module,
    reason = "the compatibility facade follows the retained route contract unit-test module"
)]

#[cfg(test)]
mod tests {
    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    #[serde(from = "u64")]
    #[derive(newtype::FromInner)]
    struct Request(u64);

    #[derive(
        optimal_memory_layout::OptimalMemoryLayout,
        Clone,
        Debug,
        Eq,
        PartialEq,
        serde::Deserialize,
        serde::Serialize,
    )]
    #[serde(from = "u64")]
    #[derive(newtype::FromInner)]
    struct Response(u64);

    #[derive(optimal_memory_layout::OptimalMemoryLayout)]
    struct Route;
    impl crate::typed_route::TypedRoute for Route {
        type Request = Request;
        type Response = Response;
        type Transport = crate::public_transport::PublicTransport;
        fn metadata() -> crate::route_metadata::RouteMetadata {
            crate::route_metadata::RouteMetadata::new(
                crate::route_method::RouteMethod::Get,
                crate::contract_str::ContractStr::from(constants_str::ROUTE_READ),
                crate::contract_str::ContractStr::from(constants_str::ROUTE),
            )
        }
    }
    #[test]
    fn test_matching_request_response_and_metadata_share_one_route_contract() {
        let request = crate::client_request::client_request::<Route>(Request::from(1u64));
        let response = crate::server_response::server_response::<Route>(Response::from(2u64));
        assert_eq!(request.body(), &Request(1u64));
        assert_eq!(response.body(), &Response(2u64));
        assert_eq!(
            <Route as crate::typed_route::TypedRoute>::metadata()
                .path()
                .as_ref(),
            "/route"
        );
        assert_eq!(
            crate::client_route_metadata::client_route_metadata::<Route>(),
            crate::server_route_metadata::server_route_metadata::<Route>()
        );
        assert_eq!(
            crate::server_route_metadata::server_route_metadata::<Route>(),
            crate::openapi_route_metadata::openapi_route_metadata::<Route>()
        );
    }
}
