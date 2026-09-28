pub trait SwaggerUrlPathSelfQuotesTokenStream {
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        swagger_url_path_prefix: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream;
}

impl<T> SwaggerUrlPathSelfQuotesTokenStream for T
where
    T: crate::swagger_url_path_self_quotes_str::SwaggerUrlPathSelfQuotesStr,
{
    fn swagger_url_path_self_quotes_token_stream(
        &self,
        swagger_url_path_prefix: crate::swagger_url_path_prefix::SwaggerUrlPathPrefix<'_>,
    ) -> generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
    {
        let compile_error = |message| {
            generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream::from(
                quote::quote! {compile_error!(#message);},
            )
        };
        match self.swagger_url_path_self_quotes_str(swagger_url_path_prefix) {
            Ok(quoted_literal) => match quoted_literal.as_ref().parse::<proc_macro2::TokenStream>() {
                Ok(tokens) => generate_quotes::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream::from(tokens),
                Err(error) => compile_error(error.to_string()),
            },
            Err(error) => compile_error(error.to_string()),
        }
    }
}
