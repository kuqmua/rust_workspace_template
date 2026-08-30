#[cfg(test)]
mod tests {
    fn assert_quote_str(actual: &crate::quoted_literal::QuotedLiteral, expected: &str) {
        assert_eq!(actual.as_ref(), expected);
    }
    fn assert_quote_token_stream(
        actual: &crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
        expected: &str,
    ) {
        assert_eq!(actual.to_string(), expected);
    }
    #[test]
    fn quote_str_helpers_return_expected_literals() {
        assert_quote_str(
            &crate::single_quotes_str::single_quotes_str(constants_str::catalog::ABC_ALT_3),
            constants_str::catalog::ABC,
        );
        assert_quote_str(
            &crate::double_quoted_string::double_quoted_string(&constants_str::catalog::ABC_ALT_3),
            constants_str::catalog::ABC_ALT,
        );
        assert_quote_str(
            &crate::binary_single_quotes_str::binary_single_quotes_str(
                constants_str::catalog::ABC_ALT_3,
            ),
            constants_str::catalog::B_ABC,
        );
        assert_quote_str(
            &crate::binary_double_quoted_str::binary_double_quoted_str(
                &constants_str::catalog::ABC_ALT_3,
            ),
            constants_str::catalog::B_ABC_ALT,
        );
    }
    #[test]
    fn quote_token_stream_helpers_return_expected_tokens() {
        assert_quote_token_stream(
            &crate::single_quotes_token_stream::single_quotes_token_stream(
                constants_str::catalog::A_ALT,
            ),
            constants_str::catalog::A,
        );
        assert_quote_token_stream(
            &crate::dq_token_stream::dq_token_stream(&constants_str::catalog::ABC_ALT_3),
            constants_str::catalog::ABC_ALT,
        );
        assert_quote_token_stream(
            &crate::binary_single_quotes_token_stream::binary_single_quotes_token_stream(
                constants_str::catalog::A_ALT,
            ),
            constants_str::catalog::B_A,
        );
        assert_quote_token_stream(
            &crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(
                &constants_str::catalog::ABC_ALT_3,
            ),
            constants_str::catalog::B_ABC_ALT,
        );
    }
    #[test]
    fn quote_helpers_support_non_string_display_inputs() {
        assert_quote_str(
            &crate::double_quoted_string::double_quoted_string(&42i32),
            constants_str::catalog::VALUE_42_ALT,
        );
        assert_quote_str(
            &crate::binary_double_quoted_str::binary_double_quoted_str(&42i32),
            constants_str::catalog::B_42,
        );
        assert_quote_token_stream(
            &crate::dq_token_stream::dq_token_stream(&42i32),
            constants_str::catalog::VALUE_42_ALT,
        );
        assert_quote_token_stream(
            &crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(&42i32),
            constants_str::catalog::B_42,
        );
    }
    #[test]
    fn quote_helpers_support_empty_input() {
        assert_quote_str(
            &crate::single_quotes_str::single_quotes_str(
                constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::catalog::TEXT_ALT_4,
        );
        assert_quote_str(
            &crate::double_quoted_string::double_quoted_string(
                &constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::catalog::TEXT_ALT_12,
        );
        assert_quote_str(
            &crate::binary_single_quotes_str::binary_single_quotes_str(
                constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::catalog::B_ALT,
        );
        assert_quote_str(
            &crate::binary_double_quoted_str::binary_double_quoted_str(
                &constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::catalog::B_ALT_3,
        );
        assert!(
            crate::single_quotes_token_stream::single_quotes_token_stream("")
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_token_stream(
            &crate::dq_token_stream::dq_token_stream(
                &constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::catalog::TEXT_ALT_12,
        );
        assert!(
            crate::binary_single_quotes_token_stream::binary_single_quotes_token_stream("")
                .to_string()
                .contains("compile_error !")
        );
        assert_quote_token_stream(
            &crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(
                &constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::catalog::B_ALT_3,
        );
    }
}
