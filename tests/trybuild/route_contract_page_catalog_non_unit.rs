#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[derive(frontend_contract_macros::PageCatalog)]
#[page_catalog(
    spec = BrokenSpec,
    path_ref = BrokenPathRef,
    inventory = broken_inventory
)]
enum BrokenPages {
    #[page_catalog_page(path = "/broken", title = "Broken")]
    Broken(frontend_contract::contract_str::ContractStr),
}

fn main() {}
