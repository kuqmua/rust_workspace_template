pub(super) fn quote_token_stream<Dsp>(
    style: super::QuoteStyle,
    value: &Dsp,
) -> super::ProcMacro2QuotedLiteralTokenStream
where
    Dsp: std::fmt::Display + ?Sized,
{
    super::ProcMacro2QuotedLiteralTokenStream::from(
        super::quote_literal(style.prefix, style.quote_ch, value)
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
