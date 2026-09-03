#[proc_macro]
pub fn tp_parts(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_token_patterns_shared::tp_parts(token_stream.into()).into()
}
