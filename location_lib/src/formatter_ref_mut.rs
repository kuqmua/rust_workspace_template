#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(super) struct FormatterRefMut<'fmt_ref_lt, 'fmt_lt>(
    &'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>,
);
