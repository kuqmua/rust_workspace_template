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
#[path = "route/route_transport.rs"]
mod route_transport;
pub use route_transport::*;
#[path = "route/public_transport.rs"]
mod public_transport;
pub use public_transport::*;
#[path = "route/authenticated_transport.rs"]
mod authenticated_transport;
pub use authenticated_transport::*;
#[path = "route/route_method.rs"]
mod route_method;
pub use route_method::*;
#[path = "route/axum_method_filter.rs"]
mod axum_method_filter;
pub use axum_method_filter::*;
#[path = "route/route_metadata.rs"]
mod route_metadata;
pub use route_metadata::*;
#[path = "route/utoipa_open_api_components_ref_mut.rs"]
mod utoipa_open_api_components_ref_mut;
pub use utoipa_open_api_components_ref_mut::*;
#[path = "route/utoipa_open_api_ref_mut.rs"]
mod utoipa_open_api_ref_mut;
pub use utoipa_open_api_ref_mut::*;
#[path = "route/typed_route.rs"]
mod typed_route;
pub use typed_route::*;
#[path = "route/route_request_body.rs"]
mod route_request_body;
pub use route_request_body::*;
#[path = "route/route_schema_contract.rs"]
mod route_schema_contract;
pub use route_schema_contract::*;
#[path = "route/utoipa_open_api_route_schema.rs"]
mod utoipa_open_api_route_schema;
pub use utoipa_open_api_route_schema::*;
#[path = "route/utoipa_open_api_path_parameter.rs"]
mod utoipa_open_api_path_parameter;
pub use utoipa_open_api_path_parameter::*;
#[path = "route/parameterized_route_path.rs"]
mod parameterized_route_path;
pub use parameterized_route_path::*;
#[path = "route/parameterized_route_path_try_from_string_error.rs"]
mod parameterized_route_path_try_from_string_error;
pub use parameterized_route_path_try_from_string_error::*;
#[path = "route/open_api_security_scheme_ref.rs"]
mod open_api_security_scheme_ref;
pub use open_api_security_scheme_ref::*;
#[path = "route/covered_route.rs"]
mod covered_route;
pub use covered_route::*;
#[path = "route/parameterized_route.rs"]
mod parameterized_route;
pub use parameterized_route::*;
#[path = "route/route_body_limit.rs"]
mod route_body_limit;
pub use route_body_limit::*;
#[path = "route/route_coverage_descriptors.rs"]
mod route_coverage_descriptors;
pub use route_coverage_descriptors::*;
#[path = "route/route_schema_contracts.rs"]
mod route_schema_contracts;
pub use route_schema_contracts::*;
#[path = "route/route_metadata_list.rs"]
mod route_metadata_list;
pub use route_metadata_list::*;
#[path = "route/route_family.rs"]
mod route_family;
pub use route_family::*;
#[path = "route/route_in_family.rs"]
mod route_in_family;
pub use route_in_family::*;
#[path = "route/route_request.rs"]
mod route_request;
pub use route_request::*;
#[path = "route/route_response.rs"]
mod route_response;
pub use route_response::*;
#[path = "route/client_request.rs"]
mod client_request;
pub use client_request::*;
#[path = "route/server_response.rs"]
mod server_response;
pub use server_response::*;
#[path = "route/client_route_metadata.rs"]
mod client_route_metadata;
pub use client_route_metadata::*;
#[path = "route/server_route_metadata.rs"]
mod server_route_metadata;
pub use server_route_metadata::*;
#[path = "route/openapi_route_metadata.rs"]
mod openapi_route_metadata;
pub use openapi_route_metadata::*;
#[path = "route/apply_openapi_success_contract.rs"]
mod apply_openapi_success_contract;
pub use apply_openapi_success_contract::*;
#[path = "route/apply_openapi_request_contract.rs"]
mod apply_openapi_request_contract;
pub use apply_openapi_request_contract::*;
#[path = "route/register_openapi_schema.rs"]
mod register_openapi_schema;
pub use register_openapi_schema::*;
#[path = "route/register_openapi_route_schemas.rs"]
mod register_openapi_route_schemas;
pub use register_openapi_route_schemas::*;
#[path = "route/apply_openapi_path_parameter_contract.rs"]
mod apply_openapi_path_parameter_contract;
pub use apply_openapi_path_parameter_contract::*;
#[path = "route/apply_openapi_security_contract.rs"]
mod apply_openapi_security_contract;
pub use apply_openapi_security_contract::*;
#[path = "route/apply_openapi_error_contract.rs"]
mod apply_openapi_error_contract;
pub use apply_openapi_error_contract::*;
#[path = "route/typed_route_path.rs"]
mod typed_route_path;
pub use typed_route_path::*;
#[path = "route/typed_parameterized_route_path.rs"]
mod typed_parameterized_route_path;
pub use typed_parameterized_route_path::*;
#[path = "route/functions.rs"]
mod functions;
pub use functions::*;
