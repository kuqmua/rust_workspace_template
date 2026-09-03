#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(super) struct FormatterRefMut<'fmt_ref_lt, 'fmt_lt>(
    &'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>,
);
