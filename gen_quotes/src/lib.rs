#[derive(Debug, Clone, Copy)]
struct QuotePrefix(&'static str);
#[derive(Debug, Clone, Copy)]
struct QuoteChar(char);
#[derive(Debug, Clone, Copy)]
struct QuotePanicId(&'static str);
#[derive(Debug, Clone, PartialEq, Eq, newtype::Newtype)]
#[newtype(display, as_ref_str, deref)]
pub struct QuotedLiteral(pub String);
#[derive(Debug, Clone)]
pub struct QuotedLiteralTs(pub proc_macro2::TokenStream);
impl quote::ToTokens for QuotedLiteralTs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.clone());
    }
}
impl std::fmt::Display for QuotedLiteralTs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
fn quote_literal<Dsp>(prefix: QuotePrefix, quote_ch: QuoteChar, v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let mut out = String::with_capacity(prefix.0.len().saturating_add(2));
    out.push_str(prefix.0);
    out.push(quote_ch.0);
    if std::fmt::Write::write_fmt(&mut out, format_args!("{v}")).is_err() {
        return QuotedLiteral(format!("{}{}{v}{}", prefix.0, quote_ch.0, quote_ch.0));
    }
    out.push(quote_ch.0);
    QuotedLiteral(out)
}
#[allow(clippy::single_call_fn)] // shared with prefix-aware token quote wrapper to keep parse+panic-id flow in one place
fn quote_literal_ts<Dsp>(
    prefix: QuotePrefix,
    quote_ch: QuoteChar,
    v: &Dsp,
    panic_id: QuotePanicId,
) -> QuotedLiteralTs
where
    Dsp: std::fmt::Display + ?Sized,
{
    QuotedLiteralTs(
        quote_literal(prefix, quote_ch, v)
            .0
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|er| {
                let msg = format!("{}: {er}", panic_id.0);
                format!("compile_error!(\"{msg}\");")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| proc_macro2::TokenStream::new())
            }),
    )
}
#[must_use]
pub fn single_quotes_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix(""), QuoteChar('\''), v)
}
#[must_use]
pub fn single_quotes_ts<Dsp>(v: &Dsp) -> QuotedLiteralTs
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_ts(
        QuotePrefix(""),
        QuoteChar('\''),
        v,
        QuotePanicId("ec1e77d5"),
    )
}
#[must_use]
pub fn dq_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix(""), QuoteChar('\"'), v)
}
#[must_use]
pub fn dq_ts<Dsp>(v: &Dsp) -> QuotedLiteralTs
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_ts(
        QuotePrefix(""),
        QuoteChar('\"'),
        v,
        QuotePanicId("0391ac99"),
    )
}
#[must_use]
pub fn binary_single_quotes_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix("b"), QuoteChar('\''), v)
}
#[must_use]
pub fn binary_single_quotes_ts<Dsp>(v: &Dsp) -> QuotedLiteralTs
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_ts(
        QuotePrefix("b"),
        QuoteChar('\''),
        v,
        QuotePanicId("8bce26e7"),
    )
}
#[must_use]
pub fn binary_dq_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix("b"), QuoteChar('\"'), v)
}
#[must_use]
pub fn binary_dq_ts<Dsp>(v: &Dsp) -> QuotedLiteralTs
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_ts(
        QuotePrefix("b"),
        QuoteChar('\"'),
        v,
        QuotePanicId("5dc6f142"),
    )
}
#[cfg(test)]
mod tests {
    fn assert_quote_str(actual: &super::QuotedLiteral, expected: &str) {
        assert_eq!(actual.0, expected);
    }
    fn assert_quote_ts(actual: &super::QuotedLiteralTs, expected: &str) {
        assert_eq!(actual.0.to_string(), expected);
    }
    #[test]
    fn quote_str_helpers_return_expected_literals() {
        assert_quote_str(&super::single_quotes_str("abc"), "'abc'");
        assert_quote_str(&super::dq_str(&"abc"), "\"abc\"");
        assert_quote_str(&super::binary_single_quotes_str("abc"), "b'abc'");
        assert_quote_str(&super::binary_dq_str(&"abc"), "b\"abc\"");
    }
    #[test]
    fn quote_ts_helpers_return_expected_tokens() {
        assert_quote_ts(&super::single_quotes_ts("a"), "'a'");
        assert_quote_ts(&super::dq_ts(&"abc"), "\"abc\"");
        assert_quote_ts(&super::binary_single_quotes_ts("a"), "b'a'");
        assert_quote_ts(&super::binary_dq_ts(&"abc"), "b\"abc\"");
    }
    #[test]
    fn quote_helpers_support_non_string_display_inputs() {
        assert_quote_str(&super::dq_str(&42i32), "\"42\"");
        assert_quote_str(&super::binary_dq_str(&42i32), "b\"42\"");
        assert_quote_ts(&super::dq_ts(&42i32), "\"42\"");
        assert_quote_ts(&super::binary_dq_ts(&42i32), "b\"42\"");
    }
    #[test]
    fn quote_helpers_handle_empty_input() {
        assert_quote_str(&super::single_quotes_str(""), "''");
        assert_quote_str(&super::dq_str(&""), "\"\"");
        assert_quote_str(&super::binary_single_quotes_str(""), "b''");
        assert_quote_str(&super::binary_dq_str(&""), "b\"\"");
        assert!(
            super::single_quotes_ts("")
                .0
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_ts(&super::dq_ts(&""), "\"\"");
        assert!(
            super::binary_single_quotes_ts("")
                .0
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_ts(&super::binary_dq_ts(&""), "b\"\"");
    }
}
