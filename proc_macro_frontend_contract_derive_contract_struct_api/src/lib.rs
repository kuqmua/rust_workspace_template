#[proc_macro_derive(ContractStructApi, attributes(contract_struct_api))]
pub fn derive_contract_struct_api(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::derive_contract_struct_api(token_stream.into()).into()
}
