#[proc_macro_derive(PageCatalog, attributes(page_catalog, page_catalog_page))]
pub fn derive_page_catalog(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_frontend_contract_shared::derive_page_catalog(token_stream.into()).into()
}
