#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::AsRefOwned,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct ProcMacro2GenerateWhereFiltersTokenStream(proc_macro2::TokenStream);
