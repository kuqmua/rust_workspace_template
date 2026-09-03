#[proc_macro_derive(UnitEnumCatalog)]
pub fn derive_unit_enum_catalog(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::derive_unit_enum_catalog(token_stream.into()).into()
}
