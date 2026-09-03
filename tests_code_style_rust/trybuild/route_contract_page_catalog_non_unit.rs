#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_frontend_contract_derive_page_catalog::PageCatalog)]
#[page_catalog(
    spec = BrokenSpec,
    path_ref = BrokenPathRef,
    inventory = broken_inventory
)]
enum BrokenPages {
    #[page_catalog_page(path = "/broken", title = "Broken")]
    Broken(frontend_contract::contract_str::ContractStr),
}

fn main() {
    let _arguments = std::env::args_os();
}
