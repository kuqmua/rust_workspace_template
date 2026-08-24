#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct Route;
impl frontend_contract::TypedRoute for Route {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::PublicTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::RouteMethod::Get, frontend_contract::ContractStr::from(str_constants::PG_CRUD_READ_PERMISSION_ACTION), frontend_contract::ContractStr::from(str_constants::READ)) }
}
fn main() {
    let _response = frontend_contract::server_response::<Route>(Request);
}
