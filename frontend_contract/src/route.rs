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
                crate::domain_types::ContractStr::from(constants_str::ROUTE_READ),
                crate::domain_types::ContractStr::from(constants_str::ROUTE),
            )
        }
    }
    #[test]
    fn matching_request_response_and_metadata_share_one_route_contract() {
        let request = super::client_request::<Route>(Request::from(1u64));
        let response = super::server_response::<Route>(Response::from(2u64));
        assert_eq!(request.body(), &Request(1u64));
        assert_eq!(response.body(), &Response(2u64));
        assert_eq!(
            <Route as super::TypedRoute>::metadata().path().as_ref(),
            "/route"
        );
        assert_eq!(
            super::client_route_metadata::<Route>(),
            super::server_route_metadata::<Route>()
        );
        assert_eq!(
            super::server_route_metadata::<Route>(),
            super::openapi_route_metadata::<Route>()
        );
    }
}
#[path = "route_transport.rs"]
mod route_transport;
pub use route_transport::*;
#[path = "public_transport.rs"]
mod public_transport;
pub use public_transport::*;
#[path = "authenticated_transport.rs"]
mod authenticated_transport;
pub use authenticated_transport::*;
#[path = "route_method.rs"]
mod route_method;
pub use route_method::*;
#[path = "axum_method_filter.rs"]
mod axum_method_filter;
pub use axum_method_filter::*;
#[path = "route_metadata.rs"]
mod route_metadata;
pub use route_metadata::*;
#[path = "utoipa_open_api_components_ref_mut.rs"]
mod utoipa_open_api_components_ref_mut;
pub use utoipa_open_api_components_ref_mut::*;
#[path = "utoipa_open_api_ref_mut.rs"]
mod utoipa_open_api_ref_mut;
pub use utoipa_open_api_ref_mut::*;
#[path = "typed_route.rs"]
mod typed_route;
pub use typed_route::*;
#[path = "route_request_body.rs"]
mod route_request_body;
pub use route_request_body::*;
#[path = "route_schema_contract.rs"]
mod route_schema_contract;
pub use route_schema_contract::*;
#[path = "utoipa_open_api_route_schema.rs"]
mod utoipa_open_api_route_schema;
pub use utoipa_open_api_route_schema::*;
#[path = "utoipa_open_api_path_parameter.rs"]
mod utoipa_open_api_path_parameter;
pub use utoipa_open_api_path_parameter::*;
#[path = "parameterized_route_path.rs"]
mod parameterized_route_path;
pub use parameterized_route_path::*;
#[path = "parameterized_route_path_try_from_string_error.rs"]
mod parameterized_route_path_try_from_string_error;
pub use parameterized_route_path_try_from_string_error::*;
#[path = "open_api_security_scheme_ref.rs"]
mod open_api_security_scheme_ref;
pub use open_api_security_scheme_ref::*;
#[path = "covered_route.rs"]
mod covered_route;
pub use covered_route::*;
#[path = "parameterized_route.rs"]
mod parameterized_route;
pub use parameterized_route::*;
#[path = "route_body_limit.rs"]
mod route_body_limit;
pub use route_body_limit::*;
#[path = "route_coverage_descriptors.rs"]
mod route_coverage_descriptors;
pub use route_coverage_descriptors::*;
#[path = "route_schema_contracts.rs"]
mod route_schema_contracts;
pub use route_schema_contracts::*;
#[path = "route_metadata_list.rs"]
mod route_metadata_list;
pub use route_metadata_list::*;
#[path = "route_family.rs"]
mod route_family;
pub use route_family::*;
#[path = "route_in_family.rs"]
mod route_in_family;
pub use route_in_family::*;
#[path = "route_request.rs"]
mod route_request;
pub use route_request::*;
#[path = "route_response.rs"]
mod route_response;
pub use route_response::*;
#[path = "client_request.rs"]
mod client_request;
pub use client_request::*;
#[path = "server_response.rs"]
mod server_response;
pub use server_response::*;
#[path = "client_route_metadata.rs"]
mod client_route_metadata;
pub use client_route_metadata::*;
#[path = "server_route_metadata.rs"]
mod server_route_metadata;
pub use server_route_metadata::*;
#[path = "openapi_route_metadata.rs"]
mod openapi_route_metadata;
pub use openapi_route_metadata::*;
#[path = "apply_openapi_success_contract.rs"]
mod apply_openapi_success_contract;
pub use apply_openapi_success_contract::*;
#[path = "apply_openapi_request_contract.rs"]
mod apply_openapi_request_contract;
pub use apply_openapi_request_contract::*;
#[path = "register_openapi_schema.rs"]
mod register_openapi_schema;
pub use register_openapi_schema::*;
#[path = "register_openapi_route_schemas.rs"]
mod register_openapi_route_schemas;
pub use register_openapi_route_schemas::*;
#[path = "apply_openapi_path_parameter_contract.rs"]
mod apply_openapi_path_parameter_contract;
pub use apply_openapi_path_parameter_contract::*;
#[path = "apply_openapi_security_contract.rs"]
mod apply_openapi_security_contract;
pub use apply_openapi_security_contract::*;
#[path = "apply_openapi_error_contract.rs"]
mod apply_openapi_error_contract;
pub use apply_openapi_error_contract::*;
#[path = "typed_route_path.rs"]
mod typed_route_path;
pub use typed_route_path::*;
#[path = "typed_parameterized_route_path.rs"]
mod typed_parameterized_route_path;
pub use typed_parameterized_route_path::*;
#[path = "functions.rs"]
mod functions;
pub use functions::*;
