pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        v: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: crate::swagger_url_path_self_quotes_str::SwaggerUrlPathSelfQuotesStr,
{
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        v: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
    {
        match self
            .swagger_url_path_self_quotes_str(v)
            .as_ref()
            .parse::<proc_macro2::TokenStream>()
        {
            Ok(parsed_token_stream) => {
                generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream::from(
                    parsed_token_stream,
                )
            }
            Err(error) => {
                let message = error.to_string();
                generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream::from(
                    quote::quote! {compile_error!(#message);},
                )
            }
        }
    }
}
