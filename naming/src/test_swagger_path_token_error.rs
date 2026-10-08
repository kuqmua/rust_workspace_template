#[test]
fn test_swagger_path_token_parser_failure_preserves_the_diagnostic() {
    assert!(
        [
            '"'.to_string(),
            format!("{}{}", constants_str::NON_ASCII_U_E9, '"'),
        ]
        .iter()
        .all(|prefix_text| {
            let prefix = crate::swagger_url_path_prefix::SwaggerUrlPathPrefix::from(prefix_text.as_str());
            let quoted_result = crate::swagger_url_path_self_quotes_str::SwaggerUrlPathSelfQuotesStr::swagger_url_path_self_quotes_str(
                &constants_str::X,
                prefix,
            );
            assert!(quoted_result.is_ok());
            let Ok(quoted) = quoted_result else {
                return false;
            };
            let parsed = quoted.as_ref().parse::<proc_macro2::TokenStream>();
            assert!(parsed.is_err());
            let Err(error) = parsed else {
                return false;
            };
            let generated = crate::swagger_url_path_self_quotes_token_stream::SwaggerUrlPathSelfQuotesTokenStream::swagger_url_path_self_quotes_token_stream(
                &constants_str::X,
                prefix,
            );
            syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(generated)).is_ok_and(|item| {
                item.mac.path.is_ident(stringify!(compile_error))
                    && item.semi_token.is_some()
                    && syn::parse2::<syn::LitStr>(item.mac.tokens)
                        .is_ok_and(|literal| literal.value() == error.to_string())
            })
        })
    );
}

#[test]
fn test_swagger_case_errors_preserve_every_variant_and_compile_error_diagnostic() {
    #[derive(Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    enum TestSwaggerCaseFailure {
        Invalid,
        Long,
        Nul,
        Short,
        WrongBounds,
    }
    impl naming_common::domain_types::AsRefStrToSnakeCaseStr for TestSwaggerCaseFailure {
        fn case(&self) -> String {
            constants_str::X.to_owned()
        }
        fn try_case(
            &self,
        ) -> Result<String, naming_common::case_string::CaseStringTryFromStringError> {
            Err(match self {
                Self::WrongBounds => {
                    naming_common::case_string::CaseStringTryFromStringError::InvalidBounds {
                        min: 2usize,
                        max: 1usize,
                    }
                }
                Self::Short => naming_common::case_string::CaseStringTryFromStringError::TooShort {
                    len: 0usize,
                    min: 1usize,
                },
                Self::Long => naming_common::case_string::CaseStringTryFromStringError::TooLong {
                    len: 3usize,
                    max: 2usize,
                },
                Self::Nul => naming_common::case_string::CaseStringTryFromStringError::ContainsNul,
                Self::Invalid => {
                    naming_common::case_string::CaseStringTryFromStringError::InvalidValue
                }
            })
        }
    }
    assert!([
        (TestSwaggerCaseFailure::WrongBounds, generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::InvalidBounds { min: 2usize, max: 1usize }),
        (TestSwaggerCaseFailure::Short, generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::TooShort { len: 0usize, min: 1usize }),
        (TestSwaggerCaseFailure::Long, generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::TooLong { len: 3usize, max: 2usize }),
        (TestSwaggerCaseFailure::Nul, generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::ContainsNul),
        (TestSwaggerCaseFailure::Invalid, generate_quotes::quoted_literal::QuotedLiteralTryFromStringError::InvalidValue),
    ].into_iter().all(|(test_swagger_case_failure, expected)| {
        let prefix = crate::swagger_url_path_prefix::SwaggerUrlPathPrefix::from(constants_str::SERVICE);
        let expected_message = expected.to_string();
        assert_eq!(crate::swagger_url_path_self_quotes_str::SwaggerUrlPathSelfQuotesStr::swagger_url_path_self_quotes_str(&test_swagger_case_failure, prefix), Err(expected));
        let generated = crate::swagger_url_path_self_quotes_token_stream::SwaggerUrlPathSelfQuotesTokenStream::swagger_url_path_self_quotes_token_stream(&test_swagger_case_failure, prefix);
        syn::parse2::<syn::ItemMacro>(proc_macro2::TokenStream::from(generated)).is_ok_and(|item| {
            item.mac.path.is_ident(stringify!(compile_error)) && item.semi_token.is_some()
                && syn::parse2::<syn::LitStr>(item.mac.tokens).is_ok_and(|literal| literal.value() == expected_message)
        })
    }));
}
