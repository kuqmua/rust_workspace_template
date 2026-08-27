pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        v: super::SwaggerUrlPathPrefix<'_>,
    ) -> super::SwaggerUrlPathSelfQuotesTokenStreamValue;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: super::SwaggerUrlPathSelfQuotesStr,
{
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        v: super::SwaggerUrlPathPrefix<'_>,
    ) -> super::SwaggerUrlPathSelfQuotesTokenStreamValue {
        match self
            .swagger_url_path_self_quotes_str(v)
            .as_ref()
            .parse::<proc_macro2::TokenStream>()
        {
            Ok(parsed_token_stream) => super::SwaggerUrlPathSelfQuotesTokenStreamValue::from(
                generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream::from(
                    parsed_token_stream,
                ),
            ),
            Err(error) => {
                let message = error.to_string();
                super::SwaggerUrlPathSelfQuotesTokenStreamValue::from(
                    generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream::from(
                        quote::quote! {compile_error!(#message);},
                    ),
                )
            }
        }
    }
}
