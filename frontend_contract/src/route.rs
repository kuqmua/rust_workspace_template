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
    impl super::TypedRoute for Route {
        type Request = Request;
        type Response = Response;
        type Transport = super::PublicTransport;
        fn metadata() -> super::RouteMetadata {
            super::RouteMetadata::new(
                super::RouteMethod::Get,
                crate::ContractStr::from(constants_str::ROUTE_READ),
                crate::ContractStr::from(constants_str::ROUTE),
            )
        }
    }
    #[test]
    fn matching_request_response_and_metadata_share_one_route_contract() {
        let request = crate::client_request::<Route>(Request::from(1u64));
        let response = crate::server_response::<Route>(Response::from(2u64));
        assert_eq!(request.body(), &Request(1u64));
        assert_eq!(response.body(), &Response(2u64));
        assert_eq!(
            <Route as super::TypedRoute>::metadata().path().as_ref(),
            "/route"
        );
        assert_eq!(
            crate::client_route_metadata::<Route>(),
            crate::server_route_metadata::<Route>()
        );
        assert_eq!(
            crate::server_route_metadata::<Route>(),
            crate::openapi_route_metadata::<Route>()
        );
    }
}
pub use super::apply_openapi_error_contract_fn::*;
pub use super::apply_openapi_path_parameter_contract_fn::*;
pub use super::apply_openapi_request_contract_fn::*;
pub use super::apply_openapi_security_contract_fn::*;
pub use super::apply_openapi_success_contract_fn::*;
pub use super::authenticated_transport::*;
pub use super::axum_method_filter::*;
pub use super::client_request_fn::*;
pub use super::client_route_metadata_fn::*;
pub use super::covered_route::*;
pub use super::open_api_security_scheme_ref::*;
pub use super::openapi_route_metadata_fn::*;
pub use super::parameterized_route::*;
pub use super::parameterized_route_path::*;
pub use super::parameterized_route_path_try_from_string_error::*;
pub use super::public_transport::*;
pub use super::register_openapi_route_schemas_fn::*;
pub use super::register_openapi_schema_fn::*;
pub use super::route_body_limit::*;
pub use super::route_coverage_descriptors::*;
pub use super::route_family::*;
pub use super::route_in_family::*;
pub use super::route_metadata::*;
pub use super::route_metadata_list::*;
pub use super::route_method::*;
pub use super::route_request::*;
pub use super::route_request_body::*;
pub use super::route_response::*;
pub use super::route_schema_contract::*;
pub use super::route_schema_contracts::*;
pub use super::route_transport::*;
pub use super::server_response_fn::*;
pub use super::server_route_metadata_fn::*;
pub use super::to_axum_method_filter_fn::*;
pub use super::typed_parameterized_route_path_fn::*;
pub use super::typed_route::*;
pub use super::typed_route_path_fn::*;
pub use super::utoipa_open_api_components_ref_mut::*;
pub use super::utoipa_open_api_path_parameter::*;
pub use super::utoipa_open_api_ref_mut::*;
pub use super::utoipa_open_api_route_schema::*;
