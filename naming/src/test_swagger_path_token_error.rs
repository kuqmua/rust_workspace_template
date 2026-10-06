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
