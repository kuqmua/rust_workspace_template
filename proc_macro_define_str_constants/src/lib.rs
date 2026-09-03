#[proc_macro]
pub fn define_str_constants(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_constants_str_shared::define_str_constants(token_stream.into()).into()
}
