#[proc_macro]
pub fn token_pattern_parts(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_token_patterns_shared::token_pattern_parts(token_stream.into()).into()
}
