#[derive(optml::Optml)]
#[derive(frontend_contract::RouteCatalog)]
#[route_catalog(family = BrokenFamily, body_limit = 1024usize)]
enum BrokenCatalog {
    MissingRouteAttribute,
}

fn main() {}
