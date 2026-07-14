#![allow(
    clippy::single_call_fn,
    reason = "the client emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
pub(super) const fn client_uses_text_value(
    spec: crate::model::FilterSpec,
) -> crate::model::FilterSpecValid {
    spec.has_text_value_shape()
}
pub(super) fn text_search_ts(
    spec: crate::model::FilterSpec,
) -> macros_helpers::generated_rust_ts::GeneratedRustTs {
    if !client_uses_text_value(spec).get() {
        return quote::quote! {compile_error!("text search client requires text value shape");}
            .into();
    }
    quote::quote! {
        impl PgTypeWhTextSearch {
            pub fn try_new(oprtr: pg_crud_cmn::Oprtr, mode: TextSearchMode, value: String) -> Result<Self, TextSearchValueEr> {
                let _validated_pattern = build_text_search_pattern(value.as_str(), mode)?;
                Ok(Self { value, mode, oprtr })
            }
            pub fn pattern(&self) -> Result<TextSearchPattern, TextSearchValueEr> {
                build_text_search_pattern(self.value.as_str(), self.mode)
            }
        }
    }
    .into()
}
