pub(super) fn quote_token_stream<Dsp>(
    style: crate::domain_types::QuoteStyle,
    value: &Dsp,
) -> crate::domain_types::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    crate::domain_types::ProcMacro2QuotedLiteralTokenStream::from(
        crate::domain_types::quote_literal(style.prefix, style.quote_ch, value)
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
