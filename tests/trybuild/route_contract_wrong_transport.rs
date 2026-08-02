fn require_public<Route>(_request: frontend_contract::RouteRequest<Route>)
where
    Route: frontend_contract::TypedRoute<Transport = frontend_contract::PublicTransport>,
{
}
#[derive(optml::Optml)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(optml::Optml)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
#[derive(optml::Optml)]
struct AuthenticatedRoute;
impl frontend_contract::TypedRoute for AuthenticatedRoute {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::AuthenticatedTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::RouteMethod::Post, frontend_contract::ContractStr::from(str_constants::WRITE_ALT), frontend_contract::ContractStr::from(str_constants::WRITE)) }
}
fn main() {
    require_public(frontend_contract::client_request::<AuthenticatedRoute>(Request));
}
