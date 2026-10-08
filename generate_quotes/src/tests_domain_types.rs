#[cfg(test)]
mod tests {
    fn assert_quote_str(
        result: &Result<
            crate::quoted_literal::QuotedLiteral,
            crate::quoted_literal::QuotedLiteralTryFromStringError,
        >,
        str: &str,
    ) {
        assert!(matches!(result, Ok(quoted_literal) if quoted_literal.as_ref() == str));
    }
    fn assert_quote_token_stream(
        proc_macro2_quoted_literal_token_stream: &crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
        str: &str,
    ) {
        assert_eq!(proc_macro2_quoted_literal_token_stream.to_string(), str);
    }
    fn assert_quote_error_diagnostic(
        proc_macro2_quoted_literal_token_stream: &crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream,
        quote_panic_id: crate::quote_panic_id::QuotePanicId,
        str: &str,
    ) {
        let mut message = String::from(<&str>::from(quote_panic_id));
        message.push(':');
        message.push(' ');
        message.push_str(str);
        assert_eq!(
            proc_macro2_quoted_literal_token_stream.to_string(),
            quote::quote! {compile_error!(#message);}.to_string()
        );
    }
    #[test]
    fn test_quote_str_helpers_return_expected_literals() {
        assert_quote_str(
            &crate::single_quotes_str::single_quotes_str(constants_str::ABC_ALT_3),
            constants_str::ABC,
        );
        assert_quote_str(
            &crate::double_quoted_string::double_quoted_string(&constants_str::ABC_ALT_3),
            constants_str::ABC_ALT,
        );
        assert_quote_str(
            &crate::binary_single_quotes_str::binary_single_quotes_str(constants_str::ABC_ALT_3),
            constants_str::B_ABC,
        );
        assert_quote_str(
            &crate::binary_double_quoted_str::binary_double_quoted_str(&constants_str::ABC_ALT_3),
            constants_str::B_ABC_ALT,
        );
    }
    #[test]
    fn test_quote_token_stream_helpers_return_expected_tokens() {
        assert_quote_token_stream(
            &crate::single_quotes_token_stream::single_quotes_token_stream(constants_str::A_ALT),
            constants_str::A,
        );
        assert_quote_token_stream(
            &crate::double_quoted_token_stream::double_quoted_token_stream(
                &constants_str::ABC_ALT_3,
            ),
            constants_str::ABC_ALT,
        );
        assert_quote_token_stream(
            &crate::binary_single_quotes_token_stream::binary_single_quotes_token_stream(
                constants_str::A_ALT,
            ),
            constants_str::B_A,
        );
        assert_quote_token_stream(
            &crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(
                &constants_str::ABC_ALT_3,
            ),
            constants_str::B_ABC_ALT,
        );
    }
    #[test]
    fn test_quote_helpers_support_non_string_display_inputs() {
        assert_quote_str(
            &crate::double_quoted_string::double_quoted_string(&42i32),
            constants_str::VALUE_42_ALT,
        );
        assert_quote_str(
            &crate::binary_double_quoted_str::binary_double_quoted_str(&42i32),
            constants_str::B_42,
        );
        assert_quote_token_stream(
            &crate::double_quoted_token_stream::double_quoted_token_stream(&42i32),
            constants_str::VALUE_42_ALT,
        );
        assert_quote_token_stream(
            &crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(&42i32),
            constants_str::B_42,
        );
    }
    #[test]
    fn test_quote_helpers_support_empty_input() {
        assert_quote_str(
            &crate::single_quotes_str::single_quotes_str(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            constants_str::TEXT_ALT_4,
        );
        assert_quote_str(
            &crate::double_quoted_string::double_quoted_string(
                &constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::TEXT_ALT_12,
        );
        assert_quote_str(
            &crate::binary_single_quotes_str::binary_single_quotes_str(
                constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::B_ALT,
        );
        assert_quote_str(
            &crate::binary_double_quoted_str::binary_double_quoted_str(
                &constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::B_ALT_3,
        );
        assert!(
            crate::single_quotes_token_stream::single_quotes_token_stream(constants_str::EMPTY)
                .to_string()
                .contains(constants_str::VALUE_2EDAC0BF)
        );
        assert_quote_token_stream(
            &crate::double_quoted_token_stream::double_quoted_token_stream(
                &constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::TEXT_ALT_12,
        );
        assert!(
            crate::binary_single_quotes_token_stream::binary_single_quotes_token_stream(
                constants_str::EMPTY
            )
            .to_string()
            .contains(constants_str::VALUE_2EDAC0BF)
        );
        assert_quote_token_stream(
            &crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(
                &constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            ),
            constants_str::B_ALT_3,
        );
    }
    #[test]
    fn test_quoted_literal_length_boundaries_preserve_errors() {
        let maximum = crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN;
        assert!(
            [maximum - 3usize, maximum - 2usize, maximum]
                .into_iter()
                .all(|length| {
                    let input = constants_str::X.repeat(length);
                    [
                        (
                            crate::double_quoted_string::double_quoted_string(&input),
                            length + 2usize,
                        ),
                        (
                            crate::single_quotes_str::single_quotes_str(input.as_str()),
                            length + 2usize,
                        ),
                        (
                            crate::binary_double_quoted_str::binary_double_quoted_str(&input),
                            length + 3usize,
                        ),
                        (
                            crate::binary_single_quotes_str::binary_single_quotes_str(
                                input.as_str(),
                            ),
                            length + 3usize,
                        ),
                    ]
                    .into_iter()
                    .all(|(result, expected_length)| match result {
                        Ok(literal) => {
                            expected_length <= maximum && literal.as_ref().len() == expected_length
                        }
                        Err(crate::quoted_literal::QuotedLiteralTryFromStringError::TooLong {
                            len,
                            max,
                        }) => expected_length > maximum && len == expected_length && max == maximum,
                        Err(_) => false,
                    })
                })
        );
    }

    #[test]
    fn test_oversized_quote_tokens_emit_compile_error() {
        let input = constants_str::X.repeat(crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN);
        assert!(
            [
                (
                    crate::double_quoted_token_stream::double_quoted_token_stream(&input),
                    input.len() + 2usize,
                    constants_str::VALUE_0391AC99,
                ),
                (
                    crate::single_quotes_token_stream::single_quotes_token_stream(input.as_str()),
                    input.len() + 2usize,
                    constants_str::EC1E77D5,
                ),
                (
                    crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(
                        &input
                    ),
                    input.len() + 3usize,
                    constants_str::VALUE_5DC6F142,
                ),
                (
                    crate::binary_single_quotes_token_stream::binary_single_quotes_token_stream(
                        input.as_str()
                    ),
                    input.len() + 3usize,
                    constants_str::VALUE_8BCE26E7,
                ),
            ]
            .into_iter()
            .all(|(tokens, expected_length, panic_id)| {
                let error = crate::quoted_literal::QuotedLiteralTryFromStringError::TooLong {
                    len: expected_length,
                    max: crate::quoted_literal_max_len::QUOTED_LITERAL_MAX_LEN,
                };
                assert_quote_error_diagnostic(
                    &tokens,
                    crate::quote_panic_id::QuotePanicId::from(panic_id),
                    error.to_string().as_str(),
                );
                true
            })
        );
    }

    #[test]
    fn test_malformed_quote_tokens_preserve_parser_error_diagnostics() {
        let input = '\\'.to_string();
        assert!(
            [
                (
                    crate::double_quoted_string::double_quoted_string(&input),
                    crate::double_quoted_token_stream::double_quoted_token_stream(&input),
                    constants_str::VALUE_0391AC99,
                ),
                (
                    crate::single_quotes_str::single_quotes_str(input.as_str()),
                    crate::single_quotes_token_stream::single_quotes_token_stream(input.as_str()),
                    constants_str::EC1E77D5,
                ),
                (
                    crate::binary_double_quoted_str::binary_double_quoted_str(&input),
                    crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(
                        &input,
                    ),
                    constants_str::VALUE_5DC6F142,
                ),
                (
                    crate::binary_single_quotes_str::binary_single_quotes_str(input.as_str()),
                    crate::binary_single_quotes_token_stream::binary_single_quotes_token_stream(
                        input.as_str(),
                    ),
                    constants_str::VALUE_8BCE26E7,
                ),
            ]
            .into_iter()
            .all(|(result, tokens, panic_id)| {
                result.is_ok_and(|quoted_literal| {
                    quoted_literal
                        .as_ref()
                        .parse::<proc_macro2::TokenStream>()
                        .is_err_and(|error| {
                            assert_quote_error_diagnostic(
                                &tokens,
                                crate::quote_panic_id::QuotePanicId::from(panic_id),
                                error.to_string().as_str(),
                            );
                            true
                        })
                })
            })
        );
    }

    #[test]
    fn test_quote_token_helpers_preserve_valid_escape_sequences() {
        let input = ['\\', 'n'].into_iter().collect::<String>();
        assert!(
            [
                (
                    crate::double_quoted_token_stream::double_quoted_token_stream(&input),
                    proc_macro2::Literal::string(&'\n'.to_string()),
                ),
                (
                    crate::single_quotes_token_stream::single_quotes_token_stream(input.as_str()),
                    proc_macro2::Literal::character('\n'),
                ),
                (
                    crate::binary_double_quoted_token_stream::binary_double_quoted_token_stream(
                        &input,
                    ),
                    proc_macro2::Literal::byte_string(std::slice::from_ref(&b'\n')),
                ),
                (
                    crate::binary_single_quotes_token_stream::binary_single_quotes_token_stream(
                        input.as_str(),
                    ),
                    proc_macro2::Literal::byte_character(b'\n'),
                ),
            ]
            .into_iter()
            .all(|(tokens, literal)| tokens.to_string() == literal.to_string())
        );
    }
}
