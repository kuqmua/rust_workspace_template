pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        v: crate::domain_types::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: crate::domain_types::SwaggerUrlPathSelfQuotesStr,
{
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        v: crate::domain_types::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream {
        match self
            .swagger_url_path_self_quotes_str(v)
            .as_ref()
            .parse::<proc_macro2::TokenStream>()
        {
            Ok(parsed_token_stream) => {
                generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream::from(
                    parsed_token_stream,
                )
            }
            Err(error) => {
                let message = error.to_string();
                generate_quotes::domain_types::ProcMacro2QuotedLiteralTokenStream::from(
                    quote::quote! {compile_error!(#message);},
                )
            }
        }
    }
}
