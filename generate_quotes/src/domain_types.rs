const QUOTED_LITERAL_MAX_LEN: usize = 1_048_576;
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct QuotePrefix(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct QuoteChar(char);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct QuotePanicId(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
struct QuoteStyle {
    panic_id: QuotePanicId,
    prefix: QuotePrefix,
    quote_ch: QuoteChar,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefStr,
    newtype::Display,
)]
#[bounded_string(max = QUOTED_LITERAL_MAX_LEN )]
pub struct QuotedLiteral(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::Display,
    newtype::FromInner,
    newtype::IntoInner,
    newtype::IntoInnerFrom,
    newtype::ToTokens,
)]
pub struct ProcMacro2QuotedLiteralTokenStream(proc_macro2::TokenStream);
fn binary_double_quote_style() -> QuoteStyle {
    quote_style(
        QuotePanicId::from(constants_str::VALUE_5DC6F142),
        QuotePrefix::from(constants_str::B),
        QuoteChar::from('"'),
    )
}
fn binary_single_quote_style() -> QuoteStyle {
    quote_style(
        QuotePanicId::from(constants_str::VALUE_8BCE26E7),
        QuotePrefix::from(constants_str::B),
        QuoteChar::from('\''),
    )
}
fn double_quote_style() -> QuoteStyle {
    quote_style(
        QuotePanicId::from(constants_str::VALUE_0391AC99),
        QuotePrefix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        QuoteChar::from('"'),
    )
}
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
const fn quote_style(
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
fn quote_token_stream<Dsp>(style: QuoteStyle, value: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    ProcMacro2QuotedLiteralTokenStream::from(
        quote_literal(style.prefix, style.quote_ch, value)
            .0
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|error| {
                let message = format!("{}: {error}", style.panic_id.0);
                format!("compile_error!(\"{message}\");")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| proc_macro2::TokenStream::new())
            }),
    )
}
fn single_quote_style() -> QuoteStyle {
    quote_style(
        QuotePanicId::from(constants_str::EC1E77D5),
        QuotePrefix::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        QuoteChar::from('\''),
    )
}
fn quote_str<Dsp>(style: QuoteStyle, value: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_literal(style.prefix, style.quote_ch, value)
}
#[must_use]
pub fn single_quotes_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_str(single_quote_style(), v)
}
#[must_use]
pub fn single_quotes_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_token_stream(single_quote_style(), v)
}
#[must_use]
pub fn dq_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_str(double_quote_style(), v)
}
#[must_use]
pub fn dq_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_token_stream(double_quote_style(), v)
}
#[must_use]
pub fn binary_single_quotes_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_str(binary_single_quote_style(), v)
}
#[must_use]
pub fn binary_single_quotes_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_token_stream(binary_single_quote_style(), v)
}
#[must_use]
pub fn binary_double_quoted_str<Dsp>(v: &Dsp) -> QuotedLiteral
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_str(binary_double_quote_style(), v)
}
#[must_use]
pub fn binary_double_quoted_token_stream<Dsp>(v: &Dsp) -> ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    quote_token_stream(binary_double_quote_style(), v)
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
            &super::single_quotes_str(constants_str::ABC_ALT_3),
            constants_str::ABC,
        );
        assert_quote_str(
            &super::dq_str(&constants_str::ABC_ALT_3),
            constants_str::ABC_ALT,
        );
        assert_quote_str(
            &super::binary_single_quotes_str(constants_str::ABC_ALT_3),
            constants_str::B_ABC,
        );
        assert_quote_str(
            &super::binary_double_quoted_str(&constants_str::ABC_ALT_3),
            constants_str::B_ABC_ALT,
        );
    }
    #[test]
    fn quote_token_stream_helpers_return_expected_tokens() {
        assert_quote_token_stream(
            &super::single_quotes_token_stream(constants_str::A_ALT),
            constants_str::A,
        );
        assert_quote_token_stream(
            &super::dq_token_stream(&constants_str::ABC_ALT_3),
            constants_str::ABC_ALT,
        );
        assert_quote_token_stream(
            &super::binary_single_quotes_token_stream(constants_str::A_ALT),
            constants_str::B_A,
        );
        assert_quote_token_stream(
            &super::binary_double_quoted_token_stream(&constants_str::ABC_ALT_3),
            constants_str::B_ABC_ALT,
        );
    }
    #[test]
    fn quote_helpers_support_non_string_display_inputs() {
        assert_quote_str(&super::dq_str(&42i32), constants_str::VALUE_42_ALT);
        assert_quote_str(
            &super::binary_double_quoted_str(&42i32),
            constants_str::B_42,
        );
        assert_quote_token_stream(&super::dq_token_stream(&42i32), constants_str::VALUE_42_ALT);
        assert_quote_token_stream(
            &super::binary_double_quoted_token_stream(&42i32),
            constants_str::B_42,
        );
    }
    #[test]
    fn quote_helpers_handle_empty_input() {
        assert_quote_str(
            &super::single_quotes_str(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::TEXT_ALT_4,
        );
        assert_quote_str(
            &super::dq_str(&constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::TEXT_ALT_12,
        );
        assert_quote_str(
            &super::binary_single_quotes_str(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::B_ALT,
        );
        assert_quote_str(
            &super::binary_double_quoted_str(&constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::B_ALT_3,
        );
        assert!(
            super::single_quotes_token_stream("")
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_token_stream(
            &super::dq_token_stream(&constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::TEXT_ALT_12,
        );
        assert!(
            super::binary_single_quotes_token_stream("")
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_token_stream(
            &super::binary_double_quoted_token_stream(&constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::B_ALT_3,
        );
    }
}
