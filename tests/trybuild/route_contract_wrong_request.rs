#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Route;
impl frontend_contract::domain_types::TypedRoute for Route {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::domain_types::PublicTransport;
    fn metadata() -> frontend_contract::domain_types::RouteMetadata { frontend_contract::domain_types::RouteMetadata::new(frontend_contract::domain_types::RouteMethod::Get, frontend_contract::domain_types::ContractStr::from(constants_str::PG_CRUD_READ_PERMISSION_ACTION), frontend_contract::domain_types::ContractStr::from(constants_str::READ)) }
}
fn main() {
    let _request = frontend_contract::domain_types::client_request::<Route>(Response);
}
