#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct FirstRequest;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct SecondRequest;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct FirstRoute;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct SecondRoute;
impl frontend_contract::typed_route::TypedRoute for FirstRoute {
    type Request = FirstRequest;
    type Response = Response;
    type Transport = frontend_contract::public_transport::PublicTransport;
    fn metadata() -> frontend_contract::route_metadata::RouteMetadata { frontend_contract::route_metadata::RouteMetadata::new(frontend_contract::route_method::RouteMethod::Get, frontend_contract::contract_str::ContractStr::from(constants_str::catalog::FIRST_ALT), frontend_contract::contract_str::ContractStr::from(constants_str::catalog::FIRST)) }
}
impl frontend_contract::typed_route::TypedRoute for SecondRoute {
    type Request = SecondRequest;
    type Response = Response;
    type Transport = frontend_contract::public_transport::PublicTransport;
    fn metadata() -> frontend_contract::route_metadata::RouteMetadata { frontend_contract::route_metadata::RouteMetadata::new(frontend_contract::route_method::RouteMethod::Get, frontend_contract::contract_str::ContractStr::from(constants_str::catalog::SECOND_ALT), frontend_contract::contract_str::ContractStr::from(constants_str::catalog::SECOND)) }
}
fn main() {
    let request = frontend_contract::client_request::client_request::<FirstRoute>(FirstRequest);
    let _: frontend_contract::route_request::RouteRequest<SecondRoute> = request;
}
