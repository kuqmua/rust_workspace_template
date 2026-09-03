#[proc_macro]
pub fn ts_path_fn(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_token_patterns_shared::ts_path_fn(token_stream.into()).into()
}
