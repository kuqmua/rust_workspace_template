#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Route;
impl frontend_contract::typed_route::TypedRoute for Route {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::public_transport::PublicTransport;
    fn metadata() -> frontend_contract::route_metadata::RouteMetadata { frontend_contract::route_metadata::RouteMetadata::new(frontend_contract::route_method::RouteMethod::Get, frontend_contract::contract_str::ContractStr::from(constants_str::catalog::PG_CRUD_READ_PERMISSION_ACTION), frontend_contract::contract_str::ContractStr::from(constants_str::catalog::READ)) }
}
fn main() {
    let _request = frontend_contract::client_request::client_request::<Route>(Response);
}
