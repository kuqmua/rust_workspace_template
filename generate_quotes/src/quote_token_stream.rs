pub(super) fn quote_token_stream<Dsp>(
    style: crate::quote_style::QuoteStyle,
    value: &Dsp,
) -> crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::proc_macro2_quoted_literal_token_stream::ProcMacro2QuotedLiteralTokenStream::from(
        crate::quote_literal::quote_literal(style.prefix, style.quote_ch, value)
            .0
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|error| {
                let message = format!("{}: {error}", style.panic_id.0);
                format!("compile_error!(\"{message}\");")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| proc_macro2::TokenStream::new())
            }),
    )
}
