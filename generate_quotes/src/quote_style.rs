#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(super) struct QuoteStyle {
    panic_id: crate::quote_panic_id::QuotePanicId,
    prefix: crate::quote_prefix::QuotePrefix,
    quote_ch: crate::quote_char::QuoteChar,
}

impl QuoteStyle {
    pub(super) const fn into_parts(
        self,
    ) -> (
        crate::quote_panic_id::QuotePanicId,
        crate::quote_prefix::QuotePrefix,
        crate::quote_char::QuoteChar,
    ) {
        (self.panic_id, self.prefix, self.quote_ch)
    }
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
