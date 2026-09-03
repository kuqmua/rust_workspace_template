#[proc_macro]
pub fn endpoint_registry(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::endpoint_registry(token_stream.into()).into()
}
