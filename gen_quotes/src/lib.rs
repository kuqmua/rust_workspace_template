const NO_PREFIX: &str = "";
const BINARY_PREFIX: &str = "b";
const SINGLE_QUOTE: char = '\'';
const DQ: char = '\"';
fn quote_literal<Dsp>(prefix: &str, quote_ch: char, v: &Dsp) -> String
where
    Dsp: std::fmt::Display + ?Sized,
{
    let mut out = String::with_capacity(prefix.len().saturating_add(2));
    out.push_str(prefix);
    out.push(quote_ch);
    if std::fmt::Write::write_fmt(&mut out, format_args!("{v}")).is_err() {
        return format!("{prefix}{quote_ch}{v}{quote_ch}");
    }
    out.push(quote_ch);
    out
}
#[allow(clippy::single_call_fn)] // shared with prefix-aware token quote wrapper to keep parse+panic-id flow in one place
fn quote_literal_ts<Dsp>(
    prefix: &str,
    quote_ch: char,
    v: &Dsp,
    panic_id: &str,
) -> proc_macro2::TokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(prefix, quote_ch, v)
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|er| {
            let msg = format!("{panic_id}: {er}");
            format!("compile_error!(\"{msg}\");")
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| proc_macro2::TokenStream::new())
        })
}
#[must_use]
pub fn single_quotes_str(v: &str) -> String {
    quote_literal(NO_PREFIX, SINGLE_QUOTE, v)
}
#[must_use]
pub fn single_quotes_ts(v: &str) -> proc_macro2::TokenStream {
    quote_literal_ts(NO_PREFIX, SINGLE_QUOTE, v, "ec1e77d5")
}
#[must_use]
pub fn dq_str<Dsp>(v: &Dsp) -> String
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(NO_PREFIX, DQ, v)
}
#[must_use]
pub fn dq_ts<Dsp>(v: &Dsp) -> proc_macro2::TokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_ts(NO_PREFIX, DQ, v, "0391ac99")
}
#[must_use]
pub fn binary_single_quotes_str(v: &str) -> String {
    quote_literal(BINARY_PREFIX, SINGLE_QUOTE, v)
}
#[must_use]
pub fn binary_single_quotes_ts(v: &str) -> proc_macro2::TokenStream {
    quote_literal_ts(BINARY_PREFIX, SINGLE_QUOTE, v, "8bce26e7")
}
#[must_use]
pub fn binary_dq_str<Dsp>(v: &Dsp) -> String
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(BINARY_PREFIX, DQ, v)
}
#[must_use]
pub fn binary_dq_ts<Dsp>(v: &Dsp) -> proc_macro2::TokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_ts(BINARY_PREFIX, DQ, v, "5dc6f142")
}
#[cfg(test)]
mod tests {
    fn assert_quote_str(actual: &str, expected: &str) {
        assert_eq!(actual, expected);
    }
    fn assert_quote_ts(actual: &proc_macro2::TokenStream, expected: &str) {
        assert_eq!(actual.to_string(), expected);
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
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_ts(&super::dq_ts(&""), "\"\"");
        assert!(
            super::binary_single_quotes_ts("")
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_ts(&super::binary_dq_ts(&""), "b\"\"");
    }
}
