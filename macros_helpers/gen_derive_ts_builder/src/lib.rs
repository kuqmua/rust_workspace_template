#[proc_macro]
pub fn gen_derive_ts_builder(
    _input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let empty_token_stream = proc_macro2::TokenStream::new();
    quote::quote! { #empty_token_stream }.into()
}
