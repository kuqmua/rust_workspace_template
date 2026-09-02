#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct QuoteTokenStreamGeneratePgTableMeasureInputTokenStream(
    quote::__private::TokenStream,
);
