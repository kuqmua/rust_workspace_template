fn require_public<Route>(_request: frontend_contract::domain_types::RouteRequest<Route>)
where
    Route: frontend_contract::domain_types::TypedRoute<Transport = frontend_contract::domain_types::PublicTransport>,
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
impl frontend_contract::domain_types::TypedRoute for AuthenticatedRoute {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::domain_types::AuthenticatedTransport;
    fn metadata() -> frontend_contract::domain_types::RouteMetadata { frontend_contract::domain_types::RouteMetadata::new(frontend_contract::domain_types::RouteMethod::Post, frontend_contract::domain_types::ContractStr::from(constants_str::WRITE_ALT), frontend_contract::domain_types::ContractStr::from(constants_str::WRITE)) }
}
fn main() {
    require_public(frontend_contract::domain_types::client_request::<AuthenticatedRoute>(Request));
}
