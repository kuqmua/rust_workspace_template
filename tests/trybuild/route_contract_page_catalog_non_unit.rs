#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(frontend_contract::domain_types::PageCatalog)]
#[page_catalog(
    spec = BrokenSpec,
    path_ref = BrokenPathRef,
    inventory = broken_inventory
)]
enum BrokenPages {
    #[page_catalog_page(path = "/broken", title = "Broken")]
    Broken(frontend_contract::domain_types::ContractStr),
}

fn main() {}
