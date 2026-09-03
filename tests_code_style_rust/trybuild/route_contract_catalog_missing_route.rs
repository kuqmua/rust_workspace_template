#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_frontend_contract_derive_route_catalog::RouteCatalog)]
#[route_catalog(family = BrokenFamily, body_limit = 1024usize)]
enum BrokenCatalog {
    MissingRouteAttribute,
}

fn main() {
    let _arguments = std::env::args_os();
}
