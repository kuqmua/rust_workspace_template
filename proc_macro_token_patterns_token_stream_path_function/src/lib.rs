#[proc_macro]
pub fn token_stream_path_function(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_token_patterns_shared::token_stream_path_function(token_stream.into()).into()
}
