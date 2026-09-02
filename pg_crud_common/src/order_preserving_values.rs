#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_getters::Getters,
)]
#[getters(get_mut)]
pub struct OrderPreservingValues<Value>(Vec<Value>);
