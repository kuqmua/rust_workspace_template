#[proc_macro]
pub fn tp_batch(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_token_patterns_shared::tp_batch(token_stream.into()).into()
}
