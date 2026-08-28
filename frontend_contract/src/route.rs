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
pub use crate::apply_openapi_error_contract_fn::*;
pub use crate::apply_openapi_path_parameter_contract_fn::*;
pub use crate::apply_openapi_request_contract_fn::*;
pub use crate::apply_openapi_security_contract_fn::*;
pub use crate::apply_openapi_success_contract_fn::*;
pub use crate::authenticated_transport::*;
pub use crate::axum_method_filter::*;
pub use crate::client_request_fn::*;
pub use crate::client_route_metadata_fn::*;
pub use crate::covered_route::*;
pub use crate::open_api_security_scheme_ref::*;
pub use crate::openapi_route_metadata_fn::*;
pub use crate::parameterized_route::*;
pub use crate::parameterized_route_path::*;
pub use crate::parameterized_route_path_try_from_string_error::*;
pub use crate::public_transport::*;
pub use crate::register_openapi_route_schemas_fn::*;
pub use crate::register_openapi_schema_fn::*;
pub use crate::route_body_limit::*;
pub use crate::route_coverage_descriptors::*;
pub use crate::route_family::*;
pub use crate::route_in_family::*;
pub use crate::route_metadata::*;
pub use crate::route_metadata_list::*;
pub use crate::route_method::*;
pub use crate::route_request::*;
pub use crate::route_request_body::*;
pub use crate::route_response::*;
pub use crate::route_schema_contract::*;
pub use crate::route_schema_contracts::*;
pub use crate::route_transport::*;
pub use crate::server_response_fn::*;
pub use crate::server_route_metadata_fn::*;
pub use crate::to_axum_method_filter_fn::*;
pub use crate::typed_parameterized_route_path_fn::*;
pub use crate::typed_route::*;
pub use crate::typed_route_path_fn::*;
pub use crate::utoipa_open_api_components_ref_mut::*;
pub use crate::utoipa_open_api_path_parameter::*;
pub use crate::utoipa_open_api_ref_mut::*;
pub use crate::utoipa_open_api_route_schema::*;
