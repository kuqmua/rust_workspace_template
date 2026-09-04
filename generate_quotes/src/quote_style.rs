#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, proc_macro_getters::Getters,
)]
#[getters(bare)]
pub(super) struct QuoteStyle {
    #[getters(copy)]
    panic_id: crate::quote_panic_id::QuotePanicId,
    #[getters(copy)]
    prefix: crate::quote_prefix::QuotePrefix,
    #[getters(copy)]
    quote_ch: crate::quote_char::QuoteChar,
}

impl
    From<(
        crate::quote_panic_id::QuotePanicId,
        crate::quote_prefix::QuotePrefix,
        crate::quote_char::QuoteChar,
    )> for QuoteStyle
{
    fn from(
        value: (
            crate::quote_panic_id::QuotePanicId,
            crate::quote_prefix::QuotePrefix,
            crate::quote_char::QuoteChar,
        ),
    ) -> Self {
        Self {
            panic_id: value.0,
            prefix: value.1,
            quote_ch: value.2,
        }
    }
}
