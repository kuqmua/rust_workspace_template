#[derive(
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
pub struct StdFmtArguments<'arguments>(std::fmt::Arguments<'arguments>);
