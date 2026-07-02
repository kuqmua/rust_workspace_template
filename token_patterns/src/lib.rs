#[derive(Debug, Clone, Copy, optml::Optml)]
pub struct Bool;

impl quote::ToTokens for Bool {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::ToTokens::to_tokens(&quote::quote! { bool }, tokens);
    }
}

#[derive(Debug, Clone, Copy, optml::Optml)]
pub struct CrateDfltSomeOneEl;

impl quote::ToTokens for CrateDfltSomeOneEl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::ToTokens::to_tokens(&quote::quote! { crate::DfltSomeOneEl }, tokens);
    }
}

#[derive(Debug, Clone, Copy, optml::Optml)]
pub struct CrateDfltSomeOneElCall;

impl quote::ToTokens for CrateDfltSomeOneElCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::ToTokens::to_tokens(
            &quote::quote! { crate::DfltSomeOneEl::dflt_some_one_el() },
            tokens,
        );
    }
}

#[derive(Debug, Clone, Copy, optml::Optml)]
pub struct DeriveDebugCloneCopy;

impl quote::ToTokens for DeriveDebugCloneCopy {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::ToTokens::to_tokens(&quote::quote! { #[derive(Debug, Clone, Copy, Optml)] }, tokens);
    }
}

#[derive(Debug, Clone, Copy, optml::Optml)]
pub struct SqlxAcquire;

impl quote::ToTokens for SqlxAcquire {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        quote::ToTokens::to_tokens(&quote::quote! { sqlx::Acquire }, tokens);
    }
}

#[must_use]
pub fn path_dflt_some_one_el_call() -> proc_macro2::TokenStream {
    quote::quote! { ::dflt_some_one_el() }
}

#[must_use]
pub fn pg_crud() -> proc_macro2::TokenStream {
    quote::quote! { pg_crud:: }
}

#[must_use]
pub fn pg_crud_cmn() -> proc_macro2::TokenStream {
    quote::quote! { pg_crud_cmn:: }
}
