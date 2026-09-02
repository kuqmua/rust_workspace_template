#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::Display,
    proc_macro_newtype::FromInner,
)]
pub struct RegexCasePostgreqlSyntax(&'static str);
