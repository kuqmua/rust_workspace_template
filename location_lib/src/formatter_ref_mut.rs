#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub(super) struct FormatterRefMut<'fmt_ref_lt, 'fmt_lt>(
    &'fmt_ref_lt mut std::fmt::Formatter<'fmt_lt>,
);
