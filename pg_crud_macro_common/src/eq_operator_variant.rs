#[derive(Debug, Clone, Copy, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub enum EqOperatorVariant {
    Eq,
    IsNull,
}
impl EqOperatorVariant {
    #[must_use]
    pub fn to_tokens_path(
        &self,
        import: &crate::import::Import,
    ) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream
    {
        let names = crate::names_context::NamesContext::new();

        #[allow(non_snake_case, reason = "lint suppression is required here")]
        let (EqOperatorUpperCamelCase,) = (names.get_eq_operator_upper_camel_case(),);
        let ts = match &self {
            Self::Eq => quote::quote! {Eq},
            Self::IsNull => quote::quote! {IsNull},
        };
        quote::quote! {#import::eq_operator::#EqOperatorUpperCamelCase::#ts}.into()
    }
}
