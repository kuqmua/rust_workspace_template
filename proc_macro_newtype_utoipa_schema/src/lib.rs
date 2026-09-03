#[proc_macro_derive(UtoipaSchema, attributes(utoipa_schema))]
pub fn utoipa_schema(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::utoipa_schema(token_stream.into()).into()
}
