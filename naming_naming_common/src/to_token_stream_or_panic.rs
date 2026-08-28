pub(super) fn to_token_stream_or_panic<T>(v: &T) -> super::ProcMacro2CaseTokenStream
where
    T: std::fmt::Display + ?Sized,
{
    super::ProcMacro2CaseTokenStream::from(
        match v.to_string().parse::<proc_macro2::TokenStream>() {
            Ok(parsed_token_stream) => parsed_token_stream,
            Err(error) => {
                let message = error.to_string();
                quote::quote! {compile_error!(#message);}
            }
        },
    )
}
