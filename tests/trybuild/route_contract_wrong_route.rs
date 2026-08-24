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
impl frontend_contract::TypedRoute for FirstRoute {
    type Request = FirstRequest;
    type Response = Response;
    type Transport = frontend_contract::PublicTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::RouteMethod::Get, frontend_contract::ContractStr::from(str_constants::FIRST_ALT), frontend_contract::ContractStr::from(str_constants::FIRST)) }
}
impl frontend_contract::TypedRoute for SecondRoute {
    type Request = SecondRequest;
    type Response = Response;
    type Transport = frontend_contract::PublicTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::RouteMethod::Get, frontend_contract::ContractStr::from(str_constants::SECOND_ALT), frontend_contract::ContractStr::from(str_constants::SECOND)) }
}
fn main() {
    let request = frontend_contract::client_request::<FirstRoute>(FirstRequest);
    let _: frontend_contract::RouteRequest<SecondRoute> = request;
}
