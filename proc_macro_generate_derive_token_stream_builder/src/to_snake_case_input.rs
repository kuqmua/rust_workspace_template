#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct ToSnakeCaseInput<'input_lt>(&'input_lt str);
