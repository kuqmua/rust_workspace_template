#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[derive(proc_macro_frontend_contract::RouteFamily)]
struct MissingAttributeRouteFamily;

fn main() {
    let _arguments = std::env::args_os();
}
