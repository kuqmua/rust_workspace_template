#![allow(clippy::single_call_fn)] // client text-search emission has one source assembly owner

pub(crate) fn client_text_search_token_stream(
    spec: crate::spec::FilterSpec,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    if !crate::client_uses_text_value::client_uses_text_value(spec).get() {
        return quote::quote! {compile_error!("text search client requires text value shape");}
            .into();
    }
    quote::quote! {
        impl PgTypeWhereTextSearch {
            pub fn try_new(operator: pg_crud_common::domain_types::Operator, mode: TextSearchMode, value: String) -> Result<Self, TextSearchValueError> {
                let _validated_pattern = build_text_search_pattern(value.as_str(), mode)?;
                Ok(Self { value, mode, operator })
            }
            pub fn pattern(&self) -> Result<TextSearchPattern, TextSearchValueError> {
                build_text_search_pattern(self.value.as_str(), self.mode)
            }
        }
    }
    .into()
}
