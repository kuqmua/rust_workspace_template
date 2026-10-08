#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
enum ConstructorRejectedEnum {
    Value,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedUnit;

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedTuple(u8);

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedNestedOrder {
    #[constructor(order = 0, order = 1)]
    value: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedRepeatedOrder {
    #[constructor(order = 0)]
    #[constructor(order = 0)]
    value: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedUnknownAttribute {
    #[constructor(unknown = 0)]
    value: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedOrderGap {
    #[constructor(order = 2)]
    value: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedDuplicatePositions {
    #[constructor(order = 0)]
    first: u8,
    #[constructor(order = 0)]
    second: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
struct ConstructorRejectedNonIntegerOrder {
    #[constructor(order = false)]
    value: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_new::New)]
#[constructor(invalid)]
struct ConstructorRejectedVisibility {
    value: u8,
}

fn main() {
    let _arguments = std::env::args_os();
}
