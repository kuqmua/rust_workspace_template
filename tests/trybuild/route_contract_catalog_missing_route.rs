#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(frontend_contract::domain_types::RouteCatalog)]
#[route_catalog(family = BrokenFamily, body_limit = 1024usize)]
enum BrokenCatalog {
    MissingRouteAttribute,
}

fn main() {}
