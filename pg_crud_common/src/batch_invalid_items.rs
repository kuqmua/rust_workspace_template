#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct BatchInvalidItems<InvalidItem>(Vec<InvalidItem>);
