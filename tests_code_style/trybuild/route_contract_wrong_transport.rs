fn require_public<Route>(_request: frontend_contract::route_request::RouteRequest<Route>)
where
    Route: frontend_contract::typed_route::TypedRoute<Transport = frontend_contract::public_transport::PublicTransport>,
{
}
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Request;
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Response;
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct AuthenticatedRoute;
impl frontend_contract::typed_route::TypedRoute for AuthenticatedRoute {
    type Request = Request;
    type Response = Response;
    type Transport = frontend_contract::authenticated_transport::AuthenticatedTransport;
    fn metadata() -> frontend_contract::route_metadata::RouteMetadata { frontend_contract::route_metadata::RouteMetadata::new(frontend_contract::route_method::RouteMethod::Post, frontend_contract::contract_str::ContractStr::from(constants_str::WRITE_ALT), frontend_contract::contract_str::ContractStr::from(constants_str::WRITE)) }
}
fn main() {
    require_public(frontend_contract::client_request::client_request::<AuthenticatedRoute>(Request));
}
