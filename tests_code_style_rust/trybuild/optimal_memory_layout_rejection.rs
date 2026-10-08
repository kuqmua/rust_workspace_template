#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct OptimalLayoutRejectedNamedFields {
    first: u8,
    second: u64,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
enum OptimalLayoutRejectedVariantFields {
    Value {
        first: u8,
        second: u64,
    },
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[optimal_memory_layout(unknown)]
struct OptimalLayoutRejectedUnknownAttribute {
    value: u8,
}

fn main() {
    let _arguments = std::env::args_os();
}
