use crate::domain_types::{QuoteChar, QuotePanicId, QuotePrefix, QuoteStyle};

pub(crate) const fn build_quote_style(
    panic_id: QuotePanicId,
    prefix: QuotePrefix,
    quote_ch: QuoteChar,
) -> QuoteStyle {
    QuoteStyle {
        panic_id,
        prefix,
        quote_ch,
    }
}
