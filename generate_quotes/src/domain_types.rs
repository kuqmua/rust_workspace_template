#[path = "binary_double_quote_style.rs"]
mod binary_double_quote_style;
#[path = "binary_double_quoted_str.rs"]
mod binary_double_quoted_str;
#[path = "binary_double_quoted_token_stream.rs"]
mod binary_double_quoted_token_stream;
#[path = "binary_single_quote_style.rs"]
mod binary_single_quote_style;
#[path = "binary_single_quotes_str.rs"]
mod binary_single_quotes_str;
#[path = "binary_single_quotes_token_stream.rs"]
mod binary_single_quotes_token_stream;
#[path = "build_quote_style.rs"]
mod build_quote_style;
#[path = "double_quote_style.rs"]
mod double_quote_style;
#[path = "double_quoted_string.rs"]
mod double_quoted_string;
#[path = "dq_token_stream.rs"]
mod dq_token_stream;
#[path = "proc_macro2_quoted_literal_token_stream.rs"]
mod proc_macro2_quoted_literal_token_stream;
#[path = "quote_char.rs"]
mod quote_char;
#[path = "quote_literal.rs"]
mod quote_literal;
#[path = "quote_panic_id.rs"]
mod quote_panic_id;
#[path = "quote_prefix.rs"]
mod quote_prefix;
#[path = "quote_str.rs"]
mod quote_str;
#[path = "quote_style.rs"]
mod quote_style;
#[path = "quote_token_stream.rs"]
mod quote_token_stream;
#[path = "quoted_literal.rs"]
mod quoted_literal;
#[path = "quoted_literal_max_len.rs"]
mod quoted_literal_max_len;
#[path = "single_quote_style.rs"]
mod single_quote_style;
#[path = "single_quotes_str.rs"]
mod single_quotes_str;
#[path = "single_quotes_token_stream.rs"]
mod single_quotes_token_stream;

use binary_double_quote_style::binary_double_quote_style;
pub use binary_double_quoted_str::binary_double_quoted_str;
pub use binary_double_quoted_token_stream::binary_double_quoted_token_stream;
use binary_single_quote_style::binary_single_quote_style;
pub use binary_single_quotes_str::binary_single_quotes_str;
pub use binary_single_quotes_token_stream::binary_single_quotes_token_stream;
use build_quote_style::build_quote_style;
use double_quote_style::double_quote_style;
pub use double_quoted_string::double_quoted_string;
pub use dq_token_stream::dq_token_stream;
pub use proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream;
use quote_char::QuoteChar;
use quote_literal::quote_literal;
use quote_panic_id::QuotePanicId;
use quote_prefix::QuotePrefix;
use quote_str::quote_str;
use quote_style::QuoteStyle;
use quote_token_stream::quote_token_stream;
pub use quoted_literal::QuotedLiteral;
use quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN;
use single_quote_style::single_quote_style;
pub use single_quotes_str::single_quotes_str;
pub use single_quotes_token_stream::single_quotes_token_stream;

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
            &super::double_quoted_string(&constants_str::ABC_ALT_3),
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
        assert_quote_str(
            &super::double_quoted_string(&42i32),
            constants_str::VALUE_42_ALT,
        );
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
    fn quote_helpers_support_empty_input() {
        assert_quote_str(
            &super::single_quotes_str(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::TEXT_ALT_4,
        );
        assert_quote_str(
            &super::double_quoted_string(&constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
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
