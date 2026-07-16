const QUOTED_LITERAL_MAX_LEN: usize = 1_048_576;
#[derive(Debug, Clone, Copy)]
struct QuotePrefix(&'static str);
#[derive(Debug, Clone, Copy)]
struct QuoteChar(char);
#[derive(Debug, Clone, Copy)]
struct QuotePanicId(&'static str);
#[derive(
    Debug, Clone, PartialEq, Eq, newtype::BoundedString, newtype::AsRefStr, newtype::Display,
)]
#[bounded_string(max = QUOTED_LITERAL_MAX_LEN )]
pub struct QuotedLiteral(String);
#[derive(
    Debug,
    Clone,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub struct ProcMacro2QuotedLiteralTokenStream(proc_macro2::TokenStream);
fn quote_literal<Dsp>(prefix: QuotePrefix, quote_ch: QuoteChar, v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    let mut out = String::with_capacity(prefix.0.len().saturating_add(2));
    out.push_str(prefix.0);
    out.push(quote_ch.0);
    if std::fmt::Write::write_fmt(&mut out, format_args!("{v}")).is_err() {
        return QuotedLiteral::try_from(format!("{}{}{v}{}", prefix.0, quote_ch.0, quote_ch.0))
            .unwrap_or_else(QuotedLiteral::from);
    }
    out.push(quote_ch.0);
    QuotedLiteral::try_from(out).unwrap_or_else(QuotedLiteral::from)
}
#[allow(clippy::single_call_fn)] // shared with prefix-aware token quote wrapper to keep parse+panic-id flow in one place
fn quote_literal_token_stream<Dsp>(
    prefix: QuotePrefix,
    quote_ch: QuoteChar,
    v: &Dsp,
    panic_id: QuotePanicId,
) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    ProcMacro2QuotedLiteralTokenStream::from(
        quote_literal(prefix, quote_ch, v)
            .0
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|error| {
                let message = format!("{}: {error}", panic_id.0);
                format!("compile_error!(\"{message}\");")
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
    quote_literal(
        QuotePrefix(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
        QuoteChar('\''),
        v,
    )
}
#[must_use]
pub fn single_quotes_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_token_stream(
        QuotePrefix(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
        QuoteChar('\''),
        v,
        QuotePanicId(str_constants::EC1E77D5),
    )
}
#[must_use]
pub fn dq_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(
        QuotePrefix(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
        QuoteChar('\"'),
        v,
    )
}
#[must_use]
pub fn dq_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_token_stream(
        QuotePrefix(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
        QuoteChar('\"'),
        v,
        QuotePanicId(str_constants::VALUE_0391AC99),
    )
}
#[must_use]
pub fn binary_single_quotes_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix(str_constants::B), QuoteChar('\''), v)
}
#[must_use]
pub fn binary_single_quotes_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_token_stream(
        QuotePrefix(str_constants::B),
        QuoteChar('\''),
        v,
        QuotePanicId(str_constants::VALUE_8BCE26E7),
    )
}
#[must_use]
pub fn binary_double_quoted_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix(str_constants::B), QuoteChar('\"'), v)
}
#[must_use]
pub fn binary_double_quoted_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal_token_stream(
        QuotePrefix(str_constants::B),
        QuoteChar('\"'),
        v,
        QuotePanicId(str_constants::VALUE_5DC6F142),
    )
}
#[cfg(test)]
mod tests {
    fn assert_quote_str(actual: &super::QuotedLiteral, expected: &str) {
        assert_eq!(actual.0, expected);
    }
    fn assert_quote_token_stream(
        actual: &super::ProcMacro2QuotedLiteralTokenStream,
        expected: &str,
    ) {
        assert_eq!(actual.to_string(), expected);
    }
    #[test]
    fn quote_str_helpers_return_expected_literals() {
        assert_quote_str(
            &super::single_quotes_str(str_constants::ABC_ALT_3),
            str_constants::ABC,
        );
        assert_quote_str(
            &super::dq_str(&str_constants::ABC_ALT_3),
            str_constants::ABC_ALT,
        );
        assert_quote_str(
            &super::binary_single_quotes_str(str_constants::ABC_ALT_3),
            str_constants::B_ABC,
        );
        assert_quote_str(
            &super::binary_double_quoted_str(&str_constants::ABC_ALT_3),
            str_constants::B_ABC_ALT,
        );
    }
    #[test]
    fn quote_token_stream_helpers_return_expected_tokens() {
        assert_quote_token_stream(
            &super::single_quotes_token_stream(str_constants::A_ALT),
            str_constants::A,
        );
        assert_quote_token_stream(
            &super::dq_token_stream(&str_constants::ABC_ALT_3),
            str_constants::ABC_ALT,
        );
        assert_quote_token_stream(
            &super::binary_single_quotes_token_stream(str_constants::A_ALT),
            str_constants::B_A,
        );
        assert_quote_token_stream(
            &super::binary_double_quoted_token_stream(&str_constants::ABC_ALT_3),
            str_constants::B_ABC_ALT,
        );
    }
    #[test]
    fn quote_helpers_support_non_string_display_inputs() {
        assert_quote_str(&super::dq_str(&42i32), str_constants::VALUE_42_ALT);
        assert_quote_str(
            &super::binary_double_quoted_str(&42i32),
            str_constants::B_42,
        );
        assert_quote_token_stream(&super::dq_token_stream(&42i32), str_constants::VALUE_42_ALT);
        assert_quote_token_stream(
            &super::binary_double_quoted_token_stream(&42i32),
            str_constants::B_42,
        );
    }
    #[test]
    fn quote_helpers_handle_empty_input() {
        assert_quote_str(
            &super::single_quotes_str(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            str_constants::TEXT_ALT_4,
        );
        assert_quote_str(
            &super::dq_str(&str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            str_constants::TEXT_ALT_12,
        );
        assert_quote_str(
            &super::binary_single_quotes_str(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            str_constants::B_ALT,
        );
        assert_quote_str(
            &super::binary_double_quoted_str(&str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            str_constants::B_ALT_3,
        );
        assert!(
            super::single_quotes_token_stream("")
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_token_stream(
            &super::dq_token_stream(&str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            str_constants::TEXT_ALT_12,
        );
        assert!(
            super::binary_single_quotes_token_stream("")
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_token_stream(
            &super::binary_double_quoted_token_stream(&str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            str_constants::B_ALT_3,
        );
    }
}
