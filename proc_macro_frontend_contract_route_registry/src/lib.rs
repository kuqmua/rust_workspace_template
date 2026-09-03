#[proc_macro]
pub fn route_registry(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::route_registry(token_stream.into()).into()
}
