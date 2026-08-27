#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::AsRefOwned, newtype::FromInner,
)]
pub(crate) struct QuoteTokenStreamGeneratePgTableMeasureInputTokenStream(
    quote::__private::TokenStream,
);
