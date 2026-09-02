pub(super) fn to_token_stream_or_panic<T>(
    t: &T,
) -> crate::proc_macro2_case_token_stream::ProcMacro2CaseTokenStream
where
    T: std::fmt::Display + ?Sized,
{
    crate::proc_macro2_case_token_stream::ProcMacro2CaseTokenStream::from(
        match t.to_string().parse::<proc_macro2::TokenStream>() {
            Ok(parsed_token_stream) => parsed_token_stream,
            Err(error) => {
                let message = error.to_string();
                quote::quote! {compile_error!(#message);}
            }
        },
    )
}
