const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';

#[derive(Debug, Clone, Copy)]
enum QuotePrefix {
    Binary,
    None,
}

#[derive(Debug, Clone, Copy)]
enum QuoteMark {
    Double,
    Single,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuotedLiteralText {
    value: String,
}

impl AsRef<str> for QuotedLiteralText {
    fn as_ref(&self) -> &str {
        self.value.as_ref()
    }
}

impl From<QuotedLiteralText> for String {
    fn from(value: QuotedLiteralText) -> Self {
        value.value
    }
}

fn quote_literal<DisplayValue>(
    quote_prefix: QuotePrefix,
    quote_mark: QuoteMark,
    value: &DisplayValue,
) -> QuotedLiteralText
where
    DisplayValue: AsRef<str> + ?Sized,
{
    let quote_character = match quote_mark {
        QuoteMark::Double => DOUBLE_QUOTE,
        QuoteMark::Single => SINGLE_QUOTE,
    };
    let string_value = value.as_ref();
    let mut output = String::with_capacity(string_value.len().saturating_add(3));
    if matches!(quote_prefix, QuotePrefix::Binary) {
        output.push('b');
    }
    output.push(quote_character);
    let write_result = core::fmt::Write::write_fmt(&mut output, format_args!("{string_value}"));
    if write_result.is_err() {
        output.push_str(string_value);
        output.push(quote_character);
        return QuotedLiteralText { value: output };
    }
    output.push(quote_character);
    QuotedLiteralText { value: output }
}

fn quote_literal_token_stream<DisplayValue>(
    quote_prefix: QuotePrefix,
    quote_mark: QuoteMark,
    value: &DisplayValue,
) -> proc_macro2::TokenStream
where
    DisplayValue: AsRef<str> + ?Sized,
{
    match quote_literal(quote_prefix, quote_mark, value)
        .as_ref()
        .parse::<proc_macro2::TokenStream>()
    {
        Ok(token_stream) => token_stream,
        Err(parse_error) => {
            let error_message = parse_error.to_string();
            let escaped_message = format!("{error_message:?}");
            let compile_error = format!("compile_error!({escaped_message})");
            Result::unwrap_or_else(compile_error.parse::<proc_macro2::TokenStream>(), |_| {
                proc_macro2::TokenStream::new()
            })
        }
    }
}

#[must_use]
pub fn single_quotes_str<DisplayValue>(value: &DisplayValue) -> QuotedLiteralText
where
    DisplayValue: AsRef<str> + ?Sized,
{
    quote_literal(QuotePrefix::None, QuoteMark::Single, value)
}

#[must_use]
pub fn single_quotes_ts<DisplayValue>(value: &DisplayValue) -> proc_macro2::TokenStream
where
    DisplayValue: AsRef<str> + ?Sized,
{
    quote_literal_token_stream(QuotePrefix::None, QuoteMark::Single, value)
}

#[must_use]
pub fn dq_str<DisplayValue>(value: &DisplayValue) -> QuotedLiteralText
where
    DisplayValue: core::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix::None, QuoteMark::Double, &value.to_string())
}

#[must_use]
pub fn dq_ts<DisplayValue>(value: &DisplayValue) -> proc_macro2::TokenStream
where
    DisplayValue: core::fmt::Display + ?Sized,
{
    quote_literal_token_stream(QuotePrefix::None, QuoteMark::Double, &value.to_string())
}

#[must_use]
pub fn binary_single_quotes_str<DisplayValue>(value: &DisplayValue) -> QuotedLiteralText
where
    DisplayValue: AsRef<str> + ?Sized,
{
    quote_literal(QuotePrefix::Binary, QuoteMark::Single, value)
}

#[must_use]
pub fn binary_single_quotes_ts<DisplayValue>(value: &DisplayValue) -> proc_macro2::TokenStream
where
    DisplayValue: AsRef<str> + ?Sized,
{
    quote_literal_token_stream(QuotePrefix::Binary, QuoteMark::Single, value)
}

#[must_use]
pub fn binary_dq_str<DisplayValue>(value: &DisplayValue) -> QuotedLiteralText
where
    DisplayValue: core::fmt::Display + ?Sized,
{
    quote_literal(QuotePrefix::Binary, QuoteMark::Double, &value.to_string())
}

#[must_use]
pub fn binary_dq_ts<DisplayValue>(value: &DisplayValue) -> proc_macro2::TokenStream
where
    DisplayValue: core::fmt::Display + ?Sized,
{
    quote_literal_token_stream(QuotePrefix::Binary, QuoteMark::Double, &value.to_string())
}

#[cfg(test)]
mod tests {
    fn assert_quote_str<Actual, Expected>(actual: Actual, expected: Expected) -> Result<(), String>
    where
        Actual: AsRef<str>,
        Expected: AsRef<str>,
    {
        if actual.as_ref() == expected.as_ref() {
            return Ok(());
        }
        Err(format!("{} != {}", actual.as_ref(), expected.as_ref()))
    }

    fn assert_quote_token_stream<Expected>(
        actual: &proc_macro2::TokenStream,
        expected: Expected,
    ) -> Result<(), String>
    where
        Expected: AsRef<str>,
    {
        assert_quote_str(actual.to_string(), expected)
    }

    #[test]
    fn quote_str_helpers_return_expected_literals() -> Result<(), String> {
        assert_quote_str(crate::single_quotes_str("abc"), "'abc'")?;
        assert_quote_str(crate::dq_str(&"abc"), "\"abc\"")?;
        assert_quote_str(crate::binary_single_quotes_str("abc"), "b'abc'")?;
        assert_quote_str(crate::binary_dq_str(&"abc"), "b\"abc\"")?;
        Ok(())
    }

    #[test]
    fn quote_token_stream_helpers_return_expected_tokens() -> Result<(), String> {
        assert_quote_token_stream(&crate::single_quotes_ts("a"), "'a'")?;
        assert_quote_token_stream(&crate::dq_ts(&"abc"), "\"abc\"")?;
        assert_quote_token_stream(&crate::binary_single_quotes_ts("a"), "b'a'")?;
        assert_quote_token_stream(&crate::binary_dq_ts(&"abc"), "b\"abc\"")?;
        Ok(())
    }

    #[test]
    fn quote_helpers_support_non_string_display_inputs() -> Result<(), String> {
        let value: i32 = 42;
        assert_quote_str(crate::dq_str(&value), "\"42\"")?;
        assert_quote_str(crate::binary_dq_str(&value), "b\"42\"")?;
        assert_quote_token_stream(&crate::dq_ts(&value), "\"42\"")?;
        assert_quote_token_stream(&crate::binary_dq_ts(&value), "b\"42\"")?;
        Ok(())
    }

    #[test]
    fn quote_helpers_handle_empty_input() -> Result<(), String> {
        assert_quote_str(crate::single_quotes_str(""), "''")?;
        assert_quote_str(crate::dq_str(&""), "\"\"")?;
        assert_quote_str(crate::binary_single_quotes_str(""), "b''")?;
        assert_quote_str(crate::binary_dq_str(&""), "b\"\"")?;
        assert_quote_token_stream(&crate::dq_ts(&""), "\"\"")?;
        assert_quote_token_stream(&crate::binary_dq_ts(&""), "b\"\"")?;
        assert_quote_str(
            crate::single_quotes_ts("").to_string(),
            "compile_error ! (\"cannot parse string into token stream\")",
        )?;
        assert_quote_str(
            crate::binary_single_quotes_ts("").to_string(),
            "compile_error ! (\"cannot parse string into token stream\")",
        )?;
        Ok(())
    }
}
