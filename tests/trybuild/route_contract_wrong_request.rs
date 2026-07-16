#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
struct Route;
impl frontend_contract::TypedRoute for Route {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::PublicTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::RouteMethod::Get, frontend_contract::ContractStr::from(str_constants::PG_CRUD_READ_PERMISSION_ACTION), frontend_contract::ContractStr::from(str_constants::READ)) }
}
fn main() {
    let _request = frontend_contract::client_request::<Route>(Response);
}
