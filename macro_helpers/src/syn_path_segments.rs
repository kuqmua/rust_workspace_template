#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_to_tokens::ToTokens,
)]
pub struct SynPathSegments(syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>);

impl SynPathSegments {
    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
