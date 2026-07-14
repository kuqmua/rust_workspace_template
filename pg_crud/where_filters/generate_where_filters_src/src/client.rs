#![allow(
    clippy::single_call_fn,
    reason = "the client emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
pub(super) const fn client_uses_text_value(
    spec: crate::model::FilterSpec,
) -> crate::model::FilterSpecValid {
    spec.has_text_value_shape()
}
pub(super) fn text_search_token_stream(
    spec: crate::model::FilterSpec,
) -> macros_helpers::generated_rust_token_stream::GeneratedRustTokenStream {
    if !client_uses_text_value(spec).get() {
        return quote::quote! {compile_error!("text search client requires text value shape");}
            .into();
    }
    quote::quote! {
        impl PgTypeWhereTextSearch {
            pub fn try_new(operator: pg_crud_common::Operator, mode: TextSearchMode, value: String) -> Result<Self, TextSearchValueError> {
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
