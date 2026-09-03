#[proc_macro_derive(UnitEnumIndex)]
pub fn derive_unit_enum_index(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::derive_unit_enum_index(token_stream.into()).into()
}
