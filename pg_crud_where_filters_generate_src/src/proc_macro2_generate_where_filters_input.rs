#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct ProcMacro2GenerateWhereFiltersInput<'input_lt>(&'input_lt proc_macro2::TokenStream);
