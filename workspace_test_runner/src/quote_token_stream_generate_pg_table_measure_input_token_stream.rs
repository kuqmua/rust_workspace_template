#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct QuoteTokenStreamGeneratePgTableMeasureInputTokenStream(
    quote::__private::TokenStream,
);
