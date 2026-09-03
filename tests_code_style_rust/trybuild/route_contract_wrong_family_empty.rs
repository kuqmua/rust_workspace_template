#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_frontend_contract_derive_route_family::RouteFamily)]
#[route_family()]
struct EmptyRouteFamily;

fn main() {
    let _arguments = std::env::args_os();
}
