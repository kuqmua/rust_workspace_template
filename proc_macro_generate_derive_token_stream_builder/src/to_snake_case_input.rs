#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct ToSnakeCaseInput<'input_lt>(&'input_lt str);
