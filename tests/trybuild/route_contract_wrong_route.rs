#[derive(serde::Serialize, serde::Deserialize)]
struct FirstRequest;
#[derive(serde::Serialize, serde::Deserialize)]
struct SecondRequest;
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
struct FirstRoute;
struct SecondRoute;
impl frontend_contract::TypedRoute for FirstRoute {
    type Request = FirstRequest;
    type Response = Response;
    type Transport = frontend_contract::PublicTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::ContractStr::from(str_constants::expr::S_0672), frontend_contract::ContractStr::from(str_constants::expr::S_1329), frontend_contract::ContractStr::from(str_constants::expr::S_0103)) }
}
impl frontend_contract::TypedRoute for SecondRoute {
    type Request = SecondRequest;
    type Response = Response;
    type Transport = frontend_contract::PublicTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::ContractStr::from(str_constants::expr::S_0672), frontend_contract::ContractStr::from(str_constants::expr::S_1704), frontend_contract::ContractStr::from(str_constants::expr::S_0120)) }
}
fn main() {
    let request = frontend_contract::client_request::<FirstRoute>(FirstRequest);
    let _: frontend_contract::RouteRequest<SecondRoute> = request;
}
