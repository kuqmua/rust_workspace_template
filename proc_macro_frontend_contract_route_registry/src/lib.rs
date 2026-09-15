#[proc_macro]
pub fn route_registry(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if matches!(
        token_stream.clone().into_iter().next(),
        Some(proc_macro::TokenTree::Punct(punctuation)) if punctuation.as_char() == '#'
    ) {
        proc_macro_frontend_contract_shared::route_registry(token_stream.into()).into()
    } else {
        proc_macro_frontend_contract_shared::endpoint_registry(token_stream.into()).into()
    }
}
