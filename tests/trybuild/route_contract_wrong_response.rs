#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
struct Route;
impl frontend_contract::TypedRoute for Route {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::PublicTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::ContractStr::from(str_constants::text::GET), frontend_contract::ContractStr::from(str_constants::pg_crud::READ_PERMISSION_ACTION), frontend_contract::ContractStr::from(str_constants::text::READ)) }
}
fn main() {
    let _response = frontend_contract::server_response::<Route>(Request);
}
