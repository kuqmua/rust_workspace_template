#[derive(frontend_contract::PageCatalog)]
#[page_catalog(
    spec = BrokenSpec,
    path_ref = BrokenPathRef,
    inventory = broken_inventory
)]
enum BrokenPages {
    #[page_catalog_page(path = "/broken", title = "Broken")]
    Broken(frontend_contract::ContractStr),
}

fn main() {}
