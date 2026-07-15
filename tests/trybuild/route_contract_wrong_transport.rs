fn require_public<Route>(_request: frontend_contract::RouteRequest<Route>)
where
    Route: frontend_contract::TypedRoute<Transport = frontend_contract::PublicTransport>,
{
}
#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
struct AuthenticatedRoute;
impl frontend_contract::TypedRoute for AuthenticatedRoute {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::AuthenticatedTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::ContractStr::from(str_constants::expr::S_0722), frontend_contract::ContractStr::from(str_constants::expr::S_1915), frontend_contract::ContractStr::from(str_constants::expr::S_0135)) }
}
fn main() {
    require_public(frontend_contract::client_request::<AuthenticatedRoute>(Request));
}
