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
impl frontend_contract::domain_types::TypedRoute for FirstRoute {
    type Request = FirstRequest;
    type Response = Response;
    type Transport = frontend_contract::domain_types::PublicTransport;
    fn metadata() -> frontend_contract::domain_types::RouteMetadata { frontend_contract::domain_types::RouteMetadata::new(frontend_contract::domain_types::RouteMethod::Get, frontend_contract::domain_types::ContractStr::from(constants_str::FIRST_ALT), frontend_contract::domain_types::ContractStr::from(constants_str::FIRST)) }
}
impl frontend_contract::domain_types::TypedRoute for SecondRoute {
    type Request = SecondRequest;
    type Response = Response;
    type Transport = frontend_contract::domain_types::PublicTransport;
    fn metadata() -> frontend_contract::domain_types::RouteMetadata { frontend_contract::domain_types::RouteMetadata::new(frontend_contract::domain_types::RouteMethod::Get, frontend_contract::domain_types::ContractStr::from(constants_str::SECOND_ALT), frontend_contract::domain_types::ContractStr::from(constants_str::SECOND)) }
}
fn main() {
    let request = frontend_contract::domain_types::client_request::<FirstRoute>(FirstRequest);
    let _: frontend_contract::domain_types::RouteRequest<SecondRoute> = request;
}
