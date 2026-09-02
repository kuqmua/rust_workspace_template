pub(super) fn quote_token_stream<Dsp>(
    quote_style: crate::quote_style::QuoteStyle,
    dsp: &Dsp,
) -> crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    let (panic_id, prefix, quote_ch) = quote_style.into_parts();
    let quoted_literal = crate::quote_literal::quote_literal(prefix, quote_ch, dsp);
    crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream::from(
        quoted_literal
            .as_ref()
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|error| {
                let message = format!("{}: {error}", <&str>::from(panic_id));
                format!("compile_error!(\"{message}\");")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| proc_macro2::TokenStream::new())
            }),
    )
}
