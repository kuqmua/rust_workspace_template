#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct OptimalLayoutAcceptedDescendingFields {
    first: u64,
    second: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[optimal_memory_layout(skip)]
struct OptimalLayoutAcceptedSkippedFields {
    first: u8,
    second: u64,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
#[optimal_memory_layout(skip)]
enum OptimalLayoutAcceptedSkippedVariant {
    Value {
        first: u8,
        second: u64,
    },
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct OptimalLayoutAcceptedGenericFields<Value> {
    first: u8,
    second: Value,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct OptimalLayoutAcceptedUnit;

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct OptimalLayoutAcceptedTuple(u8, u64);

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct OptimalLayoutAcceptedSingleField {
    value: u8,
}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct OptimalLayoutAcceptedEmptyFields {}

#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
enum OptimalLayoutAcceptedMixedVariants {
    Tuple(u8, u64),
    Named {
        first: u64,
        second: u8,
    },
    Unit,
    Single {
        value: u8,
    },
}

fn main() {
    let _sizes = [
        std::mem::size_of::<OptimalLayoutAcceptedDescendingFields>(),
        std::mem::size_of::<OptimalLayoutAcceptedSkippedFields>(),
        std::mem::size_of::<OptimalLayoutAcceptedSkippedVariant>(),
        std::mem::size_of::<OptimalLayoutAcceptedGenericFields<u64>>(),
        std::mem::size_of::<OptimalLayoutAcceptedUnit>(),
        std::mem::size_of::<OptimalLayoutAcceptedTuple>(),
        std::mem::size_of::<OptimalLayoutAcceptedSingleField>(),
        std::mem::size_of::<OptimalLayoutAcceptedEmptyFields>(),
        std::mem::size_of::<OptimalLayoutAcceptedMixedVariants>(),
    ];
}
