#![allow(
    clippy::single_call_fn,
    reason = "the client emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
pub(super) fn client_uses_text_value(
    spec: crate::domain_types::spec::FilterSpec,
) -> crate::domain_types::spec::FilterSpecValid {
    spec.has_text_value_shape()
}
pub(super) fn text_search_token_stream(
    spec: crate::domain_types::spec::FilterSpec,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    if !client_uses_text_value(spec).get() {
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
