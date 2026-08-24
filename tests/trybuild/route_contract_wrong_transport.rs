fn require_public<Route>(_request: frontend_contract::RouteRequest<Route>)
where
    Route: frontend_contract::TypedRoute<Transport = frontend_contract::PublicTransport>,
{
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AuthenticatedRoute;
impl frontend_contract::TypedRoute for AuthenticatedRoute {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::AuthenticatedTransport;
    fn metadata() -> frontend_contract::RouteMetadata { frontend_contract::RouteMetadata::new(frontend_contract::RouteMethod::Post, frontend_contract::ContractStr::from(constants_str::WRITE_ALT), frontend_contract::ContractStr::from(constants_str::WRITE)) }
}
fn main() {
    require_public(frontend_contract::client_request::<AuthenticatedRoute>(Request));
}
