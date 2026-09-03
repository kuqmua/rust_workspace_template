#[proc_macro]
pub fn api_operation_error(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::api_operation_error(token_stream.into()).into()
}
