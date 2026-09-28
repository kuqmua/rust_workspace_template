pub(super) fn quote_token_stream<Dsp>(
    quote_style: crate::quote_style::QuoteStyle,
    dsp: &Dsp,
) -> crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    let compile_error = |error| {
        let message = format!("{}: {error}", <&str>::from(quote_style.panic_id()));
        quote::quote! {compile_error!(#message);}
    };
    let tokens = match crate::quote_literal::quote_literal(
        quote_style.prefix(),
        quote_style.quote_ch(),
        dsp,
    ) {
        Ok(quoted_literal) => quoted_literal
            .as_ref()
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|error| compile_error(error.to_string())),
        Err(error) => compile_error(error.to_string()),
    };
    crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream::from(tokens)
}
